use super::{MessageKind, MessageMetadata, Result};
use crate::{
    json::{FromJson, ToJson},
    json_schemas::generated::{MESSAGE_JSON_SCHEMA, ORDER_STATUS_DATA_JSON_SCHEMA},
    DEFAULT_PROTOCOL_VERSION,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use web5::dids::bearer_did::BearerDid;

/// Represents an Order Status message in the tbDEX protocol.
///
/// An Order Status message is sent from a PFI to Alice to communicate
/// the current status of an ongoing order or exchange process.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct OrderStatus {
    /// Metadata about the message, including sender, recipient, and protocol information.
    pub metadata: MessageMetadata,

    /// The public data part of the Order Status, such as the current status of the order.
    pub data: OrderStatusData,

    /// The signature verifying the authenticity and integrity of the Order Status message.
    pub signature: String,
}

impl ToJson for OrderStatus {}
impl FromJson for OrderStatus {}

impl OrderStatus {
    /// Creates a new Order Status message.
    ///
    /// # Arguments
    ///
    /// * `to` - The DID of the recipient (Alice).
    /// * `from` - The DID of the sender (the PFI).
    /// * `exchange_id` - The exchange ID shared between Alice and the PFI.
    /// * `data` - The data containing the current status of the order.
    /// * `protocol` - Optional protocol version; defaults to the current version if not provided.
    /// * `external_id` - Optional external ID for additional identification.
    ///
    /// # Returns
    ///
    /// A new instance of `OrderStatus` containing the metadata, data, and an empty signature.
    pub fn create(
        to: &str,
        from: &str,
        exchange_id: &str,
        data: &OrderStatusData,
        protocol: Option<String>,
        external_id: Option<String>,
    ) -> Result<Self> {
        let metadata = MessageMetadata {
            from: from.to_string(),
            to: to.to_string(),
            kind: MessageKind::OrderStatus,
            id: MessageKind::OrderStatus.typesafe_id()?,
            exchange_id: exchange_id.to_string(),
            external_id,
            protocol: protocol.unwrap_or_else(|| DEFAULT_PROTOCOL_VERSION.to_string()),
            created_at: Utc::now().to_rfc3339(),
        };

        let order_status = Self {
            metadata: metadata.clone(),
            data: data.clone(),
            signature: String::default(),
        };

        Ok(order_status)
    }

    /// Signs the Order Status message using the provided Bearer DID.
    ///
    /// # Arguments
    ///
    /// * `bearer_did` - The DID to sign the Order Status message.
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

    /// Verifies the validity of the Order Status message.
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
        crate::json_schemas::validate_from_str(ORDER_STATUS_DATA_JSON_SCHEMA, &self.data)?;

        // verify signature
        crate::signature::verify(
            &self.metadata.from,
            &serde_json::to_value(self.metadata.clone())?,
            &serde_json::to_value(self.data.clone())?,
            &self.signature,
        )?;

        Ok(())
    }
}

/// Represents the data for an Order Status message in the tbDEX protocol.
///
/// This includes the current status of the order and any optional details
/// providing additional information about the status.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OrderStatusData {
    /// The current status of the order (e.g., PayinPending, PayinSettled).
    pub status: Status,

    /// Optional additional details about the current status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

/// Represents the possible statuses for an order in the tbDEX protocol.
///
/// Each status indicates a specific stage in the lifecycle of an order.
#[derive(Debug, Default, Deserialize, PartialEq, Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    /// Indicates that the PFI is awaiting payment from Alice.
    #[default]
    PayinPending,

    /// Indicates that payment from Alice has been initiated.
    PayinInitiated,

    /// Indicates that payment from Alice has been successfully settled into the PFI's account.
    PayinSettled,

    /// Indicates that payment from Alice has failed.
    PayinFailed,

    /// Indicates that payment from Alice was not received before the quote expired.
    PayinExpired,

    /// Indicates that the payout to Alice is pending further processing.
    PayoutPending,

    /// Indicates that the payout to Alice has been initiated.
    PayoutInitiated,

    /// Indicates that the payout to Alice has been successfully settled.
    PayoutSettled,

    /// Indicates that the payout to Alice has failed.
    PayoutFailed,

    /// Indicates that a refund of Alice's payin is pending further processing.
    RefundPending,

    /// Indicates that the refund of Alice's payin has been initiated.
    RefundInitiated,

    /// Indicates that the refund of Alice's payin has been successfully settled.
    RefundSettled,

    /// Indicates that the refund of Alice's payin has failed.
    RefundFailed,
}

#[cfg(test)]
mod tbdex_test_vectors_protocol {
    use super::*;
    use std::fs;

    #[derive(Debug, serde::Deserialize)]
    pub struct TestVector {
        pub input: String,
        pub output: OrderStatus,
    }

    #[test]
    fn parse_orderstatus() {
        let path = "../../tbdex/hosted/test-vectors/protocol/vectors/parse-orderstatus.json";
        let test_vector_json: String = fs::read_to_string(path).unwrap();

        let test_vector: TestVector = serde_json::from_str(&test_vector_json).unwrap();
        let parsed_order_status: OrderStatus =
            OrderStatus::from_json_string(&test_vector.input).unwrap();

        assert_eq!(test_vector.output, parsed_order_status);
    }
}
