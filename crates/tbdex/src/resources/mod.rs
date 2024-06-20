pub mod balance;
pub mod offering;

use serde::{Deserialize, Serialize};
use web5::apid::dids::bearer_did::BearerDid;

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum ResourceKind {
    Offering,
    Balance,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMetadata {
    pub kind: ResourceKind,
    pub from: String,
    pub to: String,
    pub id: String,
    pub protocol: String,
    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum ResourceError {
    #[error("unknown -- temporarily stubbed in")]
    UnknownError,
}

type Result<T> = std::result::Result<T, ResourceError>;

pub trait Resource: Send + Sync {
    fn sign(&self, bearer_did: BearerDid) -> Result<()>;
    fn verify(&self) -> Result<()>;
}
