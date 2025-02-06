// @generated
// This file is @generated by prost-build.
/// Proto representation of a consent record save
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsentSave {
    #[prost(enumeration="ConsentTypeSave", tag="1")]
    pub entity_type: i32,
    #[prost(enumeration="ConsentStateSave", tag="2")]
    pub state: i32,
    #[prost(string, tag="3")]
    pub entity: ::prost::alloc::string::String,
}
/// Consent record type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ConsentTypeSave {
    Unspecified = 0,
    ConversationId = 1,
    InboxId = 2,
    Address = 3,
}
impl ConsentTypeSave {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ConsentTypeSave::Unspecified => "CONSENT_TYPE_SAVE_UNSPECIFIED",
            ConsentTypeSave::ConversationId => "CONSENT_TYPE_SAVE_CONVERSATION_ID",
            ConsentTypeSave::InboxId => "CONSENT_TYPE_SAVE_INBOX_ID",
            ConsentTypeSave::Address => "CONSENT_TYPE_SAVE_ADDRESS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CONSENT_TYPE_SAVE_UNSPECIFIED" => Some(Self::Unspecified),
            "CONSENT_TYPE_SAVE_CONVERSATION_ID" => Some(Self::ConversationId),
            "CONSENT_TYPE_SAVE_INBOX_ID" => Some(Self::InboxId),
            "CONSENT_TYPE_SAVE_ADDRESS" => Some(Self::Address),
            _ => None,
        }
    }
}
/// Consent record state
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ConsentStateSave {
    Unspecified = 0,
    Unknown = 1,
    Allowed = 2,
    Denied = 3,
}
impl ConsentStateSave {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ConsentStateSave::Unspecified => "CONSENT_STATE_SAVE_UNSPECIFIED",
            ConsentStateSave::Unknown => "CONSENT_STATE_SAVE_UNKNOWN",
            ConsentStateSave::Allowed => "CONSENT_STATE_SAVE_ALLOWED",
            ConsentStateSave::Denied => "CONSENT_STATE_SAVE_DENIED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CONSENT_STATE_SAVE_UNSPECIFIED" => Some(Self::Unspecified),
            "CONSENT_STATE_SAVE_UNKNOWN" => Some(Self::Unknown),
            "CONSENT_STATE_SAVE_ALLOWED" => Some(Self::Allowed),
            "CONSENT_STATE_SAVE_DENIED" => Some(Self::Denied),
            _ => None,
        }
    }
}
/// Encoded file descriptor set for the `xmtp.device_sync.consent_backup` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xe5, 0x0b, 0x0a, 0x20, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x73, 0x79, 0x6e, 0x63,
    0x2f, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x74, 0x5f, 0x62, 0x61, 0x63, 0x6b, 0x75, 0x70, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x1f, 0x78, 0x6d, 0x74, 0x70, 0x2e, 0x64, 0x65, 0x76, 0x69,
    0x63, 0x65, 0x5f, 0x73, 0x79, 0x6e, 0x63, 0x2e, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x74, 0x5f,
    0x62, 0x61, 0x63, 0x6b, 0x75, 0x70, 0x22, 0xc1, 0x01, 0x0a, 0x0b, 0x43, 0x6f, 0x6e, 0x73, 0x65,
    0x6e, 0x74, 0x53, 0x61, 0x76, 0x65, 0x12, 0x51, 0x0a, 0x0b, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79,
    0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x30, 0x2e, 0x78, 0x6d,
    0x74, 0x70, 0x2e, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x73, 0x79, 0x6e, 0x63, 0x2e, 0x63,
    0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x74, 0x5f, 0x62, 0x61, 0x63, 0x6b, 0x75, 0x70, 0x2e, 0x43, 0x6f,
    0x6e, 0x73, 0x65, 0x6e, 0x74, 0x54, 0x79, 0x70, 0x65, 0x53, 0x61, 0x76, 0x65, 0x52, 0x0a, 0x65,
    0x6e, 0x74, 0x69, 0x74, 0x79, 0x54, 0x79, 0x70, 0x65, 0x12, 0x47, 0x0a, 0x05, 0x73, 0x74, 0x61,
    0x74, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x31, 0x2e, 0x78, 0x6d, 0x74, 0x70, 0x2e,
    0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x73, 0x79, 0x6e, 0x63, 0x2e, 0x63, 0x6f, 0x6e, 0x73,
    0x65, 0x6e, 0x74, 0x5f, 0x62, 0x61, 0x63, 0x6b, 0x75, 0x70, 0x2e, 0x43, 0x6f, 0x6e, 0x73, 0x65,
    0x6e, 0x74, 0x53, 0x74, 0x61, 0x74, 0x65, 0x53, 0x61, 0x76, 0x65, 0x52, 0x05, 0x73, 0x74, 0x61,
    0x74, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x06, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x2a, 0x9a, 0x01, 0x0a, 0x0f, 0x43,
    0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x74, 0x54, 0x79, 0x70, 0x65, 0x53, 0x61, 0x76, 0x65, 0x12, 0x21,
    0x0a, 0x1d, 0x43, 0x4f, 0x4e, 0x53, 0x45, 0x4e, 0x54, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x53,
    0x41, 0x56, 0x45, 0x5f, 0x55, 0x4e, 0x53, 0x50, 0x45, 0x43, 0x49, 0x46, 0x49, 0x45, 0x44, 0x10,
    0x00, 0x12, 0x25, 0x0a, 0x21, 0x43, 0x4f, 0x4e, 0x53, 0x45, 0x4e, 0x54, 0x5f, 0x54, 0x59, 0x50,
    0x45, 0x5f, 0x53, 0x41, 0x56, 0x45, 0x5f, 0x43, 0x4f, 0x4e, 0x56, 0x45, 0x52, 0x53, 0x41, 0x54,
    0x49, 0x4f, 0x4e, 0x5f, 0x49, 0x44, 0x10, 0x01, 0x12, 0x1e, 0x0a, 0x1a, 0x43, 0x4f, 0x4e, 0x53,
    0x45, 0x4e, 0x54, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x53, 0x41, 0x56, 0x45, 0x5f, 0x49, 0x4e,
    0x42, 0x4f, 0x58, 0x5f, 0x49, 0x44, 0x10, 0x02, 0x12, 0x1d, 0x0a, 0x19, 0x43, 0x4f, 0x4e, 0x53,
    0x45, 0x4e, 0x54, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x53, 0x41, 0x56, 0x45, 0x5f, 0x41, 0x44,
    0x44, 0x52, 0x45, 0x53, 0x53, 0x10, 0x03, 0x2a, 0x95, 0x01, 0x0a, 0x10, 0x43, 0x6f, 0x6e, 0x73,
    0x65, 0x6e, 0x74, 0x53, 0x74, 0x61, 0x74, 0x65, 0x53, 0x61, 0x76, 0x65, 0x12, 0x22, 0x0a, 0x1e,
    0x43, 0x4f, 0x4e, 0x53, 0x45, 0x4e, 0x54, 0x5f, 0x53, 0x54, 0x41, 0x54, 0x45, 0x5f, 0x53, 0x41,
    0x56, 0x45, 0x5f, 0x55, 0x4e, 0x53, 0x50, 0x45, 0x43, 0x49, 0x46, 0x49, 0x45, 0x44, 0x10, 0x00,
    0x12, 0x1e, 0x0a, 0x1a, 0x43, 0x4f, 0x4e, 0x53, 0x45, 0x4e, 0x54, 0x5f, 0x53, 0x54, 0x41, 0x54,
    0x45, 0x5f, 0x53, 0x41, 0x56, 0x45, 0x5f, 0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x10, 0x01,
    0x12, 0x1e, 0x0a, 0x1a, 0x43, 0x4f, 0x4e, 0x53, 0x45, 0x4e, 0x54, 0x5f, 0x53, 0x54, 0x41, 0x54,
    0x45, 0x5f, 0x53, 0x41, 0x56, 0x45, 0x5f, 0x41, 0x4c, 0x4c, 0x4f, 0x57, 0x45, 0x44, 0x10, 0x02,
    0x12, 0x1d, 0x0a, 0x19, 0x43, 0x4f, 0x4e, 0x53, 0x45, 0x4e, 0x54, 0x5f, 0x53, 0x54, 0x41, 0x54,
    0x45, 0x5f, 0x53, 0x41, 0x56, 0x45, 0x5f, 0x44, 0x45, 0x4e, 0x49, 0x45, 0x44, 0x10, 0x03, 0x42,
    0xcf, 0x01, 0x0a, 0x23, 0x63, 0x6f, 0x6d, 0x2e, 0x78, 0x6d, 0x74, 0x70, 0x2e, 0x64, 0x65, 0x76,
    0x69, 0x63, 0x65, 0x5f, 0x73, 0x79, 0x6e, 0x63, 0x2e, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x74,
    0x5f, 0x62, 0x61, 0x63, 0x6b, 0x75, 0x70, 0x42, 0x12, 0x43, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x74,
    0x42, 0x61, 0x63, 0x6b, 0x75, 0x70, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0xa2, 0x02, 0x03,
    0x58, 0x44, 0x43, 0xaa, 0x02, 0x1d, 0x58, 0x6d, 0x74, 0x70, 0x2e, 0x44, 0x65, 0x76, 0x69, 0x63,
    0x65, 0x53, 0x79, 0x6e, 0x63, 0x2e, 0x43, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x74, 0x42, 0x61, 0x63,
    0x6b, 0x75, 0x70, 0xca, 0x02, 0x1d, 0x58, 0x6d, 0x74, 0x70, 0x5c, 0x44, 0x65, 0x76, 0x69, 0x63,
    0x65, 0x53, 0x79, 0x6e, 0x63, 0x5c, 0x43, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x74, 0x42, 0x61, 0x63,
    0x6b, 0x75, 0x70, 0xe2, 0x02, 0x29, 0x58, 0x6d, 0x74, 0x70, 0x5c, 0x44, 0x65, 0x76, 0x69, 0x63,
    0x65, 0x53, 0x79, 0x6e, 0x63, 0x5c, 0x43, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x74, 0x42, 0x61, 0x63,
    0x6b, 0x75, 0x70, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea,
    0x02, 0x1f, 0x58, 0x6d, 0x74, 0x70, 0x3a, 0x3a, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x53, 0x79,
    0x6e, 0x63, 0x3a, 0x3a, 0x43, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x74, 0x42, 0x61, 0x63, 0x6b, 0x75,
    0x70, 0x4a, 0xcc, 0x05, 0x0a, 0x06, 0x12, 0x04, 0x01, 0x00, 0x19, 0x01, 0x0a, 0x23, 0x0a, 0x01,
    0x0c, 0x12, 0x03, 0x01, 0x00, 0x12, 0x1a, 0x19, 0x20, 0x44, 0x65, 0x66, 0x69, 0x6e, 0x69, 0x74,
    0x69, 0x6f, 0x6e, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x62, 0x61, 0x63, 0x6b, 0x75, 0x70, 0x73,
    0x0a, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x28, 0x0a, 0x3b, 0x0a, 0x02, 0x04,
    0x00, 0x12, 0x04, 0x05, 0x00, 0x09, 0x01, 0x1a, 0x2f, 0x20, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x20,
    0x72, 0x65, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f,
    0x66, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x72, 0x65, 0x63, 0x6f,
    0x72, 0x64, 0x20, 0x73, 0x61, 0x76, 0x65, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12,
    0x03, 0x05, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x06, 0x02,
    0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x06, 0x02, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x06, 0x12, 0x1d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x06, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x07, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x06, 0x12, 0x03, 0x07, 0x02, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x07, 0x13, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x07,
    0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x08, 0x02, 0x14, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x08, 0x02, 0x08, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x08, 0x09, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x08, 0x12, 0x13, 0x0a, 0x21, 0x0a, 0x02, 0x05, 0x00, 0x12,
    0x04, 0x0c, 0x00, 0x11, 0x01, 0x1a, 0x15, 0x20, 0x43, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x74, 0x20,
    0x72, 0x65, 0x63, 0x6f, 0x72, 0x64, 0x20, 0x74, 0x79, 0x70, 0x65, 0x0a, 0x0a, 0x0a, 0x0a, 0x03,
    0x05, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x05, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x0d, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x0d, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0d, 0x22,
    0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x02, 0x28, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0e, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x0e, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x02, 0x12, 0x03, 0x0f, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x0f, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03,
    0x0f, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x03, 0x12, 0x03, 0x10, 0x02, 0x20,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x10, 0x02, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x10, 0x1e, 0x1f, 0x0a, 0x22, 0x0a, 0x02,
    0x05, 0x01, 0x12, 0x04, 0x14, 0x00, 0x19, 0x01, 0x1a, 0x16, 0x20, 0x43, 0x6f, 0x6e, 0x73, 0x65,
    0x6e, 0x74, 0x20, 0x72, 0x65, 0x63, 0x6f, 0x72, 0x64, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x0a,
    0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x01, 0x01, 0x12, 0x03, 0x14, 0x05, 0x15, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x01, 0x02, 0x00, 0x12, 0x03, 0x15, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x15, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x02,
    0x12, 0x03, 0x15, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x01, 0x12, 0x03, 0x16,
    0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x16, 0x02, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x02, 0x12, 0x03, 0x16, 0x1f, 0x20, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x01, 0x02, 0x02, 0x12, 0x03, 0x17, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x17, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02,
    0x02, 0x02, 0x12, 0x03, 0x17, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x03, 0x12,
    0x03, 0x18, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x18,
    0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x03, 0x02, 0x12, 0x03, 0x18, 0x1e, 0x1f,
    0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("xmtp.device_sync.consent_backup.serde.rs");
// @@protoc_insertion_point(module)