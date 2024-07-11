use super::{ResourceKind, ResourceMetadata, Result};
use crate::json_schemas::generated::{OFFERING_DATA_JSON_SCHEMA, RESOURCE_JSON_SCHEMA};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use web5::{
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
        bearer_did: &BearerDid,
        from: &str,
        data: &OfferingData,
        protocol: &str,
    ) -> Result<Self> {
        let _now = Utc::now().to_rfc3339();

        let metadata = ResourceMetadata {
            kind: ResourceKind::Offering,
            from: from.to_string(),
            // id: ResourceKind::Offering.typesafe_id()?,
            protocol: protocol.to_string(),
            // created_at: now.clone(),
            // updated_at: Some(now),
            id: "offering_01j2gw7tdkej6scmjvt5ew2rjk".to_string(),
            created_at: "2024-07-11T12:28:09Z".to_string(),
            updated_at: Some("2024-07-11T12:28:09Z".to_string()),
        };

        let offering = Self {
            metadata: metadata.clone(),
            data: data.clone(),
            signature: crate::signature::sign(
                bearer_did,
                &serde_json::to_value(metadata)?,
                &serde_json::to_value(data)?,
            )?,
        };

        offering.verify()?;

        Ok(offering)
    }

    pub fn from_json_string(json: &str) -> Result<Self> {
        let offering = serde_json::from_str::<Self>(json)?;
        offering.verify()?;
        Ok(offering)
    }

    pub fn verify(&self) -> Result<()> {
        // verify resource json schema
        crate::json_schemas::validate_from_str(RESOURCE_JSON_SCHEMA, self)?;

        // verify data json schema
        crate::json_schemas::validate_from_str(OFFERING_DATA_JSON_SCHEMA, &self.data)?;

        // verify signature
        crate::signature::verify(
            &self.metadata.from,
            &serde_json::to_value(self.metadata.clone())?,
            &serde_json::to_value(self.data.clone())?,
            &self.signature,
        )?;

        Ok(())
    }

    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self)?)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct OfferingData {
    pub description: String,
    pub payout_units_per_payin_unit: String,
    pub payin: PayinDetails,
    pub payout: PayoutDetails,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_claims: Option<PresentationDefinition>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PayinDetails {
    pub currency_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<String>,
    pub methods: Vec<PayinMethod>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PayinMethod {
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_payment_details: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PayoutDetails {
    pub currency_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<String>,
    pub methods: Vec<PayoutMethod>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PayoutMethod {
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_payment_details: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<String>,
    pub estimated_settlement_time: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use web5::{
        crypto::{
            dsa::ed25519::Ed25519Generator, key_managers::in_memory_key_manager::InMemoryKeyManager,
        },
        dids::{methods::did_jwk::DidJwk, portable_did::PortableDid},
    };

    // #[test]
    // fn can_create_and_sign_and_verify() {
    //     let key_manager = InMemoryKeyManager::new();
    //     let public_jwk = key_manager
    //         .import_private_jwk(Ed25519Generator::generate())
    //         .unwrap();
    //     let did_jwk = DidJwk::from_public_jwk(public_jwk).unwrap();

    //     let bearer_did = BearerDid::new(&did_jwk.did.uri, Arc::new(key_manager)).unwrap();

    //     let offering = Offering::new(
    //         &bearer_did,
    //         &did_jwk.did.uri,
    //         &OfferingData {
    //             description: "Selling BTC for USD".to_string(),
    //             payout_units_per_payin_unit: "1.5".to_string(),
    //             payin: PayinDetails {
    //                 currency_code: "USD".to_string(),
    //                 ..Default::default()
    //             },
    //             payout: PayoutDetails {
    //                 currency_code: "BTC".to_string(),
    //                 ..Default::default()
    //             },
    //             required_claims: Some(PresentationDefinition {
    //                 id: "7ce4004c-3c38-4853-968b-e411bafcd945".to_string(),
    //                 name: None,
    //                 purpose: None,
    //                 input_descriptors: vec![],
    //             }),
    //         },
    //         "1.0",
    //     )
    //     .unwrap();

    //     assert_ne!(String::default(), offering.signature);

    //     let offering_json_string = offering.to_json().unwrap();

    //     assert_ne!(String::default(), offering_json_string);

    //     let parsed_offering = Offering::from_json_string(&offering_json_string).unwrap();

    //     assert_eq!(offering, parsed_offering);
    // }

    #[test]
    fn test_simple_offering() {
        let portable_did = PortableDid::new(r###"{"uri":"did:jwk:eyJrdHkiOiJPS1AiLCJjcnYiOiJFZDI1NTE5IiwieCI6ImpWM3BCeUtmYXkwTHdFQ2lwZ3VUU1MyWExNTDNWQ0UzNnBUVFltZk5ONTQifQ","privateKeys":[{"kty":"OKP","crv":"Ed25519","d":"aIdFbVAIgnqnrH-TDLyZVAEP9QD6vt5C9fhUkPystB-NXekHIp9rLQvAQKKmC5NJLZcswvdUITfqlNNiZ803ng","x":"jV3pByKfay0LwECipguTSS2XLML3VCE36pTTYmfNN54"}],"document":{"@context":["https://www.w3.org/ns/did/v1"],"id":"did:jwk:eyJrdHkiOiJPS1AiLCJjcnYiOiJFZDI1NTE5IiwieCI6ImpWM3BCeUtmYXkwTHdFQ2lwZ3VUU1MyWExNTDNWQ0UzNnBUVFltZk5ONTQifQ","verificationMethod":[{"id":"did:jwk:eyJrdHkiOiJPS1AiLCJjcnYiOiJFZDI1NTE5IiwieCI6ImpWM3BCeUtmYXkwTHdFQ2lwZ3VUU1MyWExNTDNWQ0UzNnBUVFltZk5ONTQifQ#0","type":"JsonWebKey","controller":"did:jwk:eyJrdHkiOiJPS1AiLCJjcnYiOiJFZDI1NTE5IiwieCI6ImpWM3BCeUtmYXkwTHdFQ2lwZ3VUU1MyWExNTDNWQ0UzNnBUVFltZk5ONTQifQ","publicKeyJwk":{"kty":"OKP","crv":"Ed25519","x":"jV3pByKfay0LwECipguTSS2XLML3VCE36pTTYmfNN54"}}],"assertionMethod":["did:jwk:eyJrdHkiOiJPS1AiLCJjcnYiOiJFZDI1NTE5IiwieCI6ImpWM3BCeUtmYXkwTHdFQ2lwZ3VUU1MyWExNTDNWQ0UzNnBUVFltZk5ONTQifQ#0"],"authentication":["did:jwk:eyJrdHkiOiJPS1AiLCJjcnYiOiJFZDI1NTE5IiwieCI6ImpWM3BCeUtmYXkwTHdFQ2lwZ3VUU1MyWExNTDNWQ0UzNnBUVFltZk5ONTQifQ#0"],"capabilityDelegation":["did:jwk:eyJrdHkiOiJPS1AiLCJjcnYiOiJFZDI1NTE5IiwieCI6ImpWM3BCeUtmYXkwTHdFQ2lwZ3VUU1MyWExNTDNWQ0UzNnBUVFltZk5ONTQifQ#0"],"capabilityInvocation":["did:jwk:eyJrdHkiOiJPS1AiLCJjcnYiOiJFZDI1NTE5IiwieCI6ImpWM3BCeUtmYXkwTHdFQ2lwZ3VUU1MyWExNTDNWQ0UzNnBUVFltZk5ONTQifQ#0"]},"metadata":null}"###).unwrap();
        let bearer_did = BearerDid::from_portable_did(portable_did).unwrap();

        let offering = Offering::new(
            &bearer_did,
            &bearer_did.did.uri,
            &OfferingData {
                description: "USDC for USD".to_string(),
                payout_units_per_payin_unit: "1.0".to_string(),
                payin: PayinDetails {
                    currency_code: "USD".to_string(),
                    methods: vec![PayinMethod {
                        kind: "SQUAREPAY".to_string(),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                payout: PayoutDetails {
                    currency_code: "USDC".to_string(),
                    methods: vec![PayoutMethod {
                        kind: "STORED_BALANCE".to_string(),
                        estimated_settlement_time: 1200,
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                ..Default::default()
            },
            "1.0",
        )
        .unwrap();

        // ---
        println!("{}", serde_json::to_string_pretty(&offering).unwrap());
        // ---

        offering.verify().unwrap();
    }
}

// TODO: Fix offering test vector - https://github.com/TBD54566975/tbdex/issues/346
// #[cfg(test)]
// mod tbdex_test_vectors_protocol {
//     use super::*;
//     use std::fs;
//
//     #[derive(Debug, serde::Deserialize)]
//     pub struct TestVector {
//         pub input: String,
//         pub output: Offering,
//     }
//
//     #[test]
//     fn parse_offering() {
//         let path = "../../tbdex/hosted/test-vectors/protocol/vectors/parse-offering.json";
//         let test_vector_json: String = fs::read_to_string(path).unwrap();
//
//         let test_vector: TestVector = serde_json::from_str(&test_vector_json).unwrap();
//         let parsed_offering: Offering = Offering::from_json_string(&test_vector.input).unwrap();
//
//         // assert_eq!(test_vector.output, parsed_offering);
//     }
// }
