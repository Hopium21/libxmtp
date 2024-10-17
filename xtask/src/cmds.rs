//! the library in use: https://docs.rs/xflags
//! re-generate the flags with `UPDATE_XFLAGS=1 cargo build`
//! this keeps the compile time of `cargo xtask` command fast.

pub mod flags {
    use std::path::PathBuf;

    xflags::xflags! {
        src "./src/cmds.rs"
        /// `libxmtp` build system
        /// Pass additional arguments to cargo with `--`
        /// For example, `cargo xtask test -p wasm -- --release` to test with `release` profile
        cmd libxmtp {
            /// Build Command
            cmd build {
                /// build `bindings_wasm` with options
                cmd BindingsWasm {
                    optional -o,--out-dir path: PathBuf
                }
            }
        }
    }

    // generated start
    // The following code is generated by `xflags` macro.
    // Run `env UPDATE_XFLAGS=1 cargo build` to regenerate.
    #[derive(Debug)]
    pub struct Libxmtp {
        pub subcommand: LibxmtpCmd,
    }

    #[derive(Debug)]
    pub enum LibxmtpCmd {
        Build(Build),
    }

    #[derive(Debug)]
    pub struct Build {
        pub subcommand: BuildCmd,
    }

    #[derive(Debug)]
    pub enum BuildCmd {
        BindingsWasm(BindingsWasm),
    }

    #[derive(Debug)]
    pub struct BindingsWasm {
        pub out_dir: Option<PathBuf>,
    }

    impl Libxmtp {
        #[allow(dead_code)]
        pub fn from_env_or_exit() -> Self {
            Self::from_env_or_exit_()
        }

        #[allow(dead_code)]
        pub fn from_env() -> xflags::Result<Self> {
            Self::from_env_()
        }

        #[allow(dead_code)]
        pub fn from_vec(args: Vec<std::ffi::OsString>) -> xflags::Result<Self> {
            Self::from_vec_(args)
        }
    }
    // generated end
}
