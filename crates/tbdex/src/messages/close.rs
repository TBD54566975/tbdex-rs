use super::{MessageKind, MessageMetadata, Result};
use crate::{
    json::{FromJson, ToJson},
    json_schemas::generated::{CLOSE_DATA_JSON_SCHEMA, MESSAGE_JSON_SCHEMA},
    DEFAULT_PROTOCOL_VERSION,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use web5::dids::bearer_did::BearerDid;

/// Represents a Close message in the tbDEX protocol.
///
/// A Close message is sent by a PFI to Alice to signal the termination of an exchange,
/// either because the exchange was completed or because it cannot be fulfilled.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Close {
    /// Metadata about the message, including sender, recipient, and protocol information.
    pub metadata: MessageMetadata,

    /// The public data part of the Close message, which includes the reason for closure.
    pub data: CloseData,

    /// The signature verifying the authenticity and integrity of the Close message.
    pub signature: String,
}

impl ToJson for Close {}
impl FromJson for Close {}

impl Close {
    /// Creates a new Close message.
    ///
    /// # Arguments
    ///
    /// * `to` - The DID of the recipient (Alice).
    /// * `from` - The DID of the sender (the PFI).
    /// * `exchange_id` - The exchange ID shared between Alice and the PFI.
    /// * `data` - The data containing the reason for closing the exchange.
    /// * `protocol` - Optional protocol version; defaults to the current version if not provided.
    /// * `external_id` - Optional external ID for additional identification.
    ///
    /// # Returns
    ///
    /// A new instance of `Close` containing the metadata, data, and an empty signature.
    pub fn create(
        to: &str,
        from: &str,
        exchange_id: &str,
        data: &CloseData,
        protocol: Option<String>,
        external_id: Option<String>,
    ) -> Result<Self> {
        let metadata = MessageMetadata {
            from: from.to_string(),
            to: to.to_string(),
            kind: MessageKind::Close,
            id: MessageKind::Close.typesafe_id()?,
            exchange_id: exchange_id.to_string(),
            external_id,
            protocol: protocol.unwrap_or_else(|| DEFAULT_PROTOCOL_VERSION.to_string()),
            created_at: Utc::now().to_rfc3339(),
        };

        let close = Self {
            metadata: metadata.clone(),
            data: data.clone(),
            signature: String::default(),
        };

        Ok(close)
    }

    /// Signs the Close message using the provided Bearer DID.
    ///
    /// # Arguments
    ///
    /// * `bearer_did` - The DID to sign the Close message.
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

    /// Verifies the validity of the Close message.
    ///
    /// This method ensures that the message adheres to its JSON schema
    /// and verifies the signature to ensure authenticity and integrity.
    ///
    /// # Returns
    ///
    /// An empty result if verification succeeds, or an error if verification fails.
    pub async fn verify(&self) -> Result<()> {
        // verify resource json schema
        crate::json_schemas::validate_from_str(MESSAGE_JSON_SCHEMA, self)?;

        // verify data json schema
        crate::json_schemas::validate_from_str(CLOSE_DATA_JSON_SCHEMA, &self.data)?;

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

/// Represents the data for a Close message in the tbDEX protocol.
///
/// This includes information about the reason for closing the exchange.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct CloseData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

impl FromJson for CloseData {}
impl ToJson for CloseData {}

#[cfg(test)]
mod tbdex_test_vectors_protocol {
    use super::*;
    use std::fs;

    #[derive(Debug, serde::Deserialize)]
    pub struct TestVector {
        pub input: String,
        pub output: Close,
    }

    #[test]
    fn parse_close() {
        let path = "../../tbdex/hosted/test-vectors/protocol/vectors/parse-close.json";
        let test_vector_json: String = fs::read_to_string(path).unwrap();

        let test_vector: TestVector = serde_json::from_str(&test_vector_json).unwrap();
        let parsed_close: Close = Close::from_json_string(&test_vector.input).unwrap();

        assert_eq!(test_vector.output, parsed_close);
    }
}
