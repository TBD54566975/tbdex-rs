use crate::errors::{Result, RustCoreError};
use std::sync::{Arc, RwLock};
use tbdex::messages::close::{Close as InnerClose, CloseData};
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub struct Close(pub Arc<RwLock<InnerClose>>);

impl Close {
    pub fn new(
        bearer_did: Arc<BearerDid>,
        to: String,
        from: String,
        exchange_id: String,
        data: CloseData,
        protocol: String,
        external_id: Option<String>,
    ) -> Result<Self> {
        let close = InnerClose::new(
            &bearer_did.0.clone(),
            &to,
            &from,
            &exchange_id,
            &data,
            &protocol,
            external_id,
        )
        .map_err(|e| Arc::new(e.into()))?;
        Ok(Self(Arc::new(RwLock::new(close))))
    }

    pub fn from_json_string(json: &str) -> Result<Self> {
        let inner_close = InnerClose::from_json_string(json).map_err(|e| Arc::new(e.into()))?;
        Ok(Self(Arc::new(RwLock::new(inner_close))))
    }

    pub fn to_json(&self) -> Result<String> {
        let inner_close = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;
        inner_close.to_json().map_err(|e| Arc::new(e.into()))
    }

    pub fn get_data(&self) -> Result<InnerClose> {
        let close = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;
        Ok(close.clone())
    }
}
