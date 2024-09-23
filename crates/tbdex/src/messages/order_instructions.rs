use super::{MessageKind, MessageMetadata, Result};
use crate::{
    json::{FromJson, ToJson},
    json_schemas::generated::{MESSAGE_JSON_SCHEMA, ORDER_INSTRUCTIONS_DATA_JSON_SCHEMA},
    DEFAULT_PROTOCOL_VERSION,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use web5::dids::bearer_did::BearerDid;

/// Represents an Order Instructions message in the tbDEX protocol.
///
/// An Order Instructions message is sent from a PFI to Alice, providing
/// detailed instructions on how to make a payin or receive a payout.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct OrderInstructions {
    /// Metadata about the message, including sender, recipient, and protocol information.
    pub metadata: MessageMetadata,

    /// The public data part of the Order Instructions, including payment instructions for payin and payout.
    pub data: OrderInstructionsData,

    /// The signature verifying the authenticity and integrity of the Order Instructions message.
    pub signature: String,
}

impl ToJson for OrderInstructions {}
impl FromJson for OrderInstructions {}

impl OrderInstructions {
    /// Creates a new Order Instructions message.
    ///
    /// # Arguments
    ///
    /// * `to` - The DID of the recipient (Alice).
    /// * `from` - The DID of the sender (the PFI).
    /// * `exchange_id` - The exchange ID shared between Alice and the PFI.
    /// * `data` - The data containing payment instructions for payin and payout.
    /// * `protocol` - Optional protocol version; defaults to the current version if not provided.
    /// * `external_id` - Optional external ID for additional identification.
    ///
    /// # Returns
    ///
    /// A new instance of `OrderInstructions` containing the metadata, data, and an empty signature.
    pub fn create(
        to: &str,
        from: &str,
        exchange_id: &str,
        data: &OrderInstructionsData,
        protocol: Option<String>,
        external_id: Option<String>,
    ) -> Result<Self> {
        let metadata = MessageMetadata {
            from: from.to_string(),
            to: to.to_string(),
            kind: MessageKind::OrderInstructions,
            id: MessageKind::OrderInstructions.typesafe_id()?,
            exchange_id: exchange_id.to_string(),
            external_id,
            protocol: protocol.unwrap_or_else(|| DEFAULT_PROTOCOL_VERSION.to_string()),
            created_at: Utc::now().to_rfc3339(),
        };

        let order_instructions = Self {
            metadata: metadata.clone(),
            data: data.clone(),
            signature: String::default(),
        };

        Ok(order_instructions)
    }

    /// Signs the Order Instructions message using the provided Bearer DID.
    ///
    /// # Arguments
    ///
    /// * `bearer_did` - The DID to sign the Order Instructions message.
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

    /// Verifies the validity of the Order Instructions message.
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
        crate::json_schemas::validate_from_str(ORDER_INSTRUCTIONS_DATA_JSON_SCHEMA, &self.data)?;

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

/// Represents the data for Order Instructions in the tbDEX protocol.
///
/// This includes payment instructions for both payin and payout operations.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OrderInstructionsData {
    /// The payment instruction for the payin (e.g., how Alice should pay the PFI).
    pub payin: PaymentInstruction,

    /// The payment instruction for the payout (e.g., how Alice should receive the payout).
    pub payout: PaymentInstruction,
}

/// Represents the payment instruction for either payin or payout in an Order Instructions message.
///
/// This includes optional fields such as a link or additional instructions.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PaymentInstruction {
    /// Optional link for Alice to make the payment or receive the payout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,

    /// Optional instructions for Alice on how to make the payment or receive the payout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instruction: Option<String>,
}
