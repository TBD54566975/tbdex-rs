use crate::signer::sign;

use super::{Message, MessageKind, MessageMetadata, Result};
use chrono::Utc;
use serde::Serialize;
use web5::apid::dids::bearer_did::BearerDid;

#[derive(Clone)]
pub struct Order {
    pub metadata: MessageMetadata,
    pub signature: String,
}

impl Order {
    pub fn new(
        to: String,
        from: String,
        exchange_id: String,
        protocol: String,
        external_id: Option<String>,
    ) -> Result<Self> {
        Ok(Self {
            metadata: MessageMetadata {
                from,
                to,
                kind: MessageKind::Order,
                id: MessageKind::Order.typesafe_id()?,
                exchange_id,
                external_id,
                protocol,
                created_at: Utc::now().to_rfc3339(),
            },
            signature: String::default(), // not set until call to sign()
        })
    }
}

impl Message for Order {
    fn sign(&mut self, bearer_did: BearerDid) -> Result<()> {
        let metadata = serde_json::to_value(&self.metadata)?;
        let data = serde_json::to_value(&OrderData {})?;

        self.signature = sign(bearer_did, metadata, data);

        Ok(())
    }

    fn verify(&self) -> Result<()> {
        println!("Order.verify() invoked");
        Ok(())
    }
}

#[derive(Clone, Serialize)]
pub struct OrderData;
