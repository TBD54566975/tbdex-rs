pub mod order;
pub mod rfq;
pub mod quote;

use crate::errors::Result;
use std::sync::Arc;
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub trait Message: Send + Sync {
    fn sign(&self, bearer_did: Arc<BearerDid>) -> Result<()>;
    fn verify(&self) -> Result<()>;
}
