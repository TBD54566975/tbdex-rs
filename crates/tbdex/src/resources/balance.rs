use super::{ResourceKind, ResourceMetadata, Result};
use crate::json_schemas::generated::{BALANCE_DATA_JSON_SCHEMA, RESOURCE_JSON_SCHEMA};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use web5::dids::bearer_did::BearerDid;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
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

        let balance = Self {
            metadata: metadata.clone(),
            data: data.clone(),
            signature: crate::signature::sign(
                bearer_did,
                &serde_json::to_value(metadata)?,
                &serde_json::to_value(data)?,
            )?,
        };

        balance.verify()?;

        Ok(balance)
    }

    pub fn from_json_string(json: &str) -> Result<Self> {
        let balance = serde_json::from_str::<Self>(json)?;
        balance.verify()?;
        Ok(balance)
    }

    pub fn verify(&self) -> Result<()> {
        // verify resource json schema
        crate::json_schemas::validate_from_str(RESOURCE_JSON_SCHEMA, self)?;

        // verify data json schema
        crate::json_schemas::validate_from_str(BALANCE_DATA_JSON_SCHEMA, &self.data)?;

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
#[serde(rename_all = "camelCase")]
pub struct BalanceData {
    pub currency_code: String,
    pub available: String,
}

#[cfg(test)]
mod tbdex_test_vectors_protocol {
    use super::*;
    use std::fs;

    #[derive(Debug, serde::Deserialize)]
    pub struct TestVector {
        pub input: String,
        pub output: Balance,
    }

    #[test]
    fn parse_balance() {
        let path = "../../tbdex/hosted/test-vectors/protocol/vectors/parse-balance.json";
        let test_vector_json: String = fs::read_to_string(path).unwrap();

        let test_vector: TestVector = serde_json::from_str(&test_vector_json).unwrap();
        let parsed_balance: Balance = Balance::from_json_string(&test_vector.input).unwrap();

        assert_eq!(test_vector.output, parsed_balance);
    }
}
