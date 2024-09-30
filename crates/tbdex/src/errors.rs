use crate::http::ErrorResponseBody;
use http_std::Error as HttpStdError;
use serde_json::Error as SerdeJsonError;
use type_safe_id::Error as TypeIdError;
use web5::errors::Web5Error;

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum TbdexError {
    #[error("async runtime error {0}")]
    AsyncRuntime(String),
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
    HttpStdError(#[from] HttpStdError),
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

pub type Result<T> = std::result::Result<T, TbdexError>;
