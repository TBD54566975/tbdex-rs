use ::serde::{Deserialize, Serialize};
use chrono::Utc;
use type_safe_id::{DynamicType, TypeSafeId};

use super::{Message, MessageError, MessageKind, MessageMetadata};

pub struct OrderStatus;

impl OrderStatus {
    pub fn create(
        from: String,
        to: String,
        exchange_id: TypeSafeId<DynamicType>,
        order_status: String,
    ) -> Result<Message<OrderStatusData>, MessageError> {
        let metadata = MessageMetadata {
            from,
            to,
            kind: MessageKind::OrderStatus,
            id: MessageKind::OrderStatus.typesafe_id()?,
            exchange_id: exchange_id,
            created_at: Utc::now(),
        };

        let data = OrderStatusData {
          order_status,
        };

        Ok(Message {
            metadata,
            data,
            signature: None,
        })
    }
}

/// A struct representing the data contained within the [`Message`] for an [`OrderStatus`].
/// Currently, [`OrderStatus`] contains no data fields.
///
/// See [OrderStatus](https://github.com/TBD54566975/tbdex/tree/main/specs/protocol#orderstatus) for more
/// information.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderStatusData {
  /// Current status of Order that's being executed
  order_status: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create() {
      let order_status: Message<OrderStatusData> = OrderStatus::create(
        "did:example:from_1234".to_string(),
        "did:example:to_1234".to_string(),
        MessageKind::Rfq.typesafe_id().unwrap(),
        "COMPLETED".to_string()
      )
      .expect("failed to create OrderStatus");

      assert_eq!(order_status.metadata.id.type_prefix(), "orderstatus");
    }

    #[test]
    fn can_parse_order_status_from_json() {
      let order_status = OrderStatus::create(
        "did:example:from_1234".to_string(), 
        "did:example:to_1234".to_string(),
        MessageKind::Rfq.typesafe_id().unwrap(),
        "COMPLETED".to_string()
      )
      .expect("Could not create OrderStatus");
      let json: String = serde_json::to_string(&order_status).expect("failed to serialize OrderStatus");
      let parsed_order_status: Message<OrderStatusData> =
          serde_json::from_str(&json).expect("failed to deserialize OrderStatus");
      assert_eq!(order_status, parsed_order_status);
    }
}
