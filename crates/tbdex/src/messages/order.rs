use super::{MessageKind, MessageMetadata, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use web5::apid::dids::bearer_did::BearerDid;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Order {
    pub metadata: MessageMetadata,
    pub data: OrderData,
    pub signature: String,
}

impl Order {
    pub fn new(
        bearer_did: &BearerDid,
        to: &str,
        from: &str,
        exchange_id: &str,
        protocol: &str,
        external_id: Option<String>,
    ) -> Result<Self> {
        let metadata = MessageMetadata {
            from: from.to_string(),
            to: to.to_string(),
            kind: MessageKind::Order,
            id: MessageKind::Order.typesafe_id()?,
            exchange_id: exchange_id.to_string(),
            external_id,
            protocol: protocol.to_string(),
            created_at: Utc::now().to_rfc3339(),
        };

        let data = OrderData {};

        Ok(Self {
            metadata: metadata.clone(),
            data: data.clone(),
            signature: crate::signature::sign(
                bearer_did,
                &serde_json::to_value(metadata)?,
                &serde_json::to_value(&data)?,
            )?,
        })
    }

    pub fn from_json_string(json: &str) -> Result<Self> {
        let order = serde_json::from_str::<Self>(json)?;
        order.verify()?;
        Ok(order)
    }

    pub fn verify(&self) -> Result<()> {
        Ok(crate::signature::verify(
            &self.metadata.from,
            &serde_json::to_value(self.metadata.clone())?,
            &serde_json::to_value(&OrderData {})?,
            &self.signature,
        )?)
    }

    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self)?)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct OrderData {}
