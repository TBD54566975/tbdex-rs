use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeJsonError;
use type_safe_id::Error as TypeIdError;
use web5::errors::Web5Error;

use crate::http::ErrorResponseBody;

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum TbdexError {
    #[error("json error {0}")]
    Json(String),
    #[error("json schema error {0}")]
    JsonSchema(String),
    #[error("jose {0}")]
    Jose(String),
    #[error("typeid error {0}")]
    TypeId(String),
    #[error("parse error {0}")]
    Parse(String),
    #[error("offering verification error {0}")]
    OfferingVerification(String),
    #[error("private data verification error {0}")]
    PrivateDataVerification(String),
    #[error("http error {0}")]
    Http(String),
    #[error("http client error {0}")]
    HttpClient(String),

    #[error(transparent)]
    Web5Error(#[from] Web5Error),
    #[error(transparent)]
    ErrorResponseBody(#[from] ErrorResponseBody),
}

impl From<SerdeJsonError> for TbdexError {
    fn from(err: SerdeJsonError) -> Self {
        TbdexError::Json(err.to_string())
    }
}

impl From<TypeIdError> for TbdexError {
    fn from(err: TypeIdError) -> Self {
        TbdexError::TypeId(err.to_string())
    }
}

impl From<ReqwestError> for TbdexError {
    fn from(err: ReqwestError) -> Self {
        TbdexError::Http(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, TbdexError>;
