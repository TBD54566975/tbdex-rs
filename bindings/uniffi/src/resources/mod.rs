pub mod offering;

use crate::errors::Result;
use std::sync::Arc;
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub trait Resource: Send + Sync {
    fn sign(&self, bearer_did: Arc<BearerDid>) -> Result<()>;
    fn verify(&self) -> Result<()>;
}
