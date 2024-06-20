use super::{Message, MessageKind, MessageMetadata, Result};
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
            signature: String::default(),
        }
    }
}

impl Message for Order {
    fn sign(&self, _bearer_did: BearerDid) -> Result<()> {
        println!("Order.sign() invoked");
        Ok(())
    }

    fn verify(&self) -> Result<()> {
        println!("Order.verify() invoked");
        Ok(())
    }
}
