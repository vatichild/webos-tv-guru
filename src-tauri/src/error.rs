use serde::{Serialize, Serializer};

/// Every error a command can return. The frontend sees it as a plain message.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Session(#[from] ares_connection_lib::session::SessionError),
    #[error("{0}")]
    Luna(#[from] ares_connection_lib::luna::LunaError),
    #[error("{0}")]
    Transfer(#[from] ares_connection_lib::transfer::TransferError),
    #[error("{0}")]
    Ssh(#[from] libssh_rs::Error),
    #[error("Invalid JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Invalid package: {0}")]
    Ipk(String),
    #[error("Install failed: {reason} (error {code})")]
    Install { code: i32, reason: String },
    #[error("Uploaded package is corrupted: expected sha256 {expected}, device has {actual}")]
    Checksum { expected: String, actual: String },
    #[error("{0}")]
    Other(String),
}

impl AppError {
    pub fn other(message: impl Into<String>) -> Self {
        Self::Other(message.into())
    }
}

impl Serialize for AppError {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;
