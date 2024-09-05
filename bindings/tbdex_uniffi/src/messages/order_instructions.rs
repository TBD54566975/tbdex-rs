use crate::errors::{Result, TbdexError};
use std::sync::{Arc, RwLock};
use tbdex::{
    json::{FromJson, ToJson},
    messages::order_instructions::{
        OrderInstructions as InnerOrderInstructions, OrderInstructionsData,
    },
};
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub struct OrderInstructions(pub Arc<RwLock<InnerOrderInstructions>>);

impl OrderInstructions {
    pub fn create(
        to: String,
        from: String,
        exchange_id: String,
        data: OrderInstructionsData,
        protocol: Option<String>,
        external_id: Option<String>,
    ) -> Result<Self> {
        let order_instructions =
            InnerOrderInstructions::create(&to, &from, &exchange_id, &data, protocol, external_id)?;

        Ok(Self(Arc::new(RwLock::new(order_instructions))))
    }

    pub fn sign(&self, bearer_did: Arc<BearerDid>) -> Result<()> {
        let mut inner_order_instructions = self.0.write().map_err(TbdexError::from_poison_error)?;
        inner_order_instructions.sign(&bearer_did.0.clone())?;
        Ok(())
    }

    pub fn from_json_string(json: &str) -> Result<Self> {
        let inner_order_instructions = InnerOrderInstructions::from_json_string(json)?;

        Ok(Self(Arc::new(RwLock::new(inner_order_instructions))))
    }

    pub fn to_json_string(&self) -> Result<String> {
        let inner_order_instructions = self.0.read().map_err(TbdexError::from_poison_error)?;

        Ok(inner_order_instructions.to_json_string()?)
    }

    pub fn get_data(&self) -> Result<InnerOrderInstructions> {
        let order_instructions = self.0.read().map_err(TbdexError::from_poison_error)?;

        Ok(order_instructions.clone())
    }

    pub fn verify(&self) -> Result<()> {
        let order_instructions = self.0.read().map_err(TbdexError::from_poison_error)?;

        Ok(order_instructions.verify()?)
    }
}
