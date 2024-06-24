use crate::jose::Signer;

use super::{MessageKind, MessageMetadata, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use web5::apid::dids::bearer_did::BearerDid;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Quote {
    pub metadata: MessageMetadata,
    pub data: QuoteData,
    pub signature: String,
}

impl Quote {
    pub fn new(
        bearer_did: BearerDid,
        to: String,
        from: String,
        exchange_id: String,
        data: QuoteData,
        protocol: String,
        external_id: Option<String>,
    ) -> Result<Self> {
        let metadata = MessageMetadata {
            from,
            to,
            kind: MessageKind::Quote,
            id: MessageKind::Quote.typesafe_id()?,
            exchange_id,
            external_id,
            protocol,
            created_at: Utc::now().to_rfc3339(),
        };

        let key_id = bearer_did.document.verification_method[0].id.clone();
        let web5_signer = bearer_did.get_signer(key_id.clone())?;
        let jose_signer = Signer {
            kid: key_id,
            web5_signer,
        };

        Ok(Self {
            metadata: metadata.clone(),
            data: data.clone(),
            signature: crate::signature::sign(
                jose_signer,
                serde_json::to_value(metadata)?,
                serde_json::to_value(data)?,
            )?,
        })
    }

    pub fn from_json_string(json: &str) -> Result<Self> {
        let quote = serde_json::from_str::<Self>(json)?;

        crate::signature::verify(
            &quote.metadata.from,
            serde_json::to_value(quote.metadata.clone())?,
            serde_json::to_value(quote.data.clone())?,
            quote.signature.clone(),
        )?;

        Ok(quote)
    }

    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self)?)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct QuoteData {
    pub expires_at: String,
    pub payin: QuoteDetails,
    pub payout: QuoteDetails,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct QuoteDetails {
    pub currency_code: String,
    pub amount: String,
    pub fee: Option<String>,
    pub payment_instructions: Option<PaymentInstructions>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct PaymentInstructions {
    pub link: Option<String>,
    pub instruction: Option<String>,
}
