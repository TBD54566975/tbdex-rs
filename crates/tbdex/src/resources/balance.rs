use super::{ResourceKind, ResourceMetadata, Result};
use crate::{
    json::{FromJson, ToJson},
    json_schemas::generated::{BALANCE_DATA_JSON_SCHEMA, RESOURCE_JSON_SCHEMA},
    DEFAULT_PROTOCOL_VERSION,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use web5::dids::bearer_did::BearerDid;

/// Represents a Balance resource in the tbDEX protocol.
///
/// A Balance resource is used to communicate the amount of a particular currency
/// held by a PFI on behalf of a customer. It includes metadata, balance data, and a signature.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Balance {
    /// Metadata about the resource, including sender, type of resource, and protocol information.
    pub metadata: ResourceMetadata,

    /// The public data part of the Balance resource, including the currency and available balance.
    pub data: BalanceData,

    /// The signature verifying the authenticity and integrity of the Balance resource.
    pub signature: String,
}

impl ToJson for Balance {}
impl FromJson for Balance {}

impl Balance {
    /// Creates a new Balance resource.
    ///
    /// # Arguments
    ///
    /// * `from` - The DID of the sender (the PFI).
    /// * `data` - The data containing the currency code and available balance.
    /// * `protocol` - Optional protocol version; defaults to the current version if not provided.
    ///
    /// # Returns
    ///
    /// A new instance of `Balance` containing the metadata, data, and an empty signature.
    pub fn create(from: &str, data: &BalanceData, protocol: Option<String>) -> Result<Self> {
        let now = Utc::now().to_rfc3339();

        let metadata = ResourceMetadata {
            kind: ResourceKind::Balance,
            from: from.to_string(),
            id: ResourceKind::Balance.typesafe_id()?,
            protocol: protocol.unwrap_or_else(|| DEFAULT_PROTOCOL_VERSION.to_string()),
            created_at: now.clone(),
            updated_at: Some(now),
        };

        let balance = Self {
            metadata: metadata.clone(),
            data: data.clone(),
            signature: String::default(),
        };

        Ok(balance)
    }

    /// Signs the Balance resource using the provided Bearer DID.
    ///
    /// # Arguments
    ///
    /// * `bearer_did` - The DID to sign the Balance resource.
    ///
    /// # Returns
    ///
    /// An empty result, or an error if the signing process fails.
    pub fn sign(&mut self, bearer_did: &BearerDid) -> Result<()> {
        self.signature = crate::signature::sign(
            bearer_did,
            &serde_json::to_value(&self.metadata)?,
            &serde_json::to_value(&self.data)?,
        )?;
        Ok(())
    }

    /// Verifies the validity of the Balance resource.
    ///
    /// This method ensures that the resource adheres to its JSON schema
    /// and verifies the signature to ensure authenticity and integrity.
    ///
    /// # Returns
    ///
    /// An empty result if verification succeeds, or an error if verification fails.
    pub async fn verify(&self) -> Result<()> {
        // verify resource json schema
        crate::json_schemas::validate_from_str(RESOURCE_JSON_SCHEMA, self)?;

        // verify data json schema
        crate::json_schemas::validate_from_str(BALANCE_DATA_JSON_SCHEMA, &self.data)?;

        // verify signature
        crate::signature::verify(
            &serde_json::to_value(self.metadata.clone())?,
            &serde_json::to_value(self.data.clone())?,
            &self.signature,
        )
        .await?;

        Ok(())
    }
}

/// Represents the data for a Balance resource in the tbDEX protocol.
///
/// This includes the currency code (ISO 4217 format) and the available balance.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BalanceData {
    /// The currency code (ISO 4217 format) for the balance.
    pub currency_code: String,

    /// The available amount of the currency.
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
