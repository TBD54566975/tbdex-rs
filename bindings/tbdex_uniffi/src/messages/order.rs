use super::Message;
use crate::errors::{Result, RustCoreError};
use std::sync::{Arc, RwLock};
use tbdex::messages::{order::Order as InnerOrder, Message as InnerMessage};
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub struct Order(pub Arc<RwLock<InnerOrder>>);

impl Order {
    pub fn new(
        to: String,
        from: String,
        exchange_id: String,
        protocol: String,
        external_id: Option<String>,
    ) -> Result<Self> {
        let order = InnerOrder::new(to, from, exchange_id, protocol, external_id)
            .map_err(|e| Arc::new(e.into()))?;
        Ok(Self(Arc::new(RwLock::new(order))))
    }

    pub fn get_data(&self) -> Result<InnerOrder> {
        let order = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;
        Ok(order.clone())
    }
}

impl Message for Order {
    fn sign(&self, bearer_did: Arc<BearerDid>) -> Result<()> {
        let mut order = self
            .0
            .write()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockWriteError"))?;
        order
            .sign(bearer_did.0.clone())
            .map_err(|e| Arc::new(e.into()))
    }

    fn verify(&self) -> Result<()> {
        let order = self
            .0
            .write()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockWriteError"))?;
        order.verify().map_err(|e| Arc::new(e.into()))
    }
}
