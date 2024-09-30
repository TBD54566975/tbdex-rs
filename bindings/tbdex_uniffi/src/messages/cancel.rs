use crate::errors::{Result, TbdexError};
use futures::executor::block_on;
use std::sync::{Arc, RwLock};
use tbdex::{
    json::{FromJson, ToJson},
    messages::cancel::{Cancel as InnerCancel, CancelData},
};
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub struct Cancel(pub Arc<RwLock<InnerCancel>>);

impl Cancel {
    pub fn create(
        to: String,
        from: String,
        exchange_id: String,
        data: CancelData,
        protocol: Option<String>,
        external_id: Option<String>,
    ) -> Result<Self> {
        let close = InnerCancel::create(&to, &from, &exchange_id, &data, protocol, external_id)?;

        Ok(Self(Arc::new(RwLock::new(close))))
    }

    pub fn sign(&self, bearer_did: Arc<BearerDid>) -> Result<()> {
        let mut inner_close = self.0.write().map_err(TbdexError::from_poison_error)?;
        inner_close.sign(&bearer_did.0.clone())?;
        Ok(())
    }

    pub fn from_json_string(json: &str) -> Result<Self> {
        let inner_close = InnerCancel::from_json_string(json)?;

        Ok(Self(Arc::new(RwLock::new(inner_close))))
    }

    pub fn to_json_string(&self) -> Result<String> {
        let inner_close = self.0.read().map_err(TbdexError::from_poison_error)?;

        Ok(inner_close.to_json_string()?)
    }

    pub fn get_data(&self) -> Result<InnerCancel> {
        let close = self.0.read().map_err(TbdexError::from_poison_error)?;

        Ok(close.clone())
    }

    pub fn verify(&self) -> Result<()> {
        let cancel = self.0.read().map_err(TbdexError::from_poison_error)?;

        Ok(block_on(cancel.verify())?)
    }
}
