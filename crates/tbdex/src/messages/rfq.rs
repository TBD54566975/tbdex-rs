use super::{MessageKind, MessageMetadata, Result};
use crate::{messages::MessageError, resources::offering::Offering};
use base64::{engine::general_purpose, Engine as _};
use chrono::Utc;
use rand::{rngs::OsRng, RngCore};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use web5::apid::dids::bearer_did::BearerDid;

#[derive(Clone, Serialize, Default, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Rfq {
    pub metadata: MessageMetadata,
    pub data: RfqData,
    pub private_data: RfqPrivateData,
    pub signature: String,
}

impl Rfq {
    pub fn new(
        bearer_did: &BearerDid,
        to: &str,
        from: &str,
        create_rfq_data: &CreateRfqData,
        protocol: &str,
        external_id: Option<String>,
    ) -> Result<Self> {
        let id = MessageKind::Rfq.typesafe_id()?;

        let metadata = MessageMetadata {
            from: from.to_string(),
            to: to.to_string(),
            kind: MessageKind::Rfq,
            id: id.clone(),
            exchange_id: id.clone(),
            external_id,
            protocol: protocol.to_string(),
            created_at: Utc::now().to_rfc3339(),
        };

        let (data, private_data) = hash_private_data(create_rfq_data);

        Ok(Self {
            metadata: metadata.clone(),
            data: data.clone(),
            private_data,
            signature: crate::signature::sign(
                bearer_did,
                &serde_json::to_value(metadata)?,
                &serde_json::to_value(data)?,
            )?,
        })
    }

    pub fn from_json_string(json: &str) -> Result<Self> {
        let rfq = serde_json::from_str::<Self>(json)?;
        rfq.verify()?;
        Ok(rfq)
    }

    pub fn verify(&self) -> Result<()> {
        Ok(crate::signature::verify(
            &self.metadata.from,
            &serde_json::to_value(self.metadata.clone())?,
            &serde_json::to_value(self.data.clone())?,
            &self.signature,
        )?)
    }

    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self)?)
    }

    pub fn verify_offering_requirements(&self, offering: &Offering) -> Result<bool> {
        if offering.metadata.protocol != self.metadata.protocol {
            return Err(MessageError::OfferingVerification(format!(
                "offering has protocol version {} but rfq has protocol version {}",
                offering.metadata.protocol, self.metadata.protocol
            )));
        }

        if offering.metadata.id != self.data.offering_id {
            return Err(MessageError::OfferingVerification(format!(
                "offering id is {} but rfq has offering id {}",
                offering.metadata.id, self.data.offering_id
            )));
        }

        let payin_amount = self.data.payin.amount.parse::<f64>().map_err(|_| {
            MessageError::OfferingVerification(format!(
                "rfq payin amount invalid decimal string {}",
                self.data.payin.amount
            ))
        })?;

        if let Some(max_amount) = offering.data.payin.max.as_ref() {
            let max_amount = max_amount.parse::<f64>().map_err(|_| {
                MessageError::OfferingVerification(format!(
                    "offering max amount invalid decimal string {}",
                    max_amount
                ))
            })?;

            if payin_amount > max_amount {
                return Err(MessageError::OfferingVerification(format!(
                    "rfq payin of {} is larger than max offering amount of {}",
                    payin_amount, max_amount
                )));
            }
        }

        if let Some(min_amount) = offering.data.payin.min.as_ref() {
            let min_amount = min_amount.parse::<f64>().map_err(|_| {
                MessageError::OfferingVerification(format!(
                    "offering min amount invalid decimal string {}",
                    min_amount
                ))
            })?;

            if payin_amount < min_amount {
                return Err(MessageError::OfferingVerification(format!(
                    "rfq payin of {} is smaller than min offering amount of {}",
                    payin_amount, min_amount
                )));
            }
        }

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
    pub payment_details: Option<serde_json::Value>,
    pub amount: String,
}

#[derive(Clone)]
pub struct CreateSelectedPayoutMethod {
    pub kind: String,
    pub payment_details: Option<serde_json::Value>,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RfqData {
    pub offering_id: String,
    pub payin: SelectedPayinMethod,
    pub payout: SelectedPayoutMethod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claims_hash: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SelectedPayinMethod {
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_details_hash: Option<String>,
    pub amount: String,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SelectedPayoutMethod {
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_details_hash: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RfqPrivateData {
    pub salt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payin: Option<PrivatePaymentDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout: Option<PrivatePaymentDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claims: Option<Vec<String>>,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PrivatePaymentDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_details: Option<serde_json::Value>,
}

fn hash_private_data(create_rfq_data: &CreateRfqData) -> (RfqData, RfqPrivateData) {
    let salt = generate_random_salt();

    let payin_payment_details_hash = create_rfq_data
        .payin
        .payment_details
        .as_ref()
        .map(|pd| digest_private_data(&salt, pd));
    let payout_payment_details_hash = create_rfq_data
        .payout
        .payment_details
        .as_ref()
        .map(|pd| digest_private_data(&salt, pd));
    let claims_hash = if create_rfq_data.claims.is_empty() {
        None
    } else {
        Some(digest_private_data(&salt, &create_rfq_data.claims))
    };

    let hashed_rfq_data = RfqData {
        offering_id: create_rfq_data.offering_id.clone(),
        payin: SelectedPayinMethod {
            kind: create_rfq_data.payin.kind.clone(),
            payment_details_hash: payin_payment_details_hash,
            amount: create_rfq_data.payin.amount.clone(),
        },
        payout: SelectedPayoutMethod {
            kind: create_rfq_data.payout.kind.clone(),
            payment_details_hash: payout_payment_details_hash,
        },
        claims_hash,
    };

    let private_rfq_data =
        RfqPrivateData {
            salt: salt.clone(),
            payin: create_rfq_data
                .payin
                .payment_details
                .as_ref()
                .map(|pd| PrivatePaymentDetails {
                    payment_details: Some(pd.clone()),
                }),
            payout: create_rfq_data.payout.payment_details.as_ref().map(|pd| {
                PrivatePaymentDetails {
                    payment_details: Some(pd.clone()),
                }
            }),
            claims: if !create_rfq_data.claims.is_empty() {
                Some(create_rfq_data.claims.clone())
            } else {
                None
            },
        };

    (hashed_rfq_data, private_rfq_data)
}

fn generate_random_salt() -> String {
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);
    general_purpose::URL_SAFE_NO_PAD.encode(salt)
}

fn digest_private_data<T: Serialize>(salt: &str, value: &T) -> String {
    let digestible = serde_json::json!([salt, value]);
    let serialized = serde_json::to_string(&digestible).unwrap(); // 🚧 unwrap!

    let mut hasher = Sha256::new();
    hasher.update(serialized.as_bytes());
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
            &bearer_did,
            "did:test:pfi",
            &did_jwk.did.uri,
            &CreateRfqData {
                offering_id: "offering_123".to_string(),
                payin: CreateSelectedPayinMethod {
                    kind: "BTC".to_string(),
                    payment_details: Some(serde_json::json!({"tmp": "payment-details"})),
                    amount: "101".to_string(),
                },
                payout: CreateSelectedPayoutMethod {
                    kind: "BTC".to_string(),
                    payment_details: Some(serde_json::json!({"tmp": "payment-details"})),
                },
                claims: vec!["some-claim".to_string()],
            },
            "1.0",
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
