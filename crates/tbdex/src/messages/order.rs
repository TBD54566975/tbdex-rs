use crate::jose::Signer;

use super::{MessageKind, MessageMetadata, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use web5::apid::dids::bearer_did::BearerDid;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Order {
    pub metadata: MessageMetadata,
    pub signature: String,
}

impl Order {
    pub fn new(
        bearer_did: BearerDid,
        to: String,
        from: String,
        exchange_id: String,
        protocol: String,
        external_id: Option<String>,
    ) -> Result<Self> {
        let metadata = MessageMetadata {
            from,
            to,
            kind: MessageKind::Order,
            id: MessageKind::Order.typesafe_id()?,
            exchange_id,
            external_id,
            protocol,
            created_at: Utc::now().to_rfc3339(),
        };

        let key_id = bearer_did.document.verification_method[0].id.clone();
        let web5_signer = bearer_did.get_signer(key_id.clone())?;
        let jose_signer = Signer {
            kid: key_id,
            web5_signer,
        };

        Ok(Self {
            metadata: metadata.clone(),
            signature: crate::signature::sign(
                jose_signer,
                serde_json::to_value(metadata)?,
                serde_json::to_value(&OrderData {})?,
            )?,
        })
    }

    pub fn from_json_string(json: &str) -> Result<Self> {
        let order = serde_json::from_str::<Self>(json)?;

        crate::signature::verify(
            &order.metadata.from,
            serde_json::to_value(order.metadata.clone())?,
            serde_json::to_value(&OrderData {})?,
            order.signature.clone(),
        )?;

        Ok(order)
    }

    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self)?)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct OrderData;
