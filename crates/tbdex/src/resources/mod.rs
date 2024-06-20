pub mod resource_metadata;
pub mod offering;

use serde::{Deserialize, Serialize};
use web5::apid::dids::bearer_did::BearerDid;

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum ResourceKind {
    Offering,
    Balance,
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
