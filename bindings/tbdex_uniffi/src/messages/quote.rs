use crate::errors::{Result, RustCoreError};
use std::sync::{Arc, RwLock};
use tbdex::{
    json::{FromJson, ToJson},
    messages::quote::{Quote as InnerQuote, QuoteData},
};
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
        let quote = InnerQuote::create(
            &bearer_did.0.clone(),
            &to,
            &from,
            &exchange_id,
            &data,
            &protocol,
            external_id,
        )?;

        Ok(Self(Arc::new(RwLock::new(quote))))
    }

    pub fn from_json_string(json: &str) -> Result<Self> {
        let inner_quote = InnerQuote::from_json_string(json)?;

        Ok(Self(Arc::new(RwLock::new(inner_quote))))
    }

    pub fn to_json_string(&self) -> Result<String> {
        let inner_quote = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;

        Ok(inner_quote.to_json_string()?)
    }

    pub fn get_data(&self) -> Result<InnerQuote> {
        let quote = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;

        Ok(quote.clone())
    }

    pub fn verify(&self) -> Result<()> {
        let quote = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;

        Ok(quote.verify()?)
    }
}
