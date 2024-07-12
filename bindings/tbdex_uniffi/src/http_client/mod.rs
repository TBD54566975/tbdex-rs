pub mod balances;
pub mod exchanges;
pub mod offerings;

use crate::errors::Result;

pub mod request {
    use super::*;
    use tbdex::{
        http_client::request::{Body as InnerBody, Message},
        messages::MessageKind,
    };

    #[derive(Clone)]
    pub struct BodyData {
        pub json_serialized_message: String,
        pub reply_to: Option<String>,
        // not in APID, but useful on bound side to determine deserialized type for json_serialized_message
        pub kind: MessageKind,
    }

    pub struct Body(pub InnerBody);

    impl Body {
        pub fn new(json_serialized_message: &str, reply_to: Option<String>) -> Result<Self> {
            let message = serde_json::from_str::<Message>(json_serialized_message)?;
            Ok(Self(InnerBody { message, reply_to }))
        }

        pub fn from_json_string(json: &str) -> Result<Self> {
            let inner_body = InnerBody::from_json_string(json)?;
            Ok(Self(inner_body))
        }

        pub fn get_data(&self) -> Result<BodyData> {
            let (kind, json_serialized_message) = match &self.0.message {
                Message::Rfq(rfq) => (MessageKind::Rfq, rfq.to_json()?),
                Message::Quote(quote) => (MessageKind::Quote, quote.to_json()?),
                Message::Order(order) => (MessageKind::Order, order.to_json()?),
                Message::Cancel(cancel) => (MessageKind::Cancel, cancel.to_json()?),
                Message::OrderStatus(order_status) => {
                    (MessageKind::OrderStatus, order_status.to_json()?)
                }
                Message::Close(close) => (MessageKind::Close, close.to_json()?),
                // TODO for error response
                Message::Text(txt) => (MessageKind::Rfq, txt.clone()),
            };

            Ok(BodyData {
                kind,
                json_serialized_message,
                reply_to: self.0.reply_to.clone(),
            })
        }

        pub fn to_json(&self) -> Result<String> {
            Ok(self.0.to_json()?)
        }
    }
}
