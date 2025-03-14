/// Native SQLite connection using SqlCipher
use crate::storage::encrypted_store::DbConnectionPrivate;
use crate::storage::StorageError;
use diesel::sqlite::SqliteConnection;
use diesel::{
    connection::{AnsiTransactionManager, SimpleConnection},
    r2d2::{self, CustomizeConnection, PoolTransactionManager, PooledConnection},
    Connection,
};
use parking_lot::{Mutex, RwLock};
use std::sync::Arc;

pub type ConnectionManager = r2d2::ConnectionManager<SqliteConnection>;
pub type Pool = r2d2::Pool<ConnectionManager>;
pub type RawDbConnection = PooledConnection<ConnectionManager>;

use super::{sqlcipher_connection::EncryptedConnection, EncryptionKey, StorageOption, XmtpDb};

trait XmtpConnection:
    ValidatedConnection
    + CustomizeConnection<SqliteConnection, r2d2::Error>
    + dyn_clone::DynClone
    + IntoSuper<dyn CustomizeConnection<SqliteConnection, r2d2::Error>>
{
}

impl<T> XmtpConnection for T where
    T: ValidatedConnection
        + CustomizeConnection<SqliteConnection, r2d2::Error>
        + dyn_clone::DynClone
        + IntoSuper<dyn CustomizeConnection<SqliteConnection, r2d2::Error>>
{
}
dyn_clone::clone_trait_object!(XmtpConnection);

pub(crate) trait ValidatedConnection {
    fn validate(&self, _opts: &StorageOption) -> Result<(), StorageError> {
        Ok(())
    }
}

// we can remove this once https://github.com/rust-lang/rust/issues/65991
// is merged, which should be happening soon (next ~2 releases)
trait IntoSuper<Super: ?Sized> {
    fn into_super(self: Box<Self>) -> Box<Super>;
}

impl<T: CustomizeConnection<SqliteConnection, r2d2::Error>>
    IntoSuper<dyn CustomizeConnection<SqliteConnection, r2d2::Error>> for T
{
    fn into_super(self: Box<Self>) -> Box<dyn CustomizeConnection<SqliteConnection, r2d2::Error>> {
        self
    }
}

/// An Unencrypted Connection
/// Creates a Sqlite3 Database/Connection in WAL mode.
/// Sets `busy_timeout` on each connection.
/// _*NOTE:*_Unencrypted Connections are not validated and mostly meant for testing.
/// It is not recommended to use an unencrypted connection in production.
#[derive(Clone, Debug)]
pub struct UnencryptedConnection;
impl ValidatedConnection for UnencryptedConnection {}

impl CustomizeConnection<SqliteConnection, r2d2::Error> for UnencryptedConnection {
    fn on_acquire(&self, conn: &mut SqliteConnection) -> Result<(), r2d2::Error> {
        conn.batch_execute("PRAGMA query_only = ON; PRAGMA busy_timeout = 5000;")
            .map_err(r2d2::Error::QueryError)?;
        Ok(())
    }
}

impl StorageOption {
    // create a completely new standalone connection
    pub(super) fn conn(&self) -> Result<SqliteConnection, diesel::ConnectionError> {
        use StorageOption::*;
        match self {
            Persistent(path) => SqliteConnection::establish(path),
            Ephemeral => SqliteConnection::establish(":memory:"),
        }
    }

    pub(super) fn path(&self) -> Option<&String> {
        use StorageOption::*;
        match self {
            Persistent(path) => Some(path),
            _ => None,
        }
    }
}

#[derive(Clone)]
/// Database used in `native` (everywhere but web)
pub struct NativeDb {
    pub(super) pool: Arc<RwLock<Option<Pool>>>,
    pub(super) write_conn: Arc<Mutex<RawDbConnection>>,
    transaction_lock: Arc<Mutex<()>>,
    customizer: Option<Box<dyn XmtpConnection>>,
    opts: StorageOption,
}

impl NativeDb {
    /// This function is private so that an unencrypted database cannot be created by accident
    pub(super) fn new(
        opts: &StorageOption,
        enc_key: Option<EncryptionKey>,
    ) -> Result<Self, StorageError> {
        let mut builder = Pool::builder();

        let customizer = if let Some(key) = enc_key {
            let enc_connection = EncryptedConnection::new(key, opts)?;
            builder = builder.connection_customizer(Box::new(enc_connection.clone()));
            Some(Box::new(enc_connection) as Box<dyn XmtpConnection>)
        } else if matches!(opts, StorageOption::Persistent(_)) {
            builder = builder.connection_customizer(Box::new(UnencryptedConnection));
            Some(Box::new(UnencryptedConnection) as Box<dyn XmtpConnection>)
        } else {
            None
        };

        let pool = match opts {
            StorageOption::Ephemeral => builder
                .max_size(1)
                .build(ConnectionManager::new(":memory:"))?,
            StorageOption::Persistent(ref path) => builder
                .max_size(crate::configuration::MAX_DB_POOL_SIZE)
                .build(ConnectionManager::new(path))?,
        };

        // Take one of the connections and use it as the only writer.
        let mut write_conn = pool.get()?;
        write_conn.batch_execute("PRAGMA query_only = OFF;")?;

        Ok(Self {
            pool: Arc::new(Some(pool).into()),
            write_conn: Arc::new(Mutex::new(write_conn)),
            transaction_lock: Arc::new(Mutex::new(())),
            customizer,
            opts: opts.clone(),
        })
    }

    fn raw_conn(&self) -> Result<RawDbConnection, StorageError> {
        let pool_guard = self.pool.read();

        let pool = pool_guard
            .as_ref()
            .ok_or(StorageError::PoolNeedsConnection)?;

        tracing::trace!(
            "pulling connection from pool, idle={}, total={}",
            pool.state().idle_connections,
            pool.state().connections
        );

        Ok(pool.get()?)
    }
}

impl XmtpDb for NativeDb {
    type Connection = RawDbConnection;
    type TransactionManager = PoolTransactionManager<AnsiTransactionManager>;

    /// Returns the Wrapped [`super::db_connection::DbConnection`] Connection implementation for this Database
    fn conn(&self) -> Result<DbConnectionPrivate<Self::Connection>, StorageError> {
        let conn = match self.opts {
            StorageOption::Ephemeral => None,
            StorageOption::Persistent(_) => Some(self.raw_conn()?),
        };

        Ok(DbConnectionPrivate::from_arc_mutex(
            self.write_conn.clone(),
            conn.map(|conn| Arc::new(parking_lot::Mutex::new(conn))),
            self.transaction_lock.clone(),
        ))
    }

    fn validate(&self, opts: &StorageOption) -> Result<(), StorageError> {
        if let Some(c) = &self.customizer {
            c.validate(opts)
        } else {
            Ok(())
        }
    }

    fn release_connection(&self) -> Result<(), StorageError> {
        tracing::warn!("released sqlite database connection");
        let mut pool_guard = self.pool.write();
        pool_guard.take();
        Ok(())
    }

    fn reconnect(&self) -> Result<(), StorageError> {
        tracing::info!("reconnecting sqlite database connection");
        let mut builder = Pool::builder();

        if let Some(ref opts) = self.customizer {
            builder = builder.connection_customizer(opts.clone().into_super());
        }

        let pool = match self.opts {
            StorageOption::Ephemeral => builder
                .max_size(1)
                .build(ConnectionManager::new(":memory:"))?,
            StorageOption::Persistent(ref path) => builder
                .max_size(crate::configuration::MAX_DB_POOL_SIZE)
                .build(ConnectionManager::new(path))?,
        };

        let mut pool_write = self.pool.write();
        *pool_write = Some(pool);

        Ok(())
    }
}
