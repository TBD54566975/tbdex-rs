use super::Resource;
use crate::errors::{Result, RustCoreError};
use std::sync::{Arc, RwLock};
use tbdex::resources::{
    offering::{Offering as InnerOffering, OfferingData},
    Resource as InnerResource,
};
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub struct Offering(pub Arc<RwLock<InnerOffering>>);

impl Offering {
    pub fn new(from: String, data: OfferingData, protocol: String) -> Result<Self> {
        let offering = InnerOffering::new(from, data, protocol).map_err(|e| Arc::new(e.into()))?;
        Ok(Self(Arc::new(RwLock::new(offering))))
    }

    pub fn get_data(&self) -> Result<InnerOffering> {
        let offering = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;
        Ok(offering.clone())
    }
}

impl Resource for Offering {
    fn sign(&self, bearer_did: Arc<BearerDid>) -> Result<()> {
        let mut offering = self
            .0
            .write()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockWriteError"))?;

        offering
            .sign(bearer_did.0.clone())
            .map_err(|e| Arc::new(e.into()))
    }

    fn verify(&self) -> Result<()> {
        let offering = self
            .0
            .write()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockWriteError"))?;

        offering.verify().map_err(|e| Arc::new(e.into()))
    }
}
