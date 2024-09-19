use super::{MessageKind, MessageMetadata, Result};
use crate::{
    json::{FromJson, ToJson},
    json_schemas::generated::{MESSAGE_JSON_SCHEMA, ORDER_DATA_JSON_SCHEMA},
    DEFAULT_PROTOCOL_VERSION,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use web5::dids::bearer_did::BearerDid;

/// Represents an Order message in the tbDEX protocol.
///
/// An Order message is sent by Alice to a PFI to execute a transaction based on a previously provided quote.
/// It includes metadata about the message and the signature to ensure its integrity and authenticity.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Order {
    /// Metadata about the message, including sender, recipient, and protocol information.
    pub metadata: MessageMetadata,

    /// The public data part of the Order.
    pub data: OrderData,

    /// The signature verifying the authenticity and integrity of the Order message.
    pub signature: String,
}

impl ToJson for Order {}
impl FromJson for Order {}

impl Order {
    /// Creates a new Order message.
    ///
    /// # Arguments
    ///
    /// * `to` - The DID of the recipient (the PFI).
    /// * `from` - The DID of the sender (Alice).
    /// * `exchange_id` - The exchange ID shared between Alice and the PFI.
    /// * `protocol` - Optional protocol version; defaults to the current version if not provided.
    /// * `external_id` - Optional external ID for additional identification.
    ///
    /// # Returns
    ///
    /// A new instance of `Order` containing the metadata, data, and an empty signature.
    pub fn create(
        to: &str,
        from: &str,
        exchange_id: &str,
        protocol: Option<String>,
        external_id: Option<String>,
    ) -> Result<Self> {
        let metadata = MessageMetadata {
            from: from.to_string(),
            to: to.to_string(),
            kind: MessageKind::Order,
            id: MessageKind::Order.typesafe_id()?,
            exchange_id: exchange_id.to_string(),
            external_id,
            protocol: protocol.unwrap_or_else(|| DEFAULT_PROTOCOL_VERSION.to_string()),
            created_at: Utc::now().to_rfc3339(),
        };

        let data = OrderData {};

        let order = Self {
            metadata: metadata.clone(),
            data: data.clone(),
            signature: String::default(),
        };

        Ok(order)
    }

    /// Signs the Order message using the provided Bearer DID.
    ///
    /// # Arguments
    ///
    /// * `bearer_did` - The DID to sign the Order message.
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

    /// Verifies the validity of the Order message.
    ///
    /// This method ensures that the message adheres to its JSON schema
    /// and verifies the signature to ensure authenticity and integrity.
    ///
    /// # Returns
    ///
    /// An empty result if verification succeeds, or an error if verification fails.
    pub fn verify(&self) -> Result<()> {
        // verify resource json schema
        crate::json_schemas::validate_from_str(MESSAGE_JSON_SCHEMA, self)?;

        // verify data json schema
        crate::json_schemas::validate_from_str(ORDER_DATA_JSON_SCHEMA, &self.data)?;

        // verify signature
        crate::signature::verify(
            &self.metadata.from,
            &serde_json::to_value(self.metadata.clone())?,
            &serde_json::to_value(&OrderData {})?,
            &self.signature,
        )?;

        Ok(())
    }
}

/// Represents the data for an Order in the tbDEX protocol.
///
/// Currently, it is an empty structure, but can be expanded in the future as needed.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct OrderData {}

#[cfg(test)]
mod tbdex_test_vectors_protocol {
    use super::*;
    use std::fs;

    #[derive(Debug, serde::Deserialize)]
    pub struct TestVector {
        pub input: String,
        pub output: Order,
    }

    #[test]
    fn parse_order() {
        let path = "../../tbdex/hosted/test-vectors/protocol/vectors/parse-order.json";
        let test_vector_json: String = fs::read_to_string(path).unwrap();

        let test_vector: TestVector = serde_json::from_str(&test_vector_json).unwrap();
        let parsed_order: Order = Order::from_json_string(&test_vector.input).unwrap();

        assert_eq!(test_vector.output, parsed_order);
    }
}
