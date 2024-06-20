pub mod close;
pub mod order;
pub mod order_status;
pub mod quote;
pub mod rfq;

use crate::errors::Result;
use std::sync::Arc;
use tbdex::messages::Message as InnerMessage;
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub trait Message: Send + Sync {
    fn sign(&self, bearer_did: Arc<BearerDid>) -> Result<()>;
    fn verify(&self) -> Result<()>;
}

pub struct OuterMessage(pub Arc<dyn InnerMessage>);

impl Message for OuterMessage {
    fn sign(&self, bearer_did: Arc<BearerDid>) -> Result<()> {
        self.0
            .sign(bearer_did.0.clone())
            .map_err(|e| Arc::new(e.into()))
    }

    fn verify(&self) -> Result<()> {
        self.0.verify().map_err(|e| Arc::new(e.into()))
    }
}
