use super::{Message, MessageKind, MessageMetadata, Result};
use web5::apid::dids::bearer_did::BearerDid;

#[derive(Clone)]
pub struct Quote {
    pub metadata: MessageMetadata,
    pub data: QuoteData,
    pub signature: String,
}

impl Quote {
    pub fn new(
        to: String,
        from: String,
        exchange_id: String,
        data: QuoteData,
        protocol: String,
        external_id: Option<String>,
    ) -> Self {
        // 🚧 not functional
        Self {
            metadata: MessageMetadata {
                from,
                to,
                kind: MessageKind::Quote,
                id: String::default(),
                exchange_id,
                external_id,
                protocol,
                created_at: String::default(),
            },
            data,
            signature: String::default(),
        }
    }
}

impl Message for Quote {
    fn sign(&self, _bearer_did: BearerDid) -> Result<()> {
        println!("Quote.sign() invoked");
        Ok(())
    }

    fn verify(&self) -> Result<()> {
        println!("Quote.verify() invoked");
        Ok(())
    }
}

#[derive(Clone)]
pub struct QuoteData {
    pub expires_at: String,
    pub payin: QuoteDetails,
    pub payout: QuoteDetails,
}

#[derive(Clone)]
pub struct QuoteDetails {
    pub currency_code: String,
    pub amount: String,
    pub fee: Option<String>,
    pub payment_instructions: Option<PaymentInstructions>,
}

#[derive(Clone)]
pub struct PaymentInstructions {
    pub link: Option<String>,
    pub instruction: Option<String>,
}
