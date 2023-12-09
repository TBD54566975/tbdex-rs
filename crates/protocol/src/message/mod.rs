pub mod close;
pub mod order;
pub mod order_status;
pub mod quote;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use sha2::{Digest, Sha256};
use type_safe_id::{DynamicType, TypeSafeId};

/// An enum representing all possible [`Message`] kinds.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageKind {
    Close,
    Order,
    OrderStatus,
    Quote,
    Rfq,
}

/// A struct representing the metadata present on every [`Message`].
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageMetadata {
    /// The message's ID
    pub id: TypeSafeId<DynamicType>,
    /// This defines the data property's type (e.g. rfq, quote etc.)
    pub kind: MessageKind,
    /// ID for a "exchange" of messages between Alice <-> PFI.
    /// Set by the first message in an exchange.
    pub exchange_id: TypeSafeId<DynamicType>,
    /// The sender's DID
    pub from: String,
    /// The recipient's DID
    pub to: String,
    /// ISO 8601
    pub created_at: DateTime<Utc>,
}

/// A struct representing the structure and common functionality available to all Messages.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Message<T> {
    /// An object containing fields about the message
    pub metadata: MessageMetadata,
    /// The actual message content
    pub data: T,
    /// The signature that verifies the authenticity and integrity of the message
    pub signature: Option<String>,
}

/// Errors that can occur when working with [`Message`]s.
#[derive(thiserror::Error, Debug)]
pub enum MessageError {
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    TypeSafeIdError(#[from] type_safe_id::Error),
}

impl<T: Serialize> Message<T> {
    /// Create a 44 byte digest of the message with these steps:
    ///
    /// 1. Initialize payload to be a json object that contains the `metadata` and `data`` properties whose values
    ///    are the respective metadata and data values of the message or resource for which the digest is being computed
    /// 2. JSON serialize payload using the JSON Canonicalization Scheme (JCS) as defined in RFC-8785
    /// 3. Compute the sha256 hash of the serialized payload
    /// 4. base64url encode the hash without padding as defined in RFC-7515
    pub fn digest(&self) -> Result<String, MessageError> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct DigestMessage<'a, U: Serialize> {
            pub metadata: &'a MessageMetadata,
            pub data: &'a U,
        }

        let metadata_and_data_only = DigestMessage {
            metadata: &self.metadata,
            data: &self.data,
        };
        let jcs = serde_jcs::to_string(&metadata_and_data_only)?;
        println!("jcs: {}", jcs);
        let sha256 = Sha256::new()
            .chain_update(jcs)
            .finalize()
            .to_vec();
        let base64_url_encoded = base64_url::encode(&sha256);

        Ok(base64_url_encoded)
    }
}

impl MessageKind {
    /// Returns the [`TypeSafeId`] of the [`MessageKind`].
    pub fn typesafe_id(&self) -> Result<TypeSafeId<DynamicType>, MessageError> {
        let serialized_kind = to_string(&self)?;
        let dynamic_type = DynamicType::new(serialized_kind.trim_matches('"'))?;
        Ok(TypeSafeId::new_with_type(dynamic_type))
    }
}

#[cfg(test)]
mod tests {
    use crate::test_data::TestData;

    use super::*;

    #[test]
    fn message_kind_typesafe_id() {
        let close_id = MessageKind::Close.typesafe_id().unwrap();
        let order_id = MessageKind::Order.typesafe_id().unwrap();
        let order_status_id = MessageKind::OrderStatus.typesafe_id().unwrap();
        let quote_id = MessageKind::Quote.typesafe_id().unwrap();
        let rfq_id = MessageKind::Rfq.typesafe_id().unwrap();

        assert!(close_id.to_string().starts_with("close_"));
        assert!(order_id.to_string().starts_with("order_"));
        assert!(order_status_id.to_string().starts_with("orderstatus_"));
        assert!(quote_id.to_string().starts_with("quote_"));
        assert!(rfq_id.to_string().starts_with("rfq_"));
    }

    #[test]
    fn can_digest_and_sign() {
        let message = TestData::get_close(
            "did:example:from_1234".to_string(),
            MessageKind::Rfq
                .typesafe_id()
                .expect("failed to generate exchange_id"),
        );
        let digest = message.digest().expect("Could not produce message digest");
        println!("digest: {}", digest);
    }
}
