use crate::errors::{Result, RustCoreError};
use std::sync::{Arc, RwLock};
use tbdex::messages::order_status::{OrderStatus as InnerOrderStatus, OrderStatusData};
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub struct OrderStatus(pub Arc<RwLock<InnerOrderStatus>>);

impl OrderStatus {
    pub fn new(
        bearer_did: Arc<BearerDid>,
        to: String,
        from: String,
        exchange_id: String,
        data: OrderStatusData,
        protocol: String,
        external_id: Option<String>,
    ) -> Result<Self> {
        let order_status = InnerOrderStatus::new(
            bearer_did.0.clone(),
            to,
            from,
            exchange_id,
            data,
            protocol,
            external_id,
        )
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
