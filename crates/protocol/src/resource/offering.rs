use crate::resource::{Resource, ResourceError, ResourceKind, ResourceMetadata};
use chrono::Utc;
use credentials::pex::v2::PresentationDefinition;
use jsonschema::{Draft, JSONSchema};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use serde_with::skip_serializing_none;

/// Struct that interacts with an [`Offering`] [`Resource`]
pub struct Offering;

impl Offering {
    pub fn create(
        from: String,
        data: OfferingData,
    ) -> Result<Resource<OfferingData>, ResourceError> {
        let metadata = ResourceMetadata {
            id: ResourceKind::Offering.typesafe_id()?,
            kind: ResourceKind::Offering,
            from,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        Ok(Resource {
            metadata,
            data,
            signature: None,
        })
    }
}

/// A struct representing the data contained within the [`Resource`] for an [`Offering`].
///
/// See [Offering](https://github.com/TBD54566975/tbdex/tree/main/specs/protocol#offering) for more
/// information.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OfferingData {
    /// Brief description of what is being offered.
    pub description: String,
    /// Number of payout units Alice would get for 1 payin unit
    pub payout_units_per_payin_unit: String,
    /// Details about the currency that the PFI is accepting as payment.
    pub payin_currency: CurrencyDetails,
    /// Details about the currency that the PFI is selling.
    pub payout_currency: CurrencyDetails,
    /// A list of payment methods the counterparty (Alice) can choose to send payment to the PFI
    /// from in order to qualify for this offering.
    pub payin_methods: Vec<PaymentMethod>,
    /// A list of payment methods the counterparty (Alice) can choose to receive payment from the
    /// PFI in order to qualify for this offering.
    pub payout_methods: Vec<PaymentMethod>,
    /// Articulates the claim(s) required when submitting an RFQ for this offering.
    pub required_claims: PresentationDefinition,
}

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
#[skip_serializing_none]
#[serde(rename_all = "camelCase")]
pub struct CurrencyDetails {
    /// ISO 3166 currency code string
    pub currency_code: String,
    /// Minimum amount of currency that the offer is valid for
    pub min_amount: Option<String>,
    /// Maximum amount of currency that the offer is valid for
    pub max_amount: Option<String>,
}

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
#[skip_serializing_none]
#[serde(rename_all = "camelCase")]
pub struct PaymentMethod {
    /// Type of payment method (i.e. `DEBIT_CARD`, `BITCOIN_ADDRESS`, `SQUARE_PAY`)
    pub kind: String,
    /// A JSON Schema containing the fields that need to be collected in order to use this
    /// payment method
    pub required_payment_details: Option<JsonValue>,
    /// The fee expressed in the currency's sub units to make use of this payment method
    pub fee: Option<String>,
}

impl PaymentMethod {
    pub fn required_payment_details_schema(&self) -> Option<JSONSchema> {
        self.required_payment_details
            .as_ref()
            .map(|json| {
                JSONSchema::options()
                    .with_draft(Draft::Draft7)
                    .compile(json)
                    .ok()
            })
            .flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data::TestData;

    #[test]
    fn can_create() {
        let offering = Offering::create(
            "did:example:1234".to_string(),
            OfferingData {
                description: "my fake offering".to_string(),
                payout_units_per_payin_unit: "1".to_string(),
                payin_currency: CurrencyDetails {
                    currency_code: "USD".to_string(),
                    ..Default::default()
                },
                payout_currency: CurrencyDetails {
                    currency_code: "USD".to_string(),
                    ..Default::default()
                },
                payin_methods: vec![],
                payout_methods: vec![],
                required_claims: PresentationDefinition::default(),
            },
        )
        .expect("failed to create offering");

        assert_eq!(offering.data.description, "my fake offering");
        assert_eq!(offering.metadata.id.type_prefix(), "offering");
    }

    #[test]
    fn can_parse_offering_from_json() {
        let offering = TestData::get_offering("did:example:1234".to_string());
        let json = serde_json::to_string(&offering).expect("failed to serialize offering");
        let parsed_offering: Resource<OfferingData> =
            serde_json::from_str(&json).expect("failed to deserialize offering");

        assert_eq!(offering, parsed_offering)
    }
}
