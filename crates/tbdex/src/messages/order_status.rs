use crate::jose::Signer;

use super::{MessageKind, MessageMetadata, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use web5::apid::dids::bearer_did::BearerDid;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct OrderStatus {
    pub metadata: MessageMetadata,
    pub data: OrderStatusData,
    pub signature: String,
}

impl OrderStatus {
    pub fn new(
        bearer_did: BearerDid,
        to: String,
        from: String,
        exchange_id: String,
        data: OrderStatusData,
        protocol: String,
        external_id: Option<String>,
    ) -> Result<Self> {
        let metadata = MessageMetadata {
            from,
            to,
            kind: MessageKind::OrderStatus,
            id: MessageKind::OrderStatus.typesafe_id()?,
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
            data: data.clone(),
            signature: crate::signature::sign(
                jose_signer,
                serde_json::to_value(metadata)?,
                serde_json::to_value(data)?,
            )?,
        })
    }

    pub fn from_json_string(json: &str) -> Result<Self> {
        let order_status = serde_json::from_str::<Self>(json)?;

        crate::signature::verify(
            &order_status.metadata.from,
            serde_json::to_value(order_status.metadata.clone())?,
            serde_json::to_value(order_status.data.clone())?,
            order_status.signature.clone(),
        )?;

        Ok(order_status)
    }

    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self)?)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct OrderStatusData {
    pub order_status: String,
}
