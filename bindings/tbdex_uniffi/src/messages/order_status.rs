use crate::errors::{Result, TbdexError};
use futures::executor::block_on;
use std::sync::{Arc, RwLock};
use tbdex::{
    json::{FromJson, ToJson},
    messages::order_status::{OrderStatus as InnerOrderStatus, OrderStatusData},
};
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub struct OrderStatus(pub Arc<RwLock<InnerOrderStatus>>);

impl OrderStatus {
    pub fn create(
        to: String,
        from: String,
        exchange_id: String,
        data: OrderStatusData,
        protocol: Option<String>,
        external_id: Option<String>,
    ) -> Result<Self> {
        let order_status =
            InnerOrderStatus::create(&to, &from, &exchange_id, &data, protocol, external_id)?;

        Ok(Self(Arc::new(RwLock::new(order_status))))
    }

    pub fn sign(&self, bearer_did: Arc<BearerDid>) -> Result<()> {
        let mut inner_order_status = self.0.write().map_err(TbdexError::from_poison_error)?;
        inner_order_status.sign(&bearer_did.0.clone())?;
        Ok(())
    }

    pub fn from_json_string(json: &str) -> Result<Self> {
        let inner_order_status = InnerOrderStatus::from_json_string(json)?;

        Ok(Self(Arc::new(RwLock::new(inner_order_status))))
    }

    pub fn to_json_string(&self) -> Result<String> {
        let inner_order_status = self.0.read().map_err(TbdexError::from_poison_error)?;

        Ok(inner_order_status.to_json_string()?)
    }

    pub fn get_data(&self) -> Result<InnerOrderStatus> {
        let order_status = self.0.read().map_err(TbdexError::from_poison_error)?;

        Ok(order_status.clone())
    }

    pub fn verify(&self) -> Result<()> {
        let order_status = self.0.read().map_err(TbdexError::from_poison_error)?;

        Ok(block_on(order_status.verify())?)
    }
}
