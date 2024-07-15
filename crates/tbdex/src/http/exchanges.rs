use super::{JsonDeserializer, JsonSerializer};
use crate::messages::{cancel::Cancel, order::Order, rfq::Rfq, MessageKind};
use serde::{de::Visitor, Deserialize, Deserializer, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateExchangeRequestBody {
    pub message: Rfq,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
}
impl JsonDeserializer for CreateExchangeRequestBody {}
impl JsonSerializer for CreateExchangeRequestBody {}

#[derive(Serialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum WalletUpdateMessage {
    Order(Order),
    Cancel(Cancel),
}
impl JsonSerializer for WalletUpdateMessage {}

impl<'de> Deserialize<'de> for WalletUpdateMessage {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MessageVisitor;

        impl<'de> Visitor<'de> for MessageVisitor {
            type Value = WalletUpdateMessage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an Order or Cancel")
            }

            fn visit_some<D>(self, deserializer: D) -> std::result::Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value: serde_json::Value = Deserialize::deserialize(deserializer)?;

                let kind_str = value
                    .get("metadata")
                    .and_then(|m| m.get("kind"))
                    .and_then(|k| k.as_str());

                match kind_str {
                    Some(k) => match MessageKind::from_str(k) {
                        Ok(kind) => match kind {
                            MessageKind::Order => {
                                if let Ok(order) = serde_json::from_value::<Order>(value.clone()) {
                                    Ok(WalletUpdateMessage::Order(order))
                                } else {
                                    Err(serde::de::Error::custom("failed to deserialize order"))
                                }
                            }
                            MessageKind::Cancel => {
                                if let Ok(cancel) = serde_json::from_value::<Cancel>(value.clone())
                                {
                                    Ok(WalletUpdateMessage::Cancel(cancel))
                                } else {
                                    Err(serde::de::Error::custom("failed to deserialize cancel"))
                                }
                            }
                            _ => Err(serde::de::Error::custom(format!(
                                "unexpected message kind {:?}",
                                kind
                            ))),
                        },
                        Err(_) => Err(serde::de::Error::custom(format!(
                            "unexpected message kind {}",
                            k
                        ))),
                    },
                    None => Err(serde::de::Error::custom(format!(
                        "unexpected message kind {:?}",
                        kind_str
                    ))),
                }
            }

            fn visit_none<E>(self) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Err(serde::de::Error::custom("message is missing"))
            }
        }

        deserializer.deserialize_option(MessageVisitor)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct UpdateExchangeRequestBody {
    pub message: WalletUpdateMessage,
}
impl JsonDeserializer for UpdateExchangeRequestBody {}
impl JsonSerializer for UpdateExchangeRequestBody {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[derive(Debug, serde::Deserialize)]
    pub struct TestVector {
        pub input: String,
        pub output: Order,
    }

    #[test]
    fn order_update_exchange_request_body() {
        let path = "../../tbdex/hosted/test-vectors/protocol/vectors/parse-order.json";
        let test_vector_json: String = fs::read_to_string(path).unwrap();

        let test_vector: TestVector = serde_json::from_str(&test_vector_json).unwrap();
        let parsed_order = Order::from_json_string(&test_vector.input).unwrap();

        let update_exchange_request_body = UpdateExchangeRequestBody {
            message: WalletUpdateMessage::Order(parsed_order),
        };

        let serialized = update_exchange_request_body.to_json_string().unwrap();
        let deserialized = UpdateExchangeRequestBody::from_json_string(&serialized).unwrap();

        assert_eq!(update_exchange_request_body, deserialized);
    }

    #[test]
    fn cancel_update_exchange_request_body() {
        let path = "../../tbdex/hosted/test-vectors/protocol/vectors/parse-cancel.json";
        let test_vector_json: String = fs::read_to_string(path).unwrap();

        let test_vector: TestVector = serde_json::from_str(&test_vector_json).unwrap();
        let parsed_cancel = Cancel::from_json_string(&test_vector.input).unwrap();

        let update_exchange_request_body = UpdateExchangeRequestBody {
            message: WalletUpdateMessage::Cancel(parsed_cancel),
        };

        let serialized = update_exchange_request_body.to_json_string().unwrap();
        let deserialized = UpdateExchangeRequestBody::from_json_string(&serialized).unwrap();

        assert_eq!(update_exchange_request_body, deserialized);
    }
}
