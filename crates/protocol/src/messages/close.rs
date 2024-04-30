use ::serde::{Deserialize, Serialize};
use chrono::Utc;
use serde_with::skip_serializing_none;
use type_safe_id::{DynamicType, TypeSafeId};

use super::{Message, MessageError, MessageKind, MessageMetadata};

pub struct Close;

impl Close {
    pub fn create(
        from: String,
        to: String,
        exchange_id: TypeSafeId<DynamicType>,
        reason: Option<String>,
    ) -> Result<Message<CloseData>, MessageError> {
        let metadata = MessageMetadata {
            from,
            to,
            kind: MessageKind::Close,
            id: MessageKind::Close.typesafe_id()?,
            exchange_id,
            created_at: Utc::now(),
        };

        let data = CloseData { reason };

        Ok(Message {
            metadata,
            data,
            signature: None,
        })
    }
}

/// A struct representing the data contained within the [`Message`] for a [`Close`].
///
/// See [Quote](https://github.com/TBD54566975/tbdex/tree/main/specs/protocol#close) for more
/// information.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[skip_serializing_none]
#[serde(rename_all = "camelCase")]
pub struct CloseData {
    /// an explanation of why the exchange is being closed/completed
    reason: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data::TestData;

    #[test]
    fn can_create() {
        let close = Close::create(
            "did:example:from_1234".to_string(),
            "did:example:to_1234".to_string(),
            MessageKind::Rfq.typesafe_id().unwrap(),
            Some("I don't want to do business with you any more".to_string()),
        )
        .expect("failed to create Close");

        assert_eq!(
            close.data.reason,
            Some("I don't want to do business with you any more".to_string())
        );
        assert_eq!(close.metadata.id.type_prefix(), "close");
    }

    #[test]
    fn can_parse_close_from_json() {
        let close = TestData::get_close(
            "did:example:from_1234".to_string(),
            MessageKind::Rfq
                .typesafe_id()
                .expect("failed to generate exchange_id"),
        );
        let json = serde_json::to_string(&close).expect("failed to serialize Close");
        let parsed_close: Message<CloseData> =
            serde_json::from_str(&json).expect("failed to deserialize Close");
        assert_eq!(close, parsed_close);
    }
}
