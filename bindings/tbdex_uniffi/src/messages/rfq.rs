use crate::{
    errors::{Result, TbdexError},
    resources::offering::Offering,
};
use std::sync::{Arc, RwLock};
use tbdex::{
    json::{FromJson, ToJson},
    messages::rfq::{CreateRfqData as InnerCreateRfqData, Rfq as InnerRfq},
};
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub struct Rfq(pub Arc<RwLock<InnerRfq>>);

impl Rfq {
    pub fn create(
        to: String,
        from: String,
        json_serialized_create_rfq_data: String,
        protocol: Option<String>,
        external_id: Option<String>,
    ) -> Result<Self> {
        let create_rfq_data =
            serde_json::from_str::<InnerCreateRfqData>(&json_serialized_create_rfq_data)?;
        let rfq = InnerRfq::create(&to, &from, &create_rfq_data, protocol, external_id)?;

        Ok(Self(Arc::new(RwLock::new(rfq))))
    }

    pub fn sign(&self, bearer_did: Arc<BearerDid>) -> Result<()> {
        let mut inner_rfq = self.0.write().map_err(TbdexError::from_poison_error)?;
        inner_rfq.sign(&bearer_did.0.clone())?;
        Ok(())
    }

    pub fn from_inner(inner_rfq: InnerRfq) -> Self {
        Self(Arc::new(RwLock::new(inner_rfq)))
    }

    pub fn from_json_string(json: &str) -> Result<Self> {
        let inner_rfq = InnerRfq::from_json_string(json)?;

        Ok(Self(Arc::new(RwLock::new(inner_rfq))))
    }

    pub fn to_json_string(&self) -> Result<String> {
        let inner_rfq = self.0.read().map_err(TbdexError::from_poison_error)?;

        Ok(inner_rfq.to_json_string()?)
    }

    pub fn get_data(&self) -> Result<data::Rfq> {
        let inner_rfq = self.0.read().map_err(TbdexError::from_poison_error)?;
        let json_serialized_data = serde_json::to_string(&inner_rfq.data.clone())?;
        let json_serialized_private_data = if let Some(private_data) = &inner_rfq.private_data {
            Some(serde_json::to_string(private_data)?)
        } else {
            None
        };
        Ok(data::Rfq {
            metadata: inner_rfq.metadata.clone(),
            json_serialized_data,
            json_serialized_private_data,
            signature: inner_rfq.signature.clone(),
        })
    }

    pub fn to_inner(&self) -> Result<InnerRfq> {
        let inner_rfq = self.0.read().map_err(TbdexError::from_poison_error)?;

        Ok(inner_rfq.clone())
    }

    pub fn verify(&self) -> Result<()> {
        let rfq = self.0.read().map_err(TbdexError::from_poison_error)?;

        Ok(rfq.verify()?)
    }

    pub fn verify_offering_requirements(&self, offering: Arc<Offering>) -> Result<()> {
        let rfq = self.0.read().map_err(TbdexError::from_poison_error)?;

        Ok(rfq.verify_offering_requirements(&offering.to_inner()?)?)
    }

    pub fn verify_all_private_data(&self) -> Result<()> {
        let rfq = self.0.read().map_err(TbdexError::from_poison_error)?;

        Ok(rfq.verify_all_private_data()?)
    }

    pub fn verify_present_private_data(&self) -> Result<()> {
        let rfq = self.0.read().map_err(TbdexError::from_poison_error)?;

        Ok(rfq.verify_present_private_data()?)
    }
}

pub mod data {
    use tbdex::messages::MessageMetadata;

    pub struct Rfq {
        pub metadata: MessageMetadata,
        pub json_serialized_data: String,
        pub json_serialized_private_data: Option<String>,
        pub signature: String,
    }
}
