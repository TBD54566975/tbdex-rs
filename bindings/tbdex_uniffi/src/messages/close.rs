use super::Message;
use crate::errors::Result;
use std::sync::Arc;
use tbdex::messages::{
    close::{Close as InnerClose, CloseData},
    Message as InnerMessage,
};
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub struct Close(pub InnerClose);

impl Close {
    pub fn new(
        to: String,
        from: String,
        exchange_id: String,
        data: CloseData,
        protocol: String,
        external_id: Option<String>,
    ) -> Self {
        Self(InnerClose::new(
            to,
            from,
            exchange_id,
            data,
            protocol,
            external_id,
        ))
    }

    pub fn get_data(&self) -> InnerClose {
        self.0.clone()
    }
}

impl Message for Close {
    fn sign(&self, bearer_did: Arc<BearerDid>) -> Result<()> {
        self.0
            .sign(bearer_did.0.clone())
            .map_err(|e| Arc::new(e.into()))
    }

    fn verify(&self) -> Result<()> {
        self.0.verify().map_err(|e| Arc::new(e.into()))
    }
}
