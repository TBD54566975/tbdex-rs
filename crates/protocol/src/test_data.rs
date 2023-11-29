use crate::resource::Resource;
use crate::resources::offering::{CurrencyDetails, Offering, OfferingData, PaymentMethod};
use credentials::pex::v2::{Constraints, Field, InputDescriptor, PresentationDefinition};
use serde_json::{json, Value as JsonValue};

#[cfg(test)]
pub struct TestData;

#[cfg(test)]
impl TestData {
    pub fn get_offering(from: String) -> Resource<OfferingData> {
        Offering::create(
            from,
            OfferingData {
                description: "A sample offering".to_string(),
                payout_units_per_payin_unit: "1".to_string(),
                payin_currency: CurrencyDetails {
                    currency_code: "AUD".to_string(),
                    min_subunits: Some("1".to_string()),
                    max_subunits: Some("10000".to_string()),
                },
                payout_currency: CurrencyDetails {
                    currency_code: "USDC".to_string(),
                    ..Default::default()
                },
                payin_methods: vec![PaymentMethod {
                    kind: "BTC_ADDRESS".to_string(),
                    required_payment_details: Some(TestData::required_payment_details_schema()),
                    ..Default::default()
                }],
                payout_methods: vec![PaymentMethod {
                    kind: "MOMO".to_string(),
                    required_payment_details: Some(TestData::required_payment_details_schema()),
                    ..Default::default()
                }],
                required_claims: TestData::get_presentation_definition(),
            },
        )
        .expect("failed to create offering")
    }

    fn get_presentation_definition() -> PresentationDefinition {
        PresentationDefinition {
            id: "test-pd-id".to_string(),
            name: Some("simple PD".to_string()),
            purpose: Some("pd for testing".to_string()),
            input_descriptors: vec![TestData::get_input_descriptor()],
            ..Default::default()
        }
    }

    fn get_input_descriptor() -> InputDescriptor {
        InputDescriptor {
            id: "whatever".to_string(),
            purpose: Some("id for testing".to_string()),
            constraints: Constraints {
                fields: Some(vec![Field {
                    path: vec!["$.credentialSubject.btcAddress".to_string()],
                    ..Default::default()
                }]),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn required_payment_details_schema() -> JsonValue {
        json! {
            r#"
                {
                  "${'$'}schema": "http://json-schema.org/draft-07/schema",
                  "additionalProperties": false,
                  "type": "object",
                  "properties": {
                    "phoneNumber": {
                      "minLength": 12,
                      "pattern": "^+2547[0-9]{8}${'$'}",
                      "description": "Mobile Money account number of the Recipient",
                      "type": "string",
                      "title": "Phone Number",
                      "maxLength": 12
                    },
                    "accountHolderName": {
                      "pattern": "^[A-Za-zs'-]+${'$'}",
                      "description": "Name of the account holder as it appears on the Mobile Money account",
                      "type": "string",
                      "title": "Account Holder Name",
                      "maxLength": 32
                    }
                  },
                  "required": [
                    "accountNumber",
                    "accountHolderName"
                  ]
                }
            "#
        }
    }
}
