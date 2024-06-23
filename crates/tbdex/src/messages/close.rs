use super::{MessageKind, MessageMetadata, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use web5::apid::dids::bearer_did::BearerDid;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Close {
    pub metadata: MessageMetadata,
    pub data: CloseData,
    pub signature: String,
}

impl Close {
    pub fn new(
        bearer_did: BearerDid,
        to: String,
        from: String,
        exchange_id: String,
        data: CloseData,
        protocol: String,
        external_id: Option<String>,
    ) -> Result<Self> {
        let metadata = MessageMetadata {
            from,
            to,
            kind: MessageKind::Close,
            id: MessageKind::Close.typesafe_id()?,
            exchange_id,
            external_id,
            protocol,
            created_at: Utc::now().to_rfc3339(),
        };

        Ok(Self {
            metadata: metadata.clone(),
            data: data.clone(),
            signature: crate::signature::sign(
                bearer_did,
                serde_json::to_value(metadata)?,
                serde_json::to_value(data)?,
            )?,
        })
    }

    pub fn from_json_string(json: &str) -> Result<Self> {
        let close = serde_json::from_str::<Self>(json)?;

        crate::signature::verify(
            &close.metadata.from,
            serde_json::to_value(close.metadata.clone())?,
            serde_json::to_value(close.data.clone())?,
            close.signature.clone(),
        )?;

        Ok(close)
    }

    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self)?)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct CloseData {
    pub reason: Option<String>,
    pub success: Option<bool>,
}
