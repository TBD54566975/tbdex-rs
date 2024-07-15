use crate::errors::{Result, RustCoreError};
use std::sync::{Arc, RwLock};
use tbdex::{
    json::{FromJson, ToJson},
    messages::order::Order as InnerOrder,
};
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub struct Order(pub Arc<RwLock<InnerOrder>>);

impl Order {
    pub fn new(
        bearer_did: Arc<BearerDid>,
        to: String,
        from: String,
        exchange_id: String,
        protocol: String,
        external_id: Option<String>,
    ) -> Result<Self> {
        let order = InnerOrder::new(
            &bearer_did.0.clone(),
            &to,
            &from,
            &exchange_id,
            &protocol,
            external_id,
        )?;

        Ok(Self(Arc::new(RwLock::new(order))))
    }

    pub fn from_json_string(json: &str) -> Result<Self> {
        let inner_order = InnerOrder::from_json_string(json)?;

        Ok(Self(Arc::new(RwLock::new(inner_order))))
    }

    pub fn to_json(&self) -> Result<String> {
        let inner_order = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;

        Ok(inner_order.to_json_string()?)
    }

    pub fn get_data(&self) -> Result<InnerOrder> {
        let order = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;

        Ok(order.clone())
    }
}
