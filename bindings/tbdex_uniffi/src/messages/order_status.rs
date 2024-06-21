use super::Message;
use crate::errors::{Result, RustCoreError};
use std::sync::{Arc, RwLock};
use tbdex::messages::{
    order_status::{OrderStatus as InnerOrderStatus, OrderStatusData},
    Message as InnerMessage,
};
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub struct OrderStatus(pub Arc<RwLock<InnerOrderStatus>>);

impl OrderStatus {
    pub fn new(
        to: String,
        from: String,
        exchange_id: String,
        data: OrderStatusData,
        protocol: String,
        external_id: Option<String>,
    ) -> Result<Self> {
        let order_status =
            InnerOrderStatus::new(to, from, exchange_id, data, protocol, external_id)
                .map_err(|e| Arc::new(e.into()))?;
        Ok(Self(Arc::new(RwLock::new(order_status))))
    }

    pub fn get_data(&self) -> Result<InnerOrderStatus> {
        let order_status = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;
        Ok(order_status.clone())
    }
}

impl Message for OrderStatus {
    fn sign(&self, bearer_did: Arc<BearerDid>) -> Result<()> {
        let mut order_status = self
            .0
            .write()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockWriteError"))?;
        order_status
            .sign(bearer_did.0.clone())
            .map_err(|e| Arc::new(e.into()))
    }

    fn verify(&self) -> Result<()> {
        let order_status = self
            .0
            .write()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockWriteError"))?;
        order_status.verify().map_err(|e| Arc::new(e.into()))
    }
}
