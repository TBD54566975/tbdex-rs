use super::Message;
use crate::{
    errors::{Result, RustCoreError},
    resources::offering::Offering,
};
use std::sync::{Arc, RwLock};
use tbdex::messages::{
    rfq::{CreateRfqData, Rfq as InnerRfq},
    Message as InnerMessage,
};
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub struct Rfq(pub Arc<RwLock<InnerRfq>>);

impl Rfq {
    pub fn new(
        to: String,
        from: String,
        create_rfq_data: CreateRfqData,
        protocol: String,
        external_id: Option<String>,
    ) -> Result<Self> {
        let rfq = InnerRfq::new(to, from, create_rfq_data, protocol, external_id)
            .map_err(|e| Arc::new(e.into()))?;
        Ok(Self(Arc::new(RwLock::new(rfq))))
    }

    pub fn get_data(&self) -> Result<InnerRfq> {
        let rfq = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;
        Ok(rfq.clone())
    }

    pub fn verify_offering_requirements(&self, offering: Arc<Offering>) -> Result<bool> {
        let rfq = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;
        rfq.verify_offering_requirements(offering.to_inner()?)
            .map_err(|e| Arc::new(e.into()))
    }

    pub fn verify_all_private_data(&self) -> Result<bool> {
        let rfq = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;
        rfq.verify_all_private_data()
            .map_err(|e| Arc::new(e.into()))
    }

    pub fn verify_present_private_data(&self) -> Result<bool> {
        let rfq = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;
        rfq.verify_present_private_data()
            .map_err(|e| Arc::new(e.into()))
    }
}

impl Message for Rfq {
    fn sign(&self, bearer_did: Arc<BearerDid>) -> Result<()> {
        let mut rfq = self
            .0
            .write()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockWriteError"))?;
        rfq.sign(bearer_did.0.clone())
            .map_err(|e| Arc::new(e.into()))
    }

    fn verify(&self) -> Result<()> {
        let rfq = self
            .0
            .write()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockWriteError"))?;
        rfq.verify().map_err(|e| Arc::new(e.into()))
    }
}
