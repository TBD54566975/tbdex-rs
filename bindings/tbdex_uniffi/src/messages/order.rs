use crate::errors::{Result, RustCoreError};
use std::sync::{Arc, RwLock};
use tbdex::messages::order::Order as InnerOrder;
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
            bearer_did.0.clone(),
            to,
            from,
            exchange_id,
            protocol,
            external_id,
        )
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
