use super::Message;
use crate::errors::{Result, RustCoreError};
use std::sync::{Arc, RwLock};
use tbdex::messages::{
    close::{Close as InnerClose, CloseData},
    Message as InnerMessage,
};
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub struct Close(pub Arc<RwLock<InnerClose>>);

impl Close {
    pub fn new(
        to: String,
        from: String,
        exchange_id: String,
        data: CloseData,
        protocol: String,
        external_id: Option<String>,
    ) -> Result<Self> {
        let close = InnerClose::new(to, from, exchange_id, data, protocol, external_id)
            .map_err(|e| Arc::new(e.into()))?;
        Ok(Self(Arc::new(RwLock::new(close))))
    }

    pub fn get_data(&self) -> Result<InnerClose> {
        let close = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;
        Ok(close.clone())
    }
}

impl Message for Close {
    fn sign(&self, bearer_did: Arc<BearerDid>) -> Result<()> {
        let mut close = self
            .0
            .write()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockWriteError"))?;
        close
            .sign(bearer_did.0.clone())
            .map_err(|e| Arc::new(e.into()))
    }

    fn verify(&self) -> Result<()> {
        let close = self
            .0
            .write()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockWriteError"))?;
        close.verify().map_err(|e| Arc::new(e.into()))
    }
}
