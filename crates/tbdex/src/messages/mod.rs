pub mod close;
pub mod order;
pub mod order_status;
pub mod quote;
pub mod rfq;

use crate::signature::SignatureError;
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeJsonError;
use type_safe_id::{DynamicType, Error as TypeIdError, TypeSafeId};
use web5::apid::dids::bearer_did::{BearerDid, BearerDidError};

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum MessageError {
    #[error("serde json error {0}")]
    SerdeJsonError(String),
    #[error("typeid error {0}")]
    TypeIdError(String),
    #[error(transparent)]
    BearerDidError(#[from] BearerDidError),
    #[error(transparent)]
    SignatureError(#[from] SignatureError),
}

impl From<SerdeJsonError> for MessageError {
    fn from(err: SerdeJsonError) -> Self {
        MessageError::SerdeJsonError(err.to_string())
    }
}

impl From<TypeIdError> for MessageError {
    fn from(err: TypeIdError) -> Self {
        MessageError::TypeIdError(err.to_string())
    }
}

type Result<T> = std::result::Result<T, MessageError>;

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum MessageKind {
    Rfq,
    Quote,
    Order,
    OrderStatus,
    Close,
}

impl MessageKind {
    pub fn typesafe_id(&self) -> Result<String> {
        let serialized_kind = serde_json::to_string(&self)?;
        let dynamic_type = DynamicType::new(serialized_kind.trim_matches('"'))?;
        Ok(TypeSafeId::new_with_type(dynamic_type).to_string())
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MessageMetadata {
    pub from: String,
    pub to: String,
    pub kind: MessageKind,
    pub id: String,
    pub exchange_id: String,
    pub external_id: Option<String>,
    pub protocol: String,
    pub created_at: String,
}

pub trait Message: Send + Sync {
    fn sign(&mut self, bearer_did: BearerDid) -> Result<()>;
    fn verify(&self) -> Result<()>;

    fn clone_box(&self) -> Box<dyn Message>;
}
