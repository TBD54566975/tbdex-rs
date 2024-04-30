use crate::resources::{Resource, ResourceError, ResourceKind, ResourceMetadata};
use chrono::Utc;
use credentials::pex::v2::PresentationDefinition;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

/// Struct that interacts with an [`Offering`] [`Resource`]
pub struct Offering;

/// Struct for passing parameters to [`Offering::create`]
#[derive(Debug, Default)]
pub struct CreateOptions {
    pub from: String,
    pub protocol: Option<String>,
    pub data: OfferingData,
}

impl Offering {
    pub fn create(options: CreateOptions) -> Result<Resource<OfferingData>, ResourceError> {
        let metadata = ResourceMetadata {
            id: ResourceKind::Offering.typesafe_id()?,
            kind: ResourceKind::Offering,
            from: options.from,
            created_at: Utc::now(),
            updated_at: Some(Utc::now()),
            protocol: match options.protocol {
                Some(p) => p,
                None => "1.0".to_string(),
            },
        };

        // todo implement signing https://github.com/TBD54566975/tbdex-rs/issues/27
        let signature = "todo a valid signature".to_string();

        Ok(Resource {
            metadata,
            data: options.data,
            signature,
        })
    }
}

/// Struct the data contained within the [`Resource`] for an [`Offering`].
///
/// See [Offering](https://github.com/TBD54566975/tbdex/tree/main/specs/protocol#offering) for more
/// information.
#[derive(Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OfferingData {
    /// Brief description of what is being offered.
    pub description: String,
    /// Number of payout units Alice would get for 1 payin unit
    pub payout_units_per_payin_unit: String,
    /// Details and options associated to the payin currency
    pub payin: PayinDetails,
    /// Details and options associated to the payout currency
    pub payout: PayoutDetails,
    /// Claim(s) required when submitting an RFQ for this offering.
    pub required_claims: PresentationDefinition,
}

/// Struct for [Offering's PayinDetails](https://github.com/TBD54566975/tbdex/tree/main/specs/protocol#payindetails)
#[derive(Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PayinDetails {
    /// ISO 4217 currency code string
    pub currency_code: String,
    /// Minimum amount of currency that the offer is valid for
    pub min: Option<String>,
    /// Maximum amount of currency that the offer is valid for
    pub max: Option<String>,
    /// A list of payment methods to select from
    pub methods: Vec<PayinMethod>,
}

/// Struct for [Offering's PayinMethod](https://github.com/TBD54566975/tbdex/tree/main/specs/protocol#payinmethod)
#[derive(Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PayinMethod {
    /// Unique string identifying a single kind of payment method i.e. (i.e. DEBIT_CARD, BITCOIN_ADDRESS, SQUARE_PAY)
    pub kind: String,
    /// Payment Method name. Expected to be rendered on screen.
    pub name: Option<String>,
    /// Blurb containing helpful information about the payment method. Expected to be rendered on screen. e.g. "segwit addresses only"
    pub description: Option<String>,
    /// The category for which the given method belongs to
    pub group: Option<String>,
    /// A JSON Schema containing the fields that need to be collected in the RFQ's selected payment methods in order to use this payment method.
    pub required_payment_details: Option<JsonValue>,
    /// Fee charged to use this payment method. absence of this field implies that there is no additional fee associated to the respective payment method
    pub fee: Option<String>,
    /// Minimum amount required to use this payment method.
    pub min: Option<String>,
    /// Maximum amount allowed when using this payment method.
    pub max: Option<String>,
}

/// Struct for [Offering's PayoutDetails](https://github.com/TBD54566975/tbdex/tree/main/specs/protocol#payoutdetails)
#[derive(Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PayoutDetails {
    /// ISO 4217 currency code string
    pub currency_code: String,
    /// Minimum amount of currency that the offer is valid for
    pub min: Option<String>,
    /// Maximum amount of currency that the offer is valid for
    pub max: Option<String>,
    /// A list of payment methods to select from
    pub methods: Vec<PayoutMethod>,
}

/// Struct for [Offering's PayinMethod](https://github.com/TBD54566975/tbdex/tree/main/specs/protocol#payinmethod)
#[derive(Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PayoutMethod {
    /// Unique string identifying a single kind of payment method i.e. (i.e. DEBIT_CARD, BITCOIN_ADDRESS, SQUARE_PAY)
    pub kind: String,
    /// Estimated time taken to settle an order, expressed in seconds
    pub estimated_settlement_time: u64,
    /// Payment Method name. Expected to be rendered on screen.
    pub name: Option<String>,
    /// Blurb containing helpful information about the payment method. Expected to be rendered on screen. e.g. "segwit addresses only"
    pub description: Option<String>,
    /// The category for which the given method belongs to
    pub group: Option<String>,
    /// A JSON Schema containing the fields that need to be collected in the RFQ's selected payment methods in order to use this payment method.
    pub required_payment_details: Option<JsonValue>,
    /// Fee charged to use this payment method. absence of this field implies that there is no additional fee associated to the respective payment method
    pub fee: Option<String>,
    /// Minimum amount required to use this payment method.
    pub min: Option<String>,
    /// Maximum amount allowed when using this payment method.
    pub max: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data::TestData;

    #[test]
    fn can_create() {
        let offering = Offering::create(CreateOptions {
            from: "did:example:1234".to_string(),
            data: OfferingData {
                description: "my fake offering".to_string(),
                payout_units_per_payin_unit: "2".to_string(),
                payin: PayinDetails {
                    currency_code: "USD".to_string(),
                    ..Default::default()
                },
                payout: PayoutDetails {
                    currency_code: "BTC".to_string(),
                    ..Default::default()
                },
                required_claims: PresentationDefinition::default(),
            },
            ..Default::default()
        })
        .expect("failed to create offering");

        assert_eq!(offering.metadata.id.type_prefix(), "offering");
        assert_eq!(offering.metadata.from, "did:example:1234".to_string());
        assert_eq!(offering.metadata.protocol, "1.0".to_string());
        assert_eq!(offering.data.description, "my fake offering");
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
