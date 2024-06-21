use super::Message;
use crate::errors::{Result, RustCoreError};
use std::sync::{Arc, RwLock};
use tbdex::messages::{
    quote::{Quote as InnerQuote, QuoteData},
    Message as InnerMessage,
};
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub struct Quote(pub Arc<RwLock<InnerQuote>>);

impl Quote {
    pub fn new(
        to: String,
        from: String,
        exchange_id: String,
        data: QuoteData,
        protocol: String,
        external_id: Option<String>,
    ) -> Result<Self> {
        let quote = InnerQuote::new(to, from, exchange_id, data, protocol, external_id)
            .map_err(|e| Arc::new(e.into()))?;
        Ok(Self(Arc::new(RwLock::new(quote))))
    }

    pub fn get_data(&self) -> Result<InnerQuote> {
        let quote = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;
        Ok(quote.clone())
    }
}

impl Message for Quote {
    fn sign(&self, bearer_did: Arc<BearerDid>) -> Result<()> {
        let mut quote = self
            .0
            .write()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockWriteError"))?;
        quote
            .sign(bearer_did.0.clone())
            .map_err(|e| Arc::new(e.into()))
    }

    fn verify(&self) -> Result<()> {
        let quote = self
            .0
            .write()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockWriteError"))?;
        quote.verify().map_err(|e| Arc::new(e.into()))
    }
}
