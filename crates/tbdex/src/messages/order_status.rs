use crate::signer::sign;

use super::{Message, MessageKind, MessageMetadata, Result};
use chrono::Utc;
use serde::Serialize;
use web5::apid::dids::bearer_did::BearerDid;

#[derive(Clone)]
pub struct OrderStatus {
    pub metadata: MessageMetadata,
    pub data: OrderStatusData,
    pub signature: String,
}

impl OrderStatus {
    pub fn new(
        to: String,
        from: String,
        exchange_id: String,
        data: OrderStatusData,
        protocol: String,
        external_id: Option<String>,
    ) -> Result<Self> {
        Ok(Self {
            metadata: MessageMetadata {
                from,
                to,
                kind: MessageKind::OrderStatus,
                id: MessageKind::OrderStatus.typesafe_id()?,
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

impl Message for OrderStatus {
    fn sign(&mut self, bearer_did: BearerDid) -> Result<()> {
        let metadata = serde_json::to_value(&self.metadata)?;
        let data = serde_json::to_value(&self.data)?;

        self.signature = sign(bearer_did, metadata, data);

        Ok(())
    }

    fn verify(&self) -> Result<()> {
        println!("Order.verify() invoked");
        Ok(())
    }

    fn clone_box(&self) -> Box<dyn Message> {
        Box::new(self.clone())
    }
}

#[derive(Clone, Serialize)]
pub struct OrderStatusData {
    pub order_status: String,
}
