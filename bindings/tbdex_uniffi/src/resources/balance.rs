use crate::errors::{Result, RustCoreError};
use std::sync::{Arc, RwLock};
use tbdex::resources::balance::{Balance as InnerBalance, BalanceData};
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub struct Balance(pub Arc<RwLock<InnerBalance>>);

impl Balance {
    pub fn new(
        bearer_did: Arc<BearerDid>,
        from: String,
        data: BalanceData,
        protocol: String,
    ) -> Result<Self> {
        let inner_balance = InnerBalance::new(&bearer_did.0.clone(), &from, &data, &protocol)
            .map_err(|e| Arc::new(e.into()))?;
        Ok(Self(Arc::new(RwLock::new(inner_balance))))
    }

    pub fn from_json_string(json: &str) -> Result<Self> {
        let inner_balance = InnerBalance::from_json_string(json).map_err(|e| Arc::new(e.into()))?;
        Ok(Self(Arc::new(RwLock::new(inner_balance))))
    }

    pub fn to_json(&self) -> Result<String> {
        let inner_balance = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;
        inner_balance.to_json().map_err(|e| Arc::new(e.into()))
    }

    pub fn get_data(&self) -> Result<InnerBalance> {
        let balance = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;
        Ok(balance.clone())
    }
}
