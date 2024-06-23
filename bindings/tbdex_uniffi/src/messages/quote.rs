use crate::errors::{Result, RustCoreError};
use std::sync::{Arc, RwLock};
use tbdex::messages::quote::{Quote as InnerQuote, QuoteData};
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub struct Quote(pub Arc<RwLock<InnerQuote>>);

impl Quote {
    pub fn new(
        bearer_did: Arc<BearerDid>,
        to: String,
        from: String,
        exchange_id: String,
        data: QuoteData,
        protocol: String,
        external_id: Option<String>,
    ) -> Result<Self> {
        let quote = InnerQuote::new(
            bearer_did.0.clone(),
            to,
            from,
            exchange_id,
            data,
            protocol,
            external_id,
        )
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
