use std::fmt::Display;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error, Clone)]
pub enum Error {
    #[error("Unexpected widget disconnect")]
    WidgetDied,
    #[error("Widget error: {0}")]
    WidgetError(String),
    #[error("Capabilities has already been negotiated")]
    AlreadyLoaded,
    #[error("Invalid JSON")]
    InvalidJSON(String),
    #[error("Unexpected response")]
    UnexpectedResponse,
    #[error("Handler did not send a reply")]
    NoReply,
    #[error("Invalid permissions")]
    InvalidPermissions,
    #[error("Failed to perform an operation")]
    Other,
}
impl Error {
    pub fn to_description_string(&self) -> String {
        match self {
            Error::InvalidJSON(e) => format!("{}: {}",self.to_string(), e),
            Error::WidgetError(e) => format!("{}: {}",self.to_string(), e),
            _ => self.to_string(),
        }
    }
}
