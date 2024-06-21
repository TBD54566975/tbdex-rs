use serde_json::Error as SerdeJsonError;
use std::sync::{Arc, PoisonError};
use std::{any::type_name, fmt::Debug};
use tbdex::http_client::TbdexHttpClientError;
use tbdex::messages::MessageError;
use tbdex::resources::ResourceError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RustCoreError {
    #[error("{message}")]
    Error {
        r#type: String,
        variant: String,
        message: String,
    },
}

impl RustCoreError {
    pub fn from_poison_error<T>(error: PoisonError<T>, error_type: &str) -> Arc<Self> {
        Arc::new(RustCoreError::Error {
            r#type: error_type.to_string(),
            variant: "PoisonError".to_string(),
            message: error.to_string(),
        })
    }
    fn new<T>(error: T) -> Self
    where
        T: std::error::Error + 'static,
    {
        Self::Error {
            r#type: type_of(&error).to_string(),
            variant: variant_name(&error),
            message: error.to_string(),
        }
    }

    pub fn error_type(&self) -> String {
        match self {
            RustCoreError::Error {
                r#type: error_type, ..
            } => error_type.clone(),
        }
    }

    pub fn variant(&self) -> String {
        match self {
            RustCoreError::Error {
                variant: error_variant,
                ..
            } => error_variant.clone(),
        }
    }

    pub fn message(&self) -> String {
        match self {
            RustCoreError::Error { message, .. } => message.clone(),
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

impl From<TbdexHttpClientError> for RustCoreError {
    fn from(error: TbdexHttpClientError) -> Self {
        RustCoreError::new(error)
    }
}

impl From<SerdeJsonError> for RustCoreError {
    fn from(error: SerdeJsonError) -> Self {
        RustCoreError::new(error)
    }
}

pub type Result<T> = std::result::Result<T, Arc<RustCoreError>>;
