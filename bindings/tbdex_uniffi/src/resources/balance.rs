use super::Resource;
use crate::errors::{Result, RustCoreError};
use std::sync::{Arc, RwLock};
use tbdex::resources::{
    balance::{Balance as InnerBalance, BalanceData},
    Resource as InnerResource,
};
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub struct Balance(pub Arc<RwLock<InnerBalance>>);

impl Balance {
    pub fn new(from: String, data: BalanceData, protocol: String) -> Result<Self> {
        let balance = InnerBalance::new(from, data, protocol).map_err(|e| Arc::new(e.into()))?;
        Ok(Self(Arc::new(RwLock::new(balance))))
    }

    pub fn get_data(&self) -> Result<InnerBalance> {
        let balance = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;
        Ok(balance.clone())
    }
}

impl Resource for Balance {
    fn sign(&self, bearer_did: Arc<BearerDid>) -> Result<()> {
        let mut balance = self
            .0
            .write()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockWriteError"))?;
        balance
            .sign(bearer_did.0.clone())
            .map_err(|e| Arc::new(e.into()))
    }

    fn verify(&self) -> Result<()> {
        let balance = self
            .0
            .write()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockWriteError"))?;
        balance.verify().map_err(|e| Arc::new(e.into()))
    }
}
