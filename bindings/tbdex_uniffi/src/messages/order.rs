use crate::errors::{Result, TbdexError};
use futures::executor::block_on;
use std::sync::{Arc, RwLock};
use tbdex::{
    json::{FromJson, ToJson},
    messages::order::Order as InnerOrder,
};
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub struct Order(pub Arc<RwLock<InnerOrder>>);

impl Order {
    pub fn create(
        to: String,
        from: String,
        exchange_id: String,
        protocol: Option<String>,
        external_id: Option<String>,
    ) -> Result<Self> {
        let order = InnerOrder::create(&to, &from, &exchange_id, protocol, external_id)?;

        Ok(Self(Arc::new(RwLock::new(order))))
    }

    pub fn sign(&self, bearer_did: Arc<BearerDid>) -> Result<()> {
        let mut inner_order = self.0.write().map_err(TbdexError::from_poison_error)?;
        inner_order.sign(&bearer_did.0.clone())?;
        Ok(())
    }

    pub fn from_json_string(json: &str) -> Result<Self> {
        let inner_order = InnerOrder::from_json_string(json)?;

        Ok(Self(Arc::new(RwLock::new(inner_order))))
    }

    pub fn to_json_string(&self) -> Result<String> {
        let inner_order = self.0.read().map_err(TbdexError::from_poison_error)?;

        Ok(inner_order.to_json_string()?)
    }

    pub fn get_data(&self) -> Result<InnerOrder> {
        let order = self.0.read().map_err(TbdexError::from_poison_error)?;

        Ok(order.clone())
    }

    pub fn verify(&self) -> Result<()> {
        let order = self.0.read().map_err(TbdexError::from_poison_error)?;

        Ok(block_on(order.verify())?)
    }
}
