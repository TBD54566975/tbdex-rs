use super::Resource;
use crate::errors::Result;
use std::sync::Arc;
use tbdex::resources::{
    balance::{Balance as InnerBalance, BalanceData},
    Resource as InnerResource,
};
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub struct Balance(pub InnerBalance);

impl Balance {
    pub fn new(from: String, data: BalanceData, protocol: String) -> Self {
        Self(InnerBalance::new(from, data, protocol))
    }

    pub fn get_data(&self) -> InnerBalance {
        self.0.clone()
    }
}

impl Resource for Balance {
    fn sign(&self, bearer_did: Arc<BearerDid>) -> Result<()> {
        self.0
            .sign(bearer_did.0.clone())
            .map_err(|e| Arc::new(e.into()))
    }

    fn verify(&self) -> Result<()> {
        self.0.verify().map_err(|e| Arc::new(e.into()))
    }
}
