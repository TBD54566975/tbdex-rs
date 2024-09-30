use crate::errors::{Result, TbdexError};
use futures::executor::block_on;
use std::sync::{Arc, RwLock};
use tbdex::{
    json::{FromJson, ToJson},
    resources::balance::{Balance as InnerBalance, BalanceData},
};
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub struct Balance(pub Arc<RwLock<InnerBalance>>);

impl Balance {
    pub fn create(from: String, data: BalanceData, protocol: Option<String>) -> Result<Self> {
        let inner_balance = InnerBalance::create(&from, &data, protocol)?;
        Ok(Self(Arc::new(RwLock::new(inner_balance))))
    }

    pub fn sign(&self, bearer_did: Arc<BearerDid>) -> Result<()> {
        let mut inner_balance = self.0.write().map_err(TbdexError::from_poison_error)?;
        inner_balance.sign(&bearer_did.0.clone())?;
        Ok(())
    }

    pub fn from_json_string(json: &str) -> Result<Self> {
        let inner_balance = InnerBalance::from_json_string(json)?;

        Ok(Self(Arc::new(RwLock::new(inner_balance))))
    }

    pub fn to_json_string(&self) -> Result<String> {
        let inner_balance = self.0.read().map_err(TbdexError::from_poison_error)?;

        Ok(inner_balance.to_json_string()?)
    }

    pub fn get_data(&self) -> Result<InnerBalance> {
        let balance = self.0.read().map_err(TbdexError::from_poison_error)?;

        Ok(balance.clone())
    }

    pub fn from_inner(inner_balance: InnerBalance) -> Self {
        Self(Arc::new(RwLock::new(inner_balance)))
    }

    pub fn to_inner(&self) -> Result<InnerBalance> {
        let inner_balance = self.0.read().map_err(TbdexError::from_poison_error)?;
        Ok(inner_balance.clone())
    }

    pub fn verify(&self) -> Result<()> {
        let inner_balance = self.0.read().map_err(TbdexError::from_poison_error)?;
        block_on(inner_balance.verify())?;
        Ok(())
    }
}
