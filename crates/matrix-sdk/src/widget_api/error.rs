use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;
use crate::Error as MatrixSdkError;

#[derive(Debug, Error, Clone)]
pub enum Error {
    #[error("Unexpected widget disconnect")]
    WidgetDied,
    #[error("Widget error: {0}")]
    WidgetError(String),
}

impl From<MatrixSdkError> for Error {
    fn from(e: MatrixSdkError) -> Self {
        Error::WidgetError(e.to_string())
    }
}