use super::Result;
use crate::messages::{
    cancel::Cancel, close::Close, order::Order, order_status::OrderStatus, quote::Quote, rfq::Rfq,
};
use serde::{de::Visitor, Deserialize, Deserializer, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct HttpBody {
    #[serde(deserialize_with = "deserialize_message")]
    pub message: Option<HttpBodyMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    // todo error response
}

impl HttpBody {
    pub fn from_json_string(json: &str) -> Result<Self> {
        let request_body = serde_json::from_str::<Self>(json)?;
        if let Some(message) = &request_body.message {
            message.verify()?;
        }
        Ok(request_body)
    }
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum HttpBodyMessage {
    Rfq(Rfq),
    Quote(Quote),
    Order(Order),
    OrderStatus(OrderStatus),
    Close(Close),
    Cancel(Cancel),
    Text(String),
}

impl HttpBodyMessage {
    pub fn as_rfq(&self) -> Option<&Rfq> {
        if let HttpBodyMessage::Rfq(rfq) = self {
            Some(rfq)
        } else {
            None
        }
    }

    pub fn as_quote(&self) -> Option<&Quote> {
        if let HttpBodyMessage::Quote(quote) = self {
            Some(quote)
        } else {
            None
        }
    }

    pub fn as_order(&self) -> Option<&Order> {
        if let HttpBodyMessage::Order(order) = self {
            Some(order)
        } else {
            None
        }
    }

    // TODO the other ones

    pub fn verify(&self) -> Result<()> {
        unimplemented!()
    }
}

fn deserialize_message<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<HttpBodyMessage>, D::Error>
where
    D: Deserializer<'de>,
{
    struct MessageVisitor;

    impl<'de> Visitor<'de> for MessageVisitor {
        type Value = Option<HttpBodyMessage>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an Rfq, Order, OrderStatus, Close, Cancel, or String")
        }

        fn visit_none<E>(self) -> std::result::Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }

        fn visit_some<D>(self, deserializer: D) -> std::result::Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            let value: serde_json::Value = Deserialize::deserialize(deserializer)?;

            if let Ok(rfq) = serde_json::from_value::<Rfq>(value.clone()) {
                return Ok(Some(HttpBodyMessage::Rfq(rfq)));
            }
            if let Ok(order) = serde_json::from_value::<Order>(value.clone()) {
                return Ok(Some(HttpBodyMessage::Order(order)));
            }
            if let Ok(order_status) = serde_json::from_value::<OrderStatus>(value.clone()) {
                return Ok(Some(HttpBodyMessage::OrderStatus(order_status)));
            }
            if let Ok(close) = serde_json::from_value::<Close>(value.clone()) {
                return Ok(Some(HttpBodyMessage::Close(close)));
            }
            if let Ok(cancel) = serde_json::from_value::<Cancel>(value.clone()) {
                return Ok(Some(HttpBodyMessage::Cancel(cancel)));
            }
            if let Ok(text) = serde_json::from_value::<String>(value) {
                return Ok(Some(HttpBodyMessage::Text(text)));
            }

            Err(serde::de::Error::custom("unexpected type"))
        }
    }

    deserializer.deserialize_option(MessageVisitor)
}
