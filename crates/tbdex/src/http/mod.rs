pub mod balances;
pub mod exchanges;
pub mod offerings;

use crate::messages::MessageError;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Error as SerdeJsonError;

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum HttpError {
    #[error("serde json error {0}")]
    SerdeJson(String),
    #[error(transparent)]
    Message(#[from] MessageError),
}

impl From<SerdeJsonError> for HttpError {
    fn from(err: SerdeJsonError) -> Self {
        HttpError::SerdeJson(err.to_string())
    }
}

type Result<T> = std::result::Result<T, HttpError>;

// TODO consider utilizing this throughout the entire codebase (move to a global space)
pub trait JsonDeserializer: Sized + DeserializeOwned {
    fn from_json_string(json: &str) -> Result<Self> {
        serde_json::from_str(json).map_err(HttpError::from)
    }
}

pub trait JsonSerializer: Serialize {
    fn to_json_string(&self) -> Result<String> {
        serde_json::to_string(self).map_err(HttpError::from)
    }
}
