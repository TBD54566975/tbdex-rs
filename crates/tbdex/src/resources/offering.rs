use crate::jose::Signer;

use super::{ResourceKind, ResourceMetadata, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use web5::apid::{
    credentials::presentation_definition::PresentationDefinition, dids::bearer_did::BearerDid,
};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Offering {
    pub metadata: ResourceMetadata,
    pub data: OfferingData,
    pub signature: String,
}

impl Offering {
    pub fn new(
        bearer_did: BearerDid,
        from: String,
        data: OfferingData,
        protocol: String,
    ) -> Result<Self> {
        let now = Utc::now().to_rfc3339();

        let metadata = ResourceMetadata {
            kind: ResourceKind::Offering,
            from,
            id: ResourceKind::Offering.typesafe_id()?,
            protocol,
            created_at: now.clone(),
            updated_at: Some(now),
        };

        let key_id = bearer_did.document.verification_method[0].id.clone();
        let web5_signer = bearer_did.get_signer(key_id.clone())?;
        let jose_signer = Signer {
            kid: key_id,
            web5_signer,
        };

        Ok(Self {
            metadata: metadata.clone(),
            data: data.clone(),
            signature: crate::signature::sign(
                jose_signer,
                serde_json::to_value(metadata)?,
                serde_json::to_value(data)?,
            )?,
        })
    }

    pub fn from_json_string(json: &str) -> Result<Self> {
        let offering = serde_json::from_str::<Self>(json)?;

        crate::signature::verify(
            &offering.metadata.from,
            serde_json::to_value(offering.metadata.clone())?,
            serde_json::to_value(offering.data.clone())?,
            offering.signature.clone(),
        )?;

        Ok(offering)
    }

    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self)?)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OfferingData {
    pub description: String,
    pub payout_units_per_payin_unit: String,
    pub payin: PayinDetails,
    pub payout: PayoutDetails,
    pub required_claims: Option<PresentationDefinition>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PayinDetails {
    pub currency_code: String,
    pub min: Option<String>,
    pub max: Option<String>,
    pub methods: Vec<PayinMethod>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
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

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PayoutDetails {
    pub currency_code: String,
    pub min: Option<String>,
    pub max: Option<String>,
    pub methods: Vec<PayoutMethod>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
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
    fn can_create_and_sign_and_verify() {
        let key_manager = InMemoryKeyManager::new();
        let public_jwk = key_manager
            .import_private_jwk(Ed25519Generator::generate())
            .unwrap();
        let did_jwk = DidJwk::from_public_jwk(public_jwk).unwrap();

        let bearer_did = BearerDid::new(&did_jwk.did.uri, Arc::new(key_manager)).unwrap();

        let offering = Offering::new(
            bearer_did,
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
                required_claims: Some(PresentationDefinition {
                    id: "7ce4004c-3c38-4853-968b-e411bafcd945".to_string(),
                    name: None,
                    purpose: None,
                    input_descriptors: vec![],
                }),
            },
            "1.0".to_string(),
        )
        .unwrap();

        assert_ne!(String::default(), offering.signature);

        let offering_json_string = offering.to_json().unwrap();

        assert_ne!(String::default(), offering_json_string);

        let parsed_offering = Offering::from_json_string(&offering_json_string).unwrap();

        assert_eq!(offering, parsed_offering);
    }
}
