use super::Message;
use crate::{errors::Result, resources::offering::Offering};
use std::sync::Arc;
use tbdex::messages::{
    rfq::{CreateRfqData, Rfq as InnerRfq},
    Message as InnerMessage,
};
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub struct Rfq(pub InnerRfq);

impl Rfq {
    pub fn new(
        to: String,
        from: String,
        create_rfq_data: CreateRfqData,
        protocol: String,
        external_id: Option<String>,
    ) -> Self {
        Self(InnerRfq::new(
            to,
            from,
            create_rfq_data,
            protocol,
            external_id,
        ))
    }

    pub fn get_data(&self) -> InnerRfq {
        self.0.clone()
    }

    pub fn verify_offering_requirements(&self, offering: Arc<Offering>) -> Result<bool> {
        self.0
            .verify_offering_requirements(offering.0.clone())
            .map_err(|e| Arc::new(e.into()))
    }

    pub fn verify_all_private_data(&self) -> Result<bool> {
        self.0
            .verify_all_private_data()
            .map_err(|e| Arc::new(e.into()))
    }

    pub fn verify_present_private_data(&self) -> Result<bool> {
        self.0
            .verify_present_private_data()
            .map_err(|e| Arc::new(e.into()))
    }
}

impl Message for Rfq {
    fn sign(&self, bearer_did: Arc<BearerDid>) -> Result<()> {
        self.0
            .sign(bearer_did.0.clone())
            .map_err(|e| Arc::new(e.into()))
    }

    fn verify(&self) -> Result<()> {
        self.0.verify().map_err(|e| Arc::new(e.into()))
    }
}
