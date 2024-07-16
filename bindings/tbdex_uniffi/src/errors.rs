use serde_json::Error as SerdeJsonError;
use std::sync::PoisonError;
use std::{any::type_name, fmt::Debug};
use tbdex::http_client::HttpClientError;
use tbdex::json::JsonError;
use tbdex::messages::MessageError;
use tbdex::resources::ResourceError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RustCoreError {
    #[error("{msg}")]
    Error {
        r#type: String,
        variant: String,
        msg: String,
    },
}

impl RustCoreError {
    pub fn from_poison_error<T>(error: PoisonError<T>, error_type: &str) -> Self {
        RustCoreError::Error {
            r#type: error_type.to_string(),
            variant: "PoisonError".to_string(),
            msg: error.to_string(),
        }
    }
    fn new<T>(error: T) -> Self
    where
        T: std::error::Error + 'static,
    {
        Self::Error {
            r#type: type_of(&error).to_string(),
            variant: variant_name(&error),
            msg: error.to_string(),
        }
    }
}

fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

fn variant_name<T>(error: &T) -> String
where
    T: Debug,
{
    let message = format!("{:?}", error);
    let variant_name = message.split('(').next().unwrap_or("UnknownVariant");
    variant_name.to_string()
}

impl From<ResourceError> for RustCoreError {
    fn from(error: ResourceError) -> Self {
        RustCoreError::new(error)
    }
}

impl From<MessageError> for RustCoreError {
    fn from(error: MessageError) -> Self {
        RustCoreError::new(error)
    }
}

impl From<HttpClientError> for RustCoreError {
    fn from(error: HttpClientError) -> Self {
        RustCoreError::new(error)
    }
}

impl From<JsonError> for RustCoreError {
    fn from(error: JsonError) -> Self {
        RustCoreError::new(error)
    }
}

impl From<SerdeJsonError> for RustCoreError {
    fn from(error: SerdeJsonError) -> Self {
        RustCoreError::new(error)
    }
}

pub type Result<T> = std::result::Result<T, RustCoreError>;
