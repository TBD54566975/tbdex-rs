pub mod quote;
pub mod rfq;

use serde::{Deserialize, Serialize};
use web5::apid::dids::bearer_did::BearerDid;

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum MessageKind {
    Rfq,
    Quote,
    Order,
    OrderStatus,
    Close,
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

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum MessageError {
    #[error("unknown -- temporarily stubbed in")]
    UnknownError,
}

type Result<T> = std::result::Result<T, MessageError>;

pub trait Message: Send + Sync {
    fn sign(&self, bearer_did: BearerDid) -> Result<()>;
    fn verify(&self) -> Result<()>;
}
