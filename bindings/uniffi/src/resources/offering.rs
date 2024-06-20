use super::Resource;
use crate::errors::Result;
use std::sync::Arc;
use tbdex::resources::{
    offering::{Offering as InnerOffering, OfferingData},
    Resource as InnerResource,
};
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub struct Offering(pub InnerOffering);

impl Offering {
    pub fn new(from: String, data: OfferingData, protocol: String) -> Self {
        Self(InnerOffering::new(from, data, protocol))
    }
}

impl Resource for Offering {
    fn sign(&self, bearer_did: Arc<BearerDid>) -> Result<()> {
        self.0
            .sign(bearer_did.0.clone())
            .map_err(|e| Arc::new(e.into()))
    }

    fn verify(&self) -> Result<()> {
        self.0.verify().map_err(|e| Arc::new(e.into()))
    }
}
