use crate::errors::{Result, RustCoreError};
use std::sync::{Arc, RwLock};
use tbdex::resources::offering::{Offering as InnerOffering, OfferingData as InnerOfferingData};
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub struct Offering(pub Arc<RwLock<InnerOffering>>);

impl Offering {
    pub fn new(
        bearer_did: Arc<BearerDid>,
        from: String,
        json_serialized_data: String,
        protocol: String,
    ) -> Result<Self> {
        let data = serde_json::from_str::<InnerOfferingData>(&json_serialized_data)
            .map_err(|e| Arc::new(e.into()))?;
        let inner_offering = InnerOffering::new(&bearer_did.0.clone(), &from, &data, &protocol)
            .map_err(|e| Arc::new(e.into()))?;
        Ok(Self(Arc::new(RwLock::new(inner_offering))))
    }

    pub fn from_json_string(json: &str) -> Result<Self> {
        let inner_offering =
            InnerOffering::from_json_string(json).map_err(|e| Arc::new(e.into()))?;
        Ok(Self(Arc::new(RwLock::new(inner_offering))))
    }

    pub fn to_json(&self) -> Result<String> {
        let inner_offering = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;
        inner_offering.to_json().map_err(|e| Arc::new(e.into()))
    }

    pub fn get_data(&self) -> Result<data::Offering> {
        let inner_offering = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;
        let json_serialized_data =
            serde_json::to_string(&inner_offering.data.clone()).map_err(|e| Arc::new(e.into()))?;
        Ok(data::Offering {
            metadata: inner_offering.metadata.clone(),
            json_serialized_data,
            signature: inner_offering.signature.clone(),
        })
    }

    pub fn to_inner(&self) -> Result<InnerOffering> {
        let inner_offering = self
            .0
            .read()
            .map_err(|e| RustCoreError::from_poison_error(e, "RwLockReadError"))?;
        Ok(inner_offering.clone())
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
