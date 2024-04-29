use ::serde::{Deserialize, Serialize};
use chrono::Utc;
use type_safe_id::{DynamicType, TypeSafeId};

use super::{Message, MessageError, MessageKind, MessageMetadata};

pub struct Order;

impl Order {
    pub fn create(
        from: String,
        to: String,
        exchange_id: TypeSafeId<DynamicType>,
    ) -> Result<Message<OrderData>, MessageError> {
        let metadata = MessageMetadata {
            from,
            to,
            kind: MessageKind::Order,
            id: MessageKind::Order.typesafe_id()?,
            exchange_id: exchange_id,
            created_at: Utc::now(),
        };

        let data = OrderData;

        Ok(Message {
            metadata,
            data,
            signature: None,
        })
    }
}

/// A struct representing the data contained within the [`Message`] for an [`Order`].
/// Currently, [`Order`] contains no data fields.
///
/// See [Order](https://github.com/TBD54566975/tbdex/tree/main/specs/protocol#order) for more
/// information.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderData;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create() {
        let order = Order::create(
            "did:example:from_1234".to_string(),
            "did:example:to_1234".to_string(),
            MessageKind::Rfq.typesafe_id().unwrap(),
        )
        .expect("failed to create Order");

        assert_eq!(order.metadata.id.type_prefix(), "order");
    }

    #[test]
    fn can_parse_order_from_json() {
        let order = Order::create(
            "did:example:from_1234".to_string(),
            "did:example:to_1234".to_string(),
            MessageKind::Rfq.typesafe_id().unwrap(),
        )
        .expect("Could not create Order");
        let json: String = serde_json::to_string(&order).expect("failed to serialize Order");
        let parsed_order: Message<OrderData> =
            serde_json::from_str(&json).expect("failed to deserialize Order");
        assert_eq!(order, parsed_order);
    }
}
