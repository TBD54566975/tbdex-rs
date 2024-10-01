use super::{MessageKind, MessageMetadata, Result};
use crate::{
    json::{FromJson, ToJson},
    json_schemas::generated::{MESSAGE_JSON_SCHEMA, QUOTE_DATA_JSON_SCHEMA},
    DEFAULT_PROTOCOL_VERSION,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use web5::dids::bearer_did::BearerDid;

/// Represents a Quote message in the tbDEX protocol.
///
/// A Quote message is sent from a PFI to Alice in response to an RFQ (Request for Quote),
/// detailing the exchange rate, fees, and other details for a potential exchange.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Quote {
    /// Metadata about the message, including sender, recipient, and protocol information.
    pub metadata: MessageMetadata,

    /// The public data part of the Quote, such as exchange rate and payment details.
    pub data: QuoteData,

    /// The signature verifying the authenticity and integrity of the Quote message.
    pub signature: String,
}

impl ToJson for Quote {}
impl FromJson for Quote {}

impl Quote {
    /// Creates a new Quote message.
    ///
    /// # Arguments
    ///
    /// * `to` - The DID of the recipient (Alice).
    /// * `from` - The DID of the sender (the PFI).
    /// * `exchange_id` - The exchange ID shared between Alice and the PFI.
    /// * `data` - The data containing details about the quote, including rates and fees.
    /// * `protocol` - Optional protocol version; defaults to the current version if not provided.
    /// * `external_id` - Optional external ID for additional identification.
    ///
    /// # Returns
    ///
    /// A new instance of `Quote` containing the metadata, data, and an empty signature.
    pub fn create(
        to: &str,
        from: &str,
        exchange_id: &str,
        data: &QuoteData,
        protocol: Option<String>,
        external_id: Option<String>,
    ) -> Result<Self> {
        let metadata = MessageMetadata {
            from: from.to_string(),
            to: to.to_string(),
            kind: MessageKind::Quote,
            id: MessageKind::Quote.typesafe_id()?,
            exchange_id: exchange_id.to_string(),
            external_id,
            protocol: protocol.unwrap_or_else(|| DEFAULT_PROTOCOL_VERSION.to_string()),
            created_at: Utc::now().to_rfc3339(),
        };

        let quote = Self {
            metadata: metadata.clone(),
            data: data.clone(),
            signature: String::default(),
        };

        Ok(quote)
    }

    /// Signs the Quote message using the provided Bearer DID.
    ///
    /// # Arguments
    ///
    /// * `bearer_did` - The DID to sign the Quote message.
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

    /// Verifies the validity of the Quote message.
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
        crate::json_schemas::validate_from_str(QUOTE_DATA_JSON_SCHEMA, &self.data)?;

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

/// Represents the data for a Quote message in the tbDEX protocol.
///
/// This includes the exchange rate, payment details for payin and payout,
/// and an expiration time for the quote.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct QuoteData {
    /// The expiration time of the quote in ISO 8601 format.
    pub expires_at: String,

    /// The exchange rate representing the payout units received per payin unit.
    pub payout_units_per_payin_unit: String,

    /// Details of the payin (e.g., amount, currency, fees).
    pub payin: QuoteDetails,

    /// Details of the payout (e.g., amount, currency, fees).
    pub payout: QuoteDetails,
}

/// Represents the details of payin or payout in a Quote message.
///
/// This includes the currency, subtotal, total, and any optional fees.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct QuoteDetails {
    /// The currency code (ISO 4217 format) for the payin or payout.
    pub currency_code: String,

    /// The subtotal amount for the transaction, excluding fees.
    pub subtotal: String,

    /// The total amount for the transaction, including fees (if any).
    pub total: String,

    /// Optional fees associated with the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
}

// TODO: Uncomment when we have parse_quote.json vector updated with no payment instructions
#[cfg(test)]
mod tbdex_test_vectors_protocol {
    use super::*;
    use std::fs;

    #[derive(Debug, serde::Deserialize)]
    pub struct TestVector {
        pub input: String,
        pub output: Quote,
    }


    #[test]
    fn parse_quote() {
        let path = "../../tbdex/hosted/test-vectors/protocol/vectors/parse-quote.json";
        let test_vector_json: String = fs::read_to_string(path).unwrap();

        let test_vector: TestVector = serde_json::from_str(&test_vector_json).unwrap();
        let parsed_quote: Quote = Quote::from_json_string(&test_vector.input).unwrap();

        assert_eq!(test_vector.output, parsed_quote);
    }
}
