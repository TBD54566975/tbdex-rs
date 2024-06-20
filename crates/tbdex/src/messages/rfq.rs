use crate::resources::offering::Offering;

use super::{Message, MessageKind, MessageMetadata, Result};
use web5::apid::dids::bearer_did::BearerDid;

#[derive(Clone)]
pub struct Rfq {
    pub metadata: MessageMetadata,
    pub data: RfqData,
    pub private_data: RfqPrivateData,
    pub signature: String,
}

impl Rfq {
    pub fn new(
        to: String,
        from: String,
        _create_rfq_data: CreateRfqData,
        protocol: String,
        external_id: Option<String>,
    ) -> Self {
        // 🚧 not functional
        Self {
            metadata: MessageMetadata {
                from,
                to,
                kind: MessageKind::Rfq,
                id: String::default(),
                exchange_id: String::default(),
                external_id,
                protocol,
                created_at: String::default(),
            },
            data: RfqData {
                offering_id: String::default(),
                payin: SelectedPayinMethod {
                    kind: String::default(),
                    payment_details_hash: None,
                    amount: String::default(),
                },
                payout: SelectedPayoutMethod {
                    kind: String::default(),
                    payment_details_hash: None,
                },
                claims_hash: None,
            },
            private_data: RfqPrivateData {
                salt: String::default(),
                payin: None,
                payout: None,
                claims: None,
            },
            signature: String::default(),
        }
    }

    pub fn verify_offering_requirements(&self, _offering: Offering) -> Result<bool> {
        println!("Rfq.verify_offering_requirements() invoked");
        Ok(true)
    }

    pub fn verify_all_private_data(&self) -> Result<bool> {
        println!("Rfq.verify_all_private_data() invoked");
        Ok(true)
    }

    pub fn verify_present_private_data(&self) -> Result<bool> {
        println!("Rfq.verify_present_private_data() invoked");
        Ok(true)
    }
}

impl Message for Rfq {
    fn sign(&self, _bearer_did: BearerDid) -> Result<()> {
        println!("Rfq.sign() invoked");
        Ok(())
    }

    fn verify(&self) -> Result<()> {
        println!("Rfq.verify() invoked");
        Ok(())
    }
}

#[derive(Clone)]
pub struct CreateRfqData {
    pub offering_id: String,
    pub payin: CreateSelectedPayinMethod,
    pub payout: CreateSelectedPayoutMethod,
    pub claims: Vec<String>,
}

#[derive(Clone)]
pub struct CreateSelectedPayinMethod {
    pub kind: String,
    pub payment_details: String, // 🚧 Map<string, JsonNode>
    pub amount: String,
}

#[derive(Clone)]
pub struct CreateSelectedPayoutMethod {
    pub kind: String,
    pub payment_details: String, // 🚧 Map<string, JsonNode>
}

#[derive(Clone)]
pub struct RfqData {
    pub offering_id: String,
    pub payin: SelectedPayinMethod,
    pub payout: SelectedPayoutMethod,
    pub claims_hash: Option<String>,
}

#[derive(Clone)]
pub struct SelectedPayinMethod {
    pub kind: String,
    pub payment_details_hash: Option<String>,
    pub amount: String,
}

#[derive(Clone)]
pub struct SelectedPayoutMethod {
    pub kind: String,
    pub payment_details_hash: Option<String>,
}

#[derive(Clone)]
pub struct RfqPrivateData {
    pub salt: String,
    pub payin: Option<PrivatePaymentDetails>,
    pub payout: Option<PrivatePaymentDetails>,
    pub claims: Option<Vec<String>>,
}

#[derive(Clone)]
pub struct PrivatePaymentDetails {
    pub payment_details: String, // 🚧 Map<string, JsonNode>
}
