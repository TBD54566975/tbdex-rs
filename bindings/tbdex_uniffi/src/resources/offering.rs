use crate::errors::{Result, RustCoreError};
use std::sync::{Arc, RwLock};
use tbdex::{
    json::{FromJson, ToJson},
    resources::offering::{Offering as InnerOffering, OfferingData as InnerOfferingData},
};
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub struct Offering(pub Arc<RwLock<InnerOffering>>);

impl Offering {
    pub fn create(
        from: String,
        json_serialized_data: String,
        protocol: Option<String>,
    ) -> Result<Self> {
        let data = serde_json::from_str::<InnerOfferingData>(&json_serialized_data)?;
        let inner_offering = InnerOffering::create(&from, &data, protocol)?;
        Ok(Self(Arc::new(RwLock::new(inner_offering))))
    }

    pub fn sign(&self, bearer_did: Arc<BearerDid>) -> Result<()> {
        let mut inner_offering = self
            .0
            .write()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockWriteError"))?;
        inner_offering.sign(&bearer_did.0.clone())?;
        Ok(())
    }

    pub fn from_json_string(json: &str) -> Result<Self> {
        let inner_offering = InnerOffering::from_json_string(json)?;
        Ok(Self(Arc::new(RwLock::new(inner_offering))))
    }

    pub fn to_json_string(&self) -> Result<String> {
        let inner_offering = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;

        Ok(inner_offering.to_json_string()?)
    }

    pub fn get_data(&self) -> Result<data::Offering> {
        let inner_offering = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;
        let json_serialized_data = serde_json::to_string(&inner_offering.data.clone())?;
        Ok(data::Offering {
            metadata: inner_offering.metadata.clone(),
            json_serialized_data,
            signature: inner_offering.signature.clone(),
        })
    }

    pub fn from_inner(inner_offering: InnerOffering) -> Self {
        Self(Arc::new(RwLock::new(inner_offering)))
    }

    pub fn to_inner(&self) -> Result<InnerOffering> {
        let inner_offering = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;
        Ok(inner_offering.clone())
    }

    pub fn verify(&self) -> Result<()> {
        let inner_offering = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;
        inner_offering.verify()?;
        Ok(())
    }
}

pub mod data {
    use tbdex::resources::ResourceMetadata;

    #[derive(Clone)]
    pub struct Offering {
        pub metadata: ResourceMetadata,
        pub json_serialized_data: String,
        pub signature: String,
    }
}
