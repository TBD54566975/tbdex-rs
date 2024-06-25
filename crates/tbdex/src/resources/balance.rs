use super::{ResourceKind, ResourceMetadata, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use web5::apid::dids::bearer_did::BearerDid;

#[derive(Clone, Serialize, Deserialize)]
pub struct Balance {
    pub metadata: ResourceMetadata,
    pub data: BalanceData,
    pub signature: String,
}

impl Balance {
    pub fn new(
        bearer_did: &BearerDid,
        from: &str,
        data: &BalanceData,
        protocol: &str,
    ) -> Result<Self> {
        let now = Utc::now().to_rfc3339();

        let metadata = ResourceMetadata {
            kind: ResourceKind::Balance,
            from: from.to_string(),
            id: ResourceKind::Balance.typesafe_id()?,
            protocol: protocol.to_string(),
            created_at: now.clone(),
            updated_at: Some(now),
        };

        Ok(Self {
            metadata: metadata.clone(),
            data: data.clone(),
            signature: crate::signature::sign(
                bearer_did,
                &serde_json::to_value(metadata)?,
                &serde_json::to_value(data)?,
            )?,
        })
    }

    pub fn from_json_string(json: &str) -> Result<Self> {
        let balance = serde_json::from_str::<Self>(json)?;
        balance.verify()?;
        Ok(balance)
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
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceData {
    pub currency_code: String,
    pub available: String,
}
