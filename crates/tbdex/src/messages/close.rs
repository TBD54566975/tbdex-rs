use super::{MessageKind, MessageMetadata, Result};
use crate::json_schemas::generated::{CLOSE_DATA_JSON_SCHEMA, MESSAGE_JSON_SCHEMA};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use web5::dids::bearer_did::BearerDid;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Close {
    pub metadata: MessageMetadata,
    pub data: CloseData,
    pub signature: String,
}

impl Close {
    pub fn new(
        bearer_did: &BearerDid,
        to: &str,
        from: &str,
        exchange_id: &str,
        data: &CloseData,
        protocol: &str,
        external_id: Option<String>,
    ) -> Result<Self> {
        let metadata = MessageMetadata {
            from: from.to_string(),
            to: to.to_string(),
            kind: MessageKind::Close,
            id: MessageKind::Close.typesafe_id()?,
            exchange_id: exchange_id.to_string(),
            external_id,
            protocol: protocol.to_string(),
            created_at: Utc::now().to_rfc3339(),
        };

        let close = Self {
            metadata: metadata.clone(),
            data: data.clone(),
            signature: crate::signature::sign(
                bearer_did,
                &serde_json::to_value(metadata)?,
                &serde_json::to_value(data)?,
            )?,
        };

        close.verify()?;

        Ok(close)
    }

    pub fn from_json_string(json: &str) -> Result<Self> {
        let close = serde_json::from_str::<Self>(json)?;
        close.verify()?;
        Ok(close)
    }

    pub fn verify(&self) -> Result<()> {
        // verify resource json schema
        crate::json_schemas::validate_from_str(MESSAGE_JSON_SCHEMA, self)?;

        // verify data json schema
        crate::json_schemas::validate_from_str(CLOSE_DATA_JSON_SCHEMA, &self.data)?;

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

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct CloseData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}
