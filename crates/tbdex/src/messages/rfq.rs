use super::{MessageKind, MessageMetadata, Result};
use crate::resources::offering::Offering;
use base64::{engine::general_purpose, Engine as _};
use rand::{rngs::OsRng, RngCore};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use web5::apid::dids::bearer_did::BearerDid;

#[derive(Clone, Serialize, Default, Deserialize, Debug, PartialEq)]
pub struct Rfq {
    pub metadata: MessageMetadata,
    pub data: RfqData,
    pub private_data: RfqPrivateData,
    pub signature: String,
}

impl Rfq {
    pub fn new(
        bearer_did: BearerDid,
        to: String,
        from: String,
        create_rfq_data: CreateRfqData,
        protocol: String,
        external_id: Option<String>,
    ) -> Result<Self> {
        let id = MessageKind::Rfq.typesafe_id()?;

        let metadata = MessageMetadata {
            from,
            to,
            kind: MessageKind::Rfq,
            id: id.clone(),
            exchange_id: id.clone(),
            external_id,
            protocol,
            created_at: String::default(),
        };

        let (data, private_data) = hash_private_data(&create_rfq_data);

        Ok(Self {
            metadata: metadata.clone(),
            data: data.clone(),
            private_data,
            signature: crate::signature::sign(
                bearer_did,
                serde_json::to_value(metadata)?,
                serde_json::to_value(data)?,
            )?,
        })
    }

    pub fn from_json_string(json: &str) -> Result<Self> {
        let rfq = serde_json::from_str::<Self>(json)?;

        crate::signature::verify(
            &rfq.metadata.from,
            serde_json::to_value(rfq.metadata.clone())?,
            serde_json::to_value(rfq.data.clone())?,
            rfq.signature.clone(),
        )?;

        Ok(rfq)
    }

    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self)?)
    }

    pub fn verify_offering_requirements(&self, _offering: Offering) -> Result<bool> {
        println!("Rfq.verify_offering_requirements() invoked");
        Ok(true)
    }

    pub fn verify_all_private_data(&self) -> Result<bool> {
        println!("Rfq.verify_all_private_data() invoked");
        Ok(true)
    }

    pub fn verify_present_private_data(&self) -> Result<bool> {
        println!("Rfq.verify_present_private_data() invoked");
        Ok(true)
    }
}

#[derive(Clone)]
pub struct CreateRfqData {
    pub offering_id: String,
    pub payin: CreateSelectedPayinMethod,
    pub payout: CreateSelectedPayoutMethod,
    pub claims: Vec<String>,
}

#[derive(Clone)]
pub struct CreateSelectedPayinMethod {
    pub kind: String,
    pub payment_details: serde_json::Value,
    pub amount: String,
}

#[derive(Clone)]
pub struct CreateSelectedPayoutMethod {
    pub kind: String,
    pub payment_details: serde_json::Value,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
pub struct RfqData {
    pub offering_id: String,
    pub payin: SelectedPayinMethod,
    pub payout: SelectedPayoutMethod,
    pub claims_hash: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
pub struct SelectedPayinMethod {
    pub kind: String,
    pub payment_details_hash: Option<String>,
    pub amount: String,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
pub struct SelectedPayoutMethod {
    pub kind: String,
    pub payment_details_hash: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
pub struct RfqPrivateData {
    pub salt: String,
    pub payin: Option<PrivatePaymentDetails>,
    pub payout: Option<PrivatePaymentDetails>,
    pub claims: Option<Vec<String>>,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
pub struct PrivatePaymentDetails {
    pub payment_details: serde_json::Value,
}

fn hash_private_data(create_rfq_data: &CreateRfqData) -> (RfqData, RfqPrivateData) {
    let salt = generate_random_salt();

    let payin_payment_details_hash =
        digest_private_data(&salt, &create_rfq_data.payin.payment_details);
    let payout_payment_details_hash =
        digest_private_data(&salt, &create_rfq_data.payout.payment_details);
    let claims_hash = if create_rfq_data.claims.is_empty() {
        None
    } else {
        Some(digest_private_data(&salt, &create_rfq_data.claims))
    };

    let hashed_rfq_data = RfqData {
        offering_id: create_rfq_data.offering_id.clone(),
        payin: SelectedPayinMethod {
            kind: create_rfq_data.payin.kind.clone(),
            payment_details_hash: Some(payin_payment_details_hash),
            amount: create_rfq_data.payin.amount.clone(),
        },
        payout: SelectedPayoutMethod {
            kind: create_rfq_data.payout.kind.clone(),
            payment_details_hash: Some(payout_payment_details_hash),
        },
        claims_hash,
    };

    let private_rfq_data = RfqPrivateData {
        salt: salt.clone(),
        payin: Some(PrivatePaymentDetails {
            payment_details: create_rfq_data.payin.payment_details.clone(),
        }),
        payout: Some(PrivatePaymentDetails {
            payment_details: create_rfq_data.payout.payment_details.clone(),
        }),
        claims: Some(create_rfq_data.claims.clone()),
    };

    (hashed_rfq_data, private_rfq_data)
}

fn generate_random_salt() -> String {
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);
    general_purpose::URL_SAFE_NO_PAD.encode(salt)
}

fn digest_private_data<T: Serialize>(salt: &str, value: &T) -> String {
    let payload = format!(
        "{}{}",
        salt,
        serde_json::to_string(value).expect("Failed to serialize value")
    );
    let mut hasher = Sha256::new();
    hasher.update(payload.as_bytes());
    let digest = hasher.finalize();
    general_purpose::URL_SAFE_NO_PAD.encode(digest)
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

        let rfq = Rfq::new(
            bearer_did,
            "did:test:pfi".to_string(),
            did_jwk.did.uri.clone(),
            CreateRfqData {
                offering_id: "offering_123".to_string(),
                payin: CreateSelectedPayinMethod {
                    kind: "BTC".to_string(),
                    payment_details: serde_json::json!({"tmp": "payment-details"}),
                    amount: "101".to_string(),
                },
                payout: CreateSelectedPayoutMethod {
                    kind: "BTC".to_string(),
                    payment_details: serde_json::json!({"tmp": "payment-details"}),
                },
                claims: vec!["some-claim".to_string()],
            },
            "1.0".to_string(),
            None,
        )
        .unwrap();

        assert_ne!(String::default(), rfq.signature);

        let rfq_json_string = rfq.to_json().unwrap();

        assert_ne!(String::default(), rfq_json_string);

        let parsed_rfq = Rfq::from_json_string(&rfq_json_string).unwrap();

        assert_eq!(rfq, parsed_rfq);
    }
}
