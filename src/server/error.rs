//! Error

/// Server error
#[derive(Debug, thiserror::Error)]
#[error("Server error")]
pub enum Error {
    Io(String),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Io(value.to_string())
    }
}
