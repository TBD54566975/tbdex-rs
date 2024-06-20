use super::Message;
use crate::errors::Result;
use std::sync::Arc;
use tbdex::messages::{order::Order as InnerOrder, Message as InnerMessage};
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub struct Order(pub InnerOrder);

impl Order {
    pub fn new(
        to: String,
        from: String,
        exchange_id: String,
        protocol: String,
        external_id: Option<String>,
    ) -> Self {
        Self(InnerOrder::new(
            to,
            from,
            exchange_id,
            protocol,
            external_id,
        ))
    }

    pub fn get_data(&self) -> InnerOrder {
        self.0.clone()
    }
}

impl Message for Order {
    fn sign(&self, bearer_did: Arc<BearerDid>) -> Result<()> {
        self.0
            .sign(bearer_did.0.clone())
            .map_err(|e| Arc::new(e.into()))
    }

    fn verify(&self) -> Result<()> {
        self.0.verify().map_err(|e| Arc::new(e.into()))
    }
}
