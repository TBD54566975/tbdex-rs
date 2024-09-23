use super::{ResourceKind, ResourceMetadata, Result};
use crate::{
    json::{FromJson, ToJson},
    json_schemas::generated::{OFFERING_DATA_JSON_SCHEMA, RESOURCE_JSON_SCHEMA},
    DEFAULT_PROTOCOL_VERSION,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use web5::{
    credentials::presentation_definition::PresentationDefinition, dids::bearer_did::BearerDid,
};

/// Represents an Offering resource in the tbDEX protocol.
///
/// An Offering resource is created by a PFI to define the requirements for a given exchange,
/// including payin/payout details, rates, and other conditions.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Offering {
    /// Metadata about the resource, including sender, type of resource, and protocol information.
    pub metadata: ResourceMetadata,

    /// The public data part of the Offering, such as exchange rates and payment methods.
    pub data: OfferingData,

    /// The signature verifying the authenticity and integrity of the Offering resource.
    pub signature: String,
}

impl ToJson for Offering {}
impl FromJson for Offering {}

impl Offering {
    /// Creates a new Offering resource.
    ///
    /// # Arguments
    ///
    /// * `from` - The DID of the sender (the PFI).
    /// * `data` - The data containing the offering details.
    /// * `protocol` - Optional protocol version; defaults to the current version if not provided.
    ///
    /// # Returns
    ///
    /// A new instance of `Offering` containing the metadata, data, and an empty signature.
    pub fn create(from: &str, data: &OfferingData, protocol: Option<String>) -> Result<Self> {
        let now = Utc::now().to_rfc3339();

        let metadata = ResourceMetadata {
            kind: ResourceKind::Offering,
            from: from.to_string(),
            id: ResourceKind::Offering.typesafe_id()?,
            protocol: protocol.unwrap_or_else(|| DEFAULT_PROTOCOL_VERSION.to_string()),
            created_at: now.clone(),
            updated_at: Some(now),
        };

        let offering = Self {
            metadata: metadata.clone(),
            data: data.clone(),
            signature: String::default(),
        };

        Ok(offering)
    }

    /// Signs the Offering resource using the provided Bearer DID.
    ///
    /// # Arguments
    ///
    /// * `bearer_did` - The DID to sign the Offering resource.
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

    /// Verifies the validity of the Offering resource.
    ///
    /// This method ensures that the resource adheres to its JSON schema
    /// and verifies the signature to ensure authenticity and integrity.
    ///
    /// # Returns
    ///
    /// An empty result if verification succeeds, or an error if verification fails.
    pub fn verify(&self) -> Result<()> {
        // verify resource json schema
        crate::json_schemas::validate_from_str(RESOURCE_JSON_SCHEMA, self)?;

        // verify data json schema
        crate::json_schemas::validate_from_str(OFFERING_DATA_JSON_SCHEMA, &self.data)?;

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

/// Represents the data for an Offering resource in the tbDEX protocol.
///
/// This includes details such as the exchange rate, payin/payout methods, and any required credentials.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OfferingData {
    /// A brief description of what is being offered.
    pub description: String,

    /// The exchange rate, indicating how many payout units are provided per payin unit.
    pub payout_units_per_payin_unit: String,

    /// Details about the payin currency and available methods.
    pub payin: PayinDetails,

    /// Details about the payout currency and available methods.
    pub payout: PayoutDetails,

    /// Optional required claims (Verifiable Credentials) needed to participate in the offering.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_claims: Option<PresentationDefinition>,

    /// Details about the offering's cancellation policy.
    pub cancellation: CancellationDetails,
}

/// Represents the details of the payin for an Offering.
///
/// This includes the currency code, optional limits, and available methods for making the payin.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PayinDetails {
    /// The currency code (ISO 4217 format) for the payin.
    pub currency_code: String,

    /// Optional minimum payin amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<String>,

    /// Optional maximum payin amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<String>,

    /// A list of available methods for making the payin.
    pub methods: Vec<PayinMethod>,
}

/// Represents a method for making a payin in an Offering.
///
/// This includes details about the method, such as its kind, optional name and description, and any associated fees.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PayinMethod {
    /// The unique identifier for the payment method.
    pub kind: String,

    /// Optional name of the payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Optional description of the payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Optional group categorization for the payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,

    /// Optional JSON schema specifying required payment details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_payment_details: Option<serde_json::Value>,

    /// Optional fee associated with the payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,

    /// Optional minimum amount for using the payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<String>,

    /// Optional maximum amount for using the payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<String>,
}

/// Represents the details of the payout for an Offering.
///
/// This includes the currency code, optional limits, and available methods for receiving the payout.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PayoutDetails {
    /// The currency code (ISO 4217 format) for the payout.
    pub currency_code: String,

    /// Optional minimum payout amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<String>,

    /// Optional maximum payout amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<String>,

    /// A list of available methods for receiving the payout.
    pub methods: Vec<PayoutMethod>,
}

/// Represents a method for receiving a payout in an Offering.
///
/// This includes details about the method, such as its kind, optional name and description, and any associated fees.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PayoutMethod {
    /// The unique identifier for the payout method.
    pub kind: String,

    /// Optional name of the payout method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Optional description of the payout method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Optional group categorization for the payout method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,

    /// Optional JSON schema specifying required payment details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_payment_details: Option<serde_json::Value>,

    /// Optional fee associated with the payout method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,

    /// Optional minimum amount for using the payout method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<String>,

    /// Optional maximum amount for using the payout method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<String>,

    /// The estimated time (in seconds) for the payout to be settled.
    pub estimated_settlement_time: i64,
}

/// Represents the cancellation policy for an Offering.
///
/// This includes whether cancellation is enabled and optional terms describing the cancellation policy.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CancellationDetails {
    /// Whether cancellation is enabled for the offering.
    pub enabled: bool,

    /// Optional URL to a page that describes the terms of cancellation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_url: Option<String>,

    /// Optional human-readable description of the cancellation terms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use web5::dids::methods::did_jwk::DidJwk;

    #[test]
    fn can_create_and_sign_and_verify() {
        let bearer_did = DidJwk::create(None).unwrap();

        let mut offering = Offering::create(
            &bearer_did.did.uri,
            &OfferingData {
                description: "Selling BTC for USD".to_string(),
                payout_units_per_payin_unit: "1.5".to_string(),
                payin: PayinDetails {
                    currency_code: "USD".to_string(),
                    ..Default::default()
                },
                payout: PayoutDetails {
                    currency_code: "BTC".to_string(),
                    ..Default::default()
                },
                required_claims: Some(PresentationDefinition {
                    id: "7ce4004c-3c38-4853-968b-e411bafcd945".to_string(),
                    name: None,
                    purpose: None,
                    input_descriptors: vec![],
                }),
                cancellation: CancellationDetails {
                    enabled: false,
                    ..Default::default()
                },
            },
            None,
        )
        .unwrap();

        offering.sign(&bearer_did).unwrap();

        assert_ne!(String::default(), offering.signature);

        let offering_json_string = offering.to_json_string().unwrap();

        assert_ne!(String::default(), offering_json_string);

        let parsed_offering = Offering::from_json_string(&offering_json_string).unwrap();

        assert_eq!(offering, parsed_offering);
    }
}

#[cfg(test)]
mod tbdex_test_vectors_protocol {
    use super::*;
    use std::fs;

    #[derive(Debug, serde::Deserialize)]
    pub struct TestVector {
        pub input: String,
        pub output: Offering,
    }

    #[test]
    fn parse_offering() {
        let path = "../../tbdex/hosted/test-vectors/protocol/vectors/parse-offering.json";
        let test_vector_json: String = fs::read_to_string(path).unwrap();

        let test_vector: TestVector = serde_json::from_str(&test_vector_json).unwrap();
        let parsed_offering: Offering = Offering::from_json_string(&test_vector.input).unwrap();

        assert_eq!(test_vector.output, parsed_offering);
    }
}
