use serde_json::Error as SerdeJsonError;
use std::fmt::Debug;
use std::sync::PoisonError;
use tbdex::errors::TbdexError as InnerTbdexError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TbdexError {
    #[error("{msg}")]
    Error { variant: String, msg: String },
}

impl TbdexError {
    pub fn from_poison_error<T>(error: PoisonError<T>) -> Self {
        TbdexError::Error {
            variant: "PoisonError".to_string(),
            msg: error.to_string(),
        }
    }
    fn new<T>(error: T) -> Self
    where
        T: std::error::Error + 'static,
    {
        Self::Error {
            variant: variant_name(&error),
            msg: error.to_string(),
        }
    }
}

fn variant_name<T>(error: &T) -> String
where
    T: Debug,
{
    let message = format!("{:?}", error);
    let variant_name = message.split('(').next().unwrap_or("UnknownVariant");
    variant_name.to_string()
}

impl From<InnerTbdexError> for TbdexError {
    fn from(error: InnerTbdexError) -> Self {
        TbdexError::new(error)
    }
}

impl From<SerdeJsonError> for TbdexError {
    fn from(error: SerdeJsonError) -> Self {
        TbdexError::new(error)
    }
}

pub type Result<T> = std::result::Result<T, TbdexError>;
