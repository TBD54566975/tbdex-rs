pub mod close;
pub mod order;
pub mod order_status;
pub mod quote;
pub mod rfq;

use crate::errors::{Result, RustCoreError};
use std::sync::{Arc, RwLock};
use tbdex::messages::Message as InnerMessage;
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub trait Message: Send + Sync {
    fn sign(&self, bearer_did: Arc<BearerDid>) -> Result<()>;
    fn verify(&self) -> Result<()>;
}

pub struct OuterMessage(pub Arc<RwLock<Box<dyn InnerMessage>>>);

impl Message for OuterMessage {
    fn sign(&self, bearer_did: Arc<BearerDid>) -> Result<()> {
        let mut inner_message = self
            .0
            .write()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockWriteError"))?;

        inner_message
            .sign(bearer_did.0.clone())
            .map_err(|e| Arc::new(e.into()))
    }

    fn verify(&self) -> Result<()> {
        let inner_message = self
            .0
            .write()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockWriteError"))?;

        inner_message.verify().map_err(|e| Arc::new(e.into()))
    }
}
