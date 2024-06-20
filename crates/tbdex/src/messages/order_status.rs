use super::{Message, MessageKind, MessageMetadata, Result};
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
    ) -> Self {
        // 🚧 not functional
        Self {
            metadata: MessageMetadata {
                from,
                to,
                kind: MessageKind::Order,
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

impl Message for OrderStatus {
    fn sign(&self, _bearer_did: BearerDid) -> Result<()> {
        println!("Order.sign() invoked");
        Ok(())
    }

    fn verify(&self) -> Result<()> {
        println!("Order.verify() invoked");
        Ok(())
    }
}

#[derive(Clone)]
pub struct OrderStatusData {
    pub order_status: String,
}
