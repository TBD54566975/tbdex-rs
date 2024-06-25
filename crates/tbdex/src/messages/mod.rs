pub mod close;
pub mod order;
pub mod order_status;
pub mod quote;
pub mod rfq;

use crate::signature::SignatureError;
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeJsonError;
use std::str::FromStr;
use type_safe_id::{DynamicType, Error as TypeIdError, TypeSafeId};
use web5::apid::dids::bearer_did::BearerDidError;

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum MessageError {
    #[error("serde json error {0}")]
    SerdeJson(String),
    #[error("typeid error {0}")]
    TypeId(String),
    #[error(transparent)]
    BearerDid(#[from] BearerDidError),
    #[error(transparent)]
    Signature(#[from] SignatureError),
    #[error("unknown kind {0}")]
    UnknownKind(String),
    #[error("offering verification failure {0}")]
    OfferingVerification(String),
}

impl From<SerdeJsonError> for MessageError {
    fn from(err: SerdeJsonError) -> Self {
        MessageError::SerdeJson(err.to_string())
    }
}

impl From<TypeIdError> for MessageError {
    fn from(err: TypeIdError) -> Self {
        MessageError::TypeId(err.to_string())
    }
}

type Result<T> = std::result::Result<T, MessageError>;

#[derive(Debug, Default, Deserialize, PartialEq, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum MessageKind {
    #[default]
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

impl FromStr for MessageKind {
    type Err = MessageError;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "rfq" => Ok(MessageKind::Rfq),
            "quote" => Ok(MessageKind::Quote),
            "order" => Ok(MessageKind::Order),
            "orderstatus" => Ok(MessageKind::OrderStatus),
            "close" => Ok(MessageKind::Close),
            _ => Err(MessageError::UnknownKind(s.to_string())),
        }
    }
}

#[derive(Debug, Deserialize, Default, PartialEq, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MessageMetadata {
    pub from: String,
    pub to: String,
    pub kind: MessageKind,
    pub id: String,
    pub exchange_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    pub protocol: String,
    pub created_at: String,
}
