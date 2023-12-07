use ::serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use type_safe_id::{DynamicType, TypeSafeId};

use super::{Message, MessageError, MessageKind, MessageMetadata};

pub struct Quote;

impl Quote {
    pub fn create(
        from: String,
        to: String,
        exchange_id: TypeSafeId<DynamicType>,
        data: QuoteData,
    ) -> Result<Message<QuoteData>, MessageError> {
        let metadata = MessageMetadata {
            from,
            to,
            kind: MessageKind::Quote,
            id: MessageKind::Quote.typesafe_id()?,
            exchange_id,
            created_at: Utc::now(),
        };

        Ok(Message {
            metadata,
            data,
            signature: None,
        })
    }
}

/// A struct representing the data contained within the [`Message`] for a [`Quote`].
///
/// See [Quote](https://github.com/TBD54566975/tbdex/tree/main/specs/protocol#quote) for more
/// information.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteData {
    /// When this quote expires. Expressed as ISO8601
    pub expires_at: DateTime<Utc>,
    /// the amount of payin currency that the PFI will receive
    pub payin: QuoteDetails,
    /// the amount of payout currency that Alice will receive
    pub payout: QuoteDetails,
    /// Object that describes how to pay the PFI, and how to get paid by the PFI (e.g. BTC address, payment link)
    pub payment_instructions: Option<PaymentInstructions>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteDetails {
    /// ISO 3166 currency code string
    pub currency_code: String,
    /// The amount of currency expressed in the smallest respective unit
    pub amount_subunits: String,
    /// The amount paid in fees expressed in the smallest respectice unit
    pub fee_subunits: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentInstructions {
    /// Link or Instruction describing how to pay the PFI.
    pub payin: Option<PaymentInstruction>,
    /// Link or Instruction describing how to get paid by the PFI
    pub payout: Option<PaymentInstruction>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentInstruction {
    /// Link or Instruction describing how to pay the PFI.
    pub link: Option<String>,
    /// Instruction on how Alice can pay PFI, or how Alice can be paid by the PFI
    pub instruction: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::test_data::TestData;

    use super::*;

    #[test]
    fn can_create() {
        let quote = Quote::create(
            "did:example:from_1234".to_string(),
            "did:example:to_1234".to_string(),
            MessageKind::Rfq.typesafe_id().unwrap(),
            QuoteData {
                expires_at: Utc::now(),
                payin: QuoteDetails {
                    currency_code: "USD".to_string(),
                    amount_subunits: "100".to_string(),
                    fee_subunits: Some("10".to_string()),
                },
                payout: QuoteDetails {
                    currency_code: "BTC".to_string(),
                    amount_subunits: "2500".to_string(),
                    fee_subunits: None,
                },
                payment_instructions: Some(PaymentInstructions {
                    payin: Some(PaymentInstruction {
                        link: Some("example.com/payin".to_string()),
                        instruction: Some("Hand me the cash".to_string()),
                    }),
                    payout: Some(PaymentInstruction {
                        link: None,
                        instruction: Some("BOLT 12".to_string()),
                    }),
                }),
            },
        )
        .expect("failed to create Quote");

        assert_eq!(quote.metadata.id.type_prefix(), "quote");
    }

    #[test]
    fn can_parse_quote_from_json() {
        let quote = TestData::get_quote(
            "did:example:from_1234".to_string(),
            "did:example:to_1234".to_string(),
            MessageKind::Rfq
                .typesafe_id()
                .expect("failed to generate exchange_id"),
        );
        let json = serde_json::to_string(&quote).expect("failed to serialize Quote");
        let parsed_quote: Message<QuoteData> =
            serde_json::from_str(&json).expect("failed to deserialize Quote");
        assert_eq!(quote, parsed_quote);
    }
}
