use crate::signer::sign;

use super::{Resource, ResourceKind, ResourceMetadata, Result};
use chrono::Utc;
use serde::Serialize;
use web5::apid::dids::bearer_did::BearerDid;

#[derive(Clone, Serialize)]
pub struct Balance {
    pub metadata: ResourceMetadata,
    pub data: BalanceData,
    pub signature: String,
}

impl Balance {
    pub fn new(from: String, data: BalanceData, protocol: String) -> Result<Self> {
        let now = Utc::now().to_rfc3339();

        Ok(Self {
            metadata: ResourceMetadata {
                kind: ResourceKind::Balance,
                from,
                id: ResourceKind::Balance.typesafe_id()?,
                protocol,
                created_at: now.clone(),
                updated_at: Some(now),
            },
            data,
            signature: String::default(), // not set until call to sign()
        })
    }
}

impl Resource for Balance {
    fn sign(&mut self, bearer_did: BearerDid) -> Result<()> {
        let metadata = serde_json::to_value(&self.metadata)?;
        let data = serde_json::to_value(&self.data)?;

        self.signature = sign(bearer_did, metadata, data);

        Ok(())
    }

    fn verify(&self) -> Result<()> {
        println!("Offering.verify() invoked");
        Ok(())
    }
}

#[derive(Clone, Serialize)]
pub struct BalanceData {
    pub currency_code: String,
    pub available: String,
}
