use super::{Resource, ResourceKind, ResourceMetadata, Result};
use crate::signer::sign;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use web5::apid::{
    credentials::presentation_definition::PresentationDefinition, dids::bearer_did::BearerDid,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Offering {
    pub metadata: ResourceMetadata,
    pub data: OfferingData,
    pub signature: String,
}

impl Offering {
    pub fn new(from: String, data: OfferingData, protocol: String) -> Result<Self> {
        let now = Utc::now().to_rfc3339();

        Ok(Self {
            metadata: ResourceMetadata {
                kind: ResourceKind::Offering,
                from,
                id: ResourceKind::Offering.typesafe_id()?,
                protocol,
                created_at: now.clone(),
                updated_at: Some(now),
            },
            data,
            signature: String::default(), // not set until call to sign()
        })
    }
}

impl Resource for Offering {
    fn sign(&mut self, bearer_did: BearerDid) -> Result<()> {
        let metadata = serde_json::to_value(&self.metadata)?;
        let data = serde_json::to_value(&self.data)?;

        self.signature = sign(bearer_did, metadata, data);

        Ok(())
    }

    fn verify(&self) -> Result<()> {
        println!("Offering.verify() invoked");
        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OfferingData {
    pub description: String,
    pub payout_units_per_payin_unit: String,
    pub payin: PayinDetails,
    pub payout: PayoutDetails,
    pub required_claims: PresentationDefinition,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct PayinDetails {
    pub currency_code: String,
    pub min: Option<String>,
    pub max: Option<String>,
    pub methods: Vec<PayinMethod>,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct PayinMethod {
    pub kind: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub group: Option<String>,
    pub required_payment_details: Option<serde_json::Value>,
    pub fee: Option<String>,
    pub min: Option<String>,
    pub max: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct PayoutDetails {
    pub currency_code: String,
    pub min: Option<String>,
    pub max: Option<String>,
    pub methods: Vec<PayoutMethod>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PayoutMethod {
    pub kind: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub group: Option<String>,
    pub required_payment_details: Option<serde_json::Value>,
    pub fee: Option<String>,
    pub min: Option<String>,
    pub max: Option<String>,
    pub estimated_settlement_time: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use web5::apid::{
        crypto::key_managers::in_memory_key_manager::InMemoryKeyManager,
        dids::methods::did_jwk::DidJwk, dsa::ed25519::Ed25519Generator,
    };

    #[test]
    fn can_create_and_sign() {
        let key_manager = InMemoryKeyManager::new();
        let public_jwk = key_manager
            .import_private_jwk(Ed25519Generator::generate())
            .unwrap();
        let did_jwk = DidJwk::from_public_jwk(public_jwk).unwrap();

        let bearer_did = BearerDid::new(&did_jwk.did.uri, Arc::new(key_manager)).unwrap();

        let mut offering = Offering::new(
            did_jwk.did.uri,
            OfferingData {
                description: "Selling BTC for USD".to_string(),
                payout_units_per_payin_unit: "1.5".to_string(),
                payin: PayinDetails {
                    currency_code: "USD".to_string(),
                    ..Default::default()
                },
                payout: PayoutDetails {
                    currency_code: "BTC".to_string(),
                    ..Default::default()
                },
                required_claims: PresentationDefinition {
                    id: "7ce4004c-3c38-4853-968b-e411bafcd945".to_string(),
                    name: None,
                    purpose: None,
                    input_descriptors: vec![],
                },
            },
            "1.0".to_string(),
        )
        .unwrap();

        offering.sign(bearer_did).unwrap();

        assert_ne!(String::default(), offering.signature)
    }
}
