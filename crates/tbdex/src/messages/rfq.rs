use super::{Message, MessageKind, MessageMetadata, Result};
use crate::{resources::offering::Offering, signer::sign};
use base64::{engine::general_purpose, Engine as _};
use rand::{rngs::OsRng, RngCore};
use serde::Serialize;
use sha2::{Digest, Sha256};
use web5::apid::dids::bearer_did::BearerDid;

#[derive(Clone)]
pub struct Rfq {
    pub metadata: MessageMetadata,
    pub data: RfqData,
    pub private_data: RfqPrivateData,
    pub signature: String,
}

impl Rfq {
    pub fn new(
        to: String,
        from: String,
        create_rfq_data: CreateRfqData,
        protocol: String,
        external_id: Option<String>,
    ) -> Result<Self> {
        let id = MessageKind::Rfq.typesafe_id()?;
        let (data, private_data) = hash_private_data(&create_rfq_data);

        Ok(Self {
            metadata: MessageMetadata {
                from,
                to,
                kind: MessageKind::Rfq,
                id: id.clone(),
                exchange_id: id.clone(),
                external_id,
                protocol,
                created_at: String::default(),
            },
            data,
            private_data,
            signature: String::default(), // not set until call to sign()
        })
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

impl Message for Rfq {
    fn sign(&mut self, bearer_did: BearerDid) -> Result<()> {
        let metadata = serde_json::to_value(&self.metadata)?;
        let data = serde_json::to_value(&self.data)?;

        self.signature = sign(bearer_did, metadata, data);

        Ok(())
    }

    fn verify(&self) -> Result<()> {
        println!("Rfq.verify() invoked");
        Ok(())
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
    pub payment_details: String, // 🚧 Map<string, JsonNode>
    pub amount: String,
}

#[derive(Clone)]
pub struct CreateSelectedPayoutMethod {
    pub kind: String,
    pub payment_details: String, // 🚧 Map<string, JsonNode>
}

#[derive(Clone, Serialize)]
pub struct RfqData {
    pub offering_id: String,
    pub payin: SelectedPayinMethod,
    pub payout: SelectedPayoutMethod,
    pub claims_hash: Option<String>,
}

#[derive(Clone, Serialize)]
pub struct SelectedPayinMethod {
    pub kind: String,
    pub payment_details_hash: Option<String>,
    pub amount: String,
}

#[derive(Clone, Serialize)]
pub struct SelectedPayoutMethod {
    pub kind: String,
    pub payment_details_hash: Option<String>,
}

#[derive(Clone, Serialize)]
pub struct RfqPrivateData {
    pub salt: String,
    pub payin: Option<PrivatePaymentDetails>,
    pub payout: Option<PrivatePaymentDetails>,
    pub claims: Option<Vec<String>>,
}

#[derive(Clone, Serialize)]
pub struct PrivatePaymentDetails {
    pub payment_details: String, // 🚧 Map<string, JsonNode>
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

        let mut rfq = Rfq::new(
            "did:test:pfi".to_string(),
            did_jwk.did.uri.clone(),
            CreateRfqData {
                offering_id: "offering_123".to_string(),
                payin: CreateSelectedPayinMethod {
                    kind: "BTC".to_string(),
                    payment_details: "tmp-payment-details".to_string(),
                    amount: "101".to_string(),
                },
                payout: CreateSelectedPayoutMethod {
                    kind: "BTC".to_string(),
                    payment_details: "tmp-payment-details".to_string(),
                },
                claims: vec!["some-claim".to_string()],
            },
            "1.0".to_string(),
            None,
        ).unwrap();

        rfq.sign(bearer_did).unwrap();

        assert_ne!(String::default(), rfq.signature)
    }
}
