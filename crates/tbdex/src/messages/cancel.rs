use super::{MessageKind, MessageMetadata, Result};
use crate::{
    json::{FromJson, ToJson},
    json_schemas::generated::{CANCEL_DATA_JSON_SCHEMA, MESSAGE_JSON_SCHEMA},
    DEFAULT_PROTOCOL_VERSION,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use web5::dids::bearer_did::BearerDid;

/// Represents a Cancel message in the tbDEX protocol.
///
/// A Cancel message is sent by Alice to a PFI to terminate an exchange that has not been completed,
/// typically when Alice decides to back out of the transaction or request a refund.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Cancel {
    /// Metadata about the message, including sender, recipient, and protocol information.
    pub metadata: MessageMetadata,

    /// The public data part of the Cancel message, which includes the reason for cancellation.
    pub data: CancelData,

    /// The signature verifying the authenticity and integrity of the Cancel message.
    pub signature: String,
}

impl ToJson for Cancel {}
impl FromJson for Cancel {}

impl Cancel {
    /// Creates a new Cancel message.
    ///
    /// # Arguments
    ///
    /// * `to` - The DID of the recipient (the PFI).
    /// * `from` - The DID of the sender (Alice).
    /// * `exchange_id` - The exchange ID shared between Alice and the PFI.
    /// * `data` - The data containing the reason for canceling the exchange.
    /// * `protocol` - Optional protocol version; defaults to the current version if not provided.
    /// * `external_id` - Optional external ID for additional identification.
    ///
    /// # Returns
    ///
    /// A new instance of `Cancel` containing the metadata, data, and an empty signature.
    pub fn create(
        to: &str,
        from: &str,
        exchange_id: &str,
        data: &CancelData,
        protocol: Option<String>,
        external_id: Option<String>,
    ) -> Result<Self> {
        let metadata = MessageMetadata {
            from: from.to_string(),
            to: to.to_string(),
            kind: MessageKind::Cancel,
            id: MessageKind::Cancel.typesafe_id()?,
            exchange_id: exchange_id.to_string(),
            external_id,
            protocol: protocol.unwrap_or_else(|| DEFAULT_PROTOCOL_VERSION.to_string()),
            created_at: Utc::now().to_rfc3339(),
        };

        let cancel = Self {
            metadata: metadata.clone(),
            data: data.clone(),
            signature: String::default(),
        };

        Ok(cancel)
    }

    /// Signs the Cancel message using the provided Bearer DID.
    ///
    /// # Arguments
    ///
    /// * `bearer_did` - The DID to sign the Cancel message.
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

    /// Verifies the validity of the Cancel message.
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
        crate::json_schemas::validate_from_str(CANCEL_DATA_JSON_SCHEMA, &self.data)?;

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

/// Represents the data for a Cancel message in the tbDEX protocol.
///
/// This includes an optional reason explaining why the exchange is being canceled.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct CancelData {
    /// An optional human-readable reason for canceling the exchange.
    pub reason: Option<String>,
}

#[cfg(test)]
mod tbdex_test_vectors_protocol {
    use super::*;
    use std::fs;

    #[derive(Debug, serde::Deserialize)]
    pub struct TestVector {
        pub input: String,
        pub output: Cancel,
    }

    #[test]
    fn parse_cancel() {
        let path = "../../tbdex/hosted/test-vectors/protocol/vectors/parse-cancel.json";
        let test_vector_json: String = fs::read_to_string(path).unwrap();

        let test_vector: TestVector = serde_json::from_str(&test_vector_json).unwrap();
        let parsed_cancel: Cancel = Cancel::from_json_string(&test_vector.input).unwrap();

        assert_eq!(test_vector.output, parsed_cancel);
    }
}
