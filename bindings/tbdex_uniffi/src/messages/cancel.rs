use crate::errors::{Result, RustCoreError};
use std::sync::{Arc, RwLock};
use tbdex::{
    json::{FromJson, ToJson},
    messages::cancel::{Cancel as InnerCancel, CancelData},
};
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub struct Cancel(pub Arc<RwLock<InnerCancel>>);

impl Cancel {
    pub fn new(
        bearer_did: Arc<BearerDid>,
        to: String,
        from: String,
        exchange_id: String,
        data: CancelData,
        protocol: String,
        external_id: Option<String>,
    ) -> Result<Self> {
        let close = InnerCancel::new(
            &bearer_did.0.clone(),
            &to,
            &from,
            &exchange_id,
            &data,
            &protocol,
            external_id,
        )?;

        Ok(Self(Arc::new(RwLock::new(close))))
    }

    pub fn from_json_string(json: &str) -> Result<Self> {
        let inner_close = InnerCancel::from_json_string(json)?;

        Ok(Self(Arc::new(RwLock::new(inner_close))))
    }

    pub fn to_json_string(&self) -> Result<String> {
        let inner_close = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;

        Ok(inner_close.to_json_string()?)
    }

    pub fn get_data(&self) -> Result<InnerCancel> {
        let close = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;

        Ok(close.clone())
    }
}
