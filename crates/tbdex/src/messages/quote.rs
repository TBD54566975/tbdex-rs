use crate::signer::sign;

use super::{Message, MessageKind, MessageMetadata, Result};
use chrono::Utc;
use serde::Serialize;
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
    ) -> Result<Self> {
        Ok(Self {
            metadata: MessageMetadata {
                from,
                to,
                kind: MessageKind::Quote,
                id: MessageKind::Quote.typesafe_id()?,
                exchange_id,
                external_id,
                protocol,
                created_at: Utc::now().to_rfc3339(),
            },
            data,
            signature: String::default(), // not set until call to sign()
        })
    }
}

impl Message for Quote {
    fn sign(&mut self, bearer_did: BearerDid) -> Result<()> {
        let metadata = serde_json::to_value(&self.metadata)?;
        let data = serde_json::to_value(&self.data)?;

        self.signature = sign(bearer_did, metadata, data);

        Ok(())
    }

    fn verify(&self) -> Result<()> {
        println!("Quote.verify() invoked");
        Ok(())
    }

    fn clone_box(&self) -> Box<dyn Message> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Serialize)]
pub struct QuoteData {
    pub expires_at: String,
    pub payin: QuoteDetails,
    pub payout: QuoteDetails,
}

#[derive(Clone, Serialize)]
pub struct QuoteDetails {
    pub currency_code: String,
    pub amount: String,
    pub fee: Option<String>,
    pub payment_instructions: Option<PaymentInstructions>,
}

#[derive(Clone, Serialize)]
pub struct PaymentInstructions {
    pub link: Option<String>,
    pub instruction: Option<String>,
}
