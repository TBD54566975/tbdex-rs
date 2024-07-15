use crate::errors::{Result, RustCoreError};
use std::sync::{Arc, RwLock};
use tbdex::{
    json::{FromJson, ToJson},
    messages::order_status::{OrderStatus as InnerOrderStatus, OrderStatusData},
};
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
        let order_status = InnerOrderStatus::create(
            &bearer_did.0.clone(),
            &to,
            &from,
            &exchange_id,
            &data,
            &protocol,
            external_id,
        )?;

        Ok(Self(Arc::new(RwLock::new(order_status))))
    }

    pub fn from_json_string(json: &str) -> Result<Self> {
        let inner_order_status = InnerOrderStatus::from_json_string(json)?;

        Ok(Self(Arc::new(RwLock::new(inner_order_status))))
    }

    pub fn to_json_string(&self) -> Result<String> {
        let inner_order_status = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;

        Ok(inner_order_status.to_json_string()?)
    }

    pub fn get_data(&self) -> Result<InnerOrderStatus> {
        let order_status = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;

        Ok(order_status.clone())
    }
}
