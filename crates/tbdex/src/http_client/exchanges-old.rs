use super::{get_service_endpoint, send_request, Result};
use crate::{
    http_client::{generate_access_token, HttpClientError},
    messages::{
        cancel::Cancel, close::Close, order::Order, order_status::OrderStatus, quote::Quote,
        rfq::Rfq, MessageKind,
    },
};
use reqwest::Method;
use serde::{de::Visitor, Deserialize, Deserializer, Serialize};
use std::str::FromStr;
use web5::dids::bearer_did::BearerDid;

#[derive(Clone, Default, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Exchange {
    pub rfq: Rfq,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<Quote>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_statuses: Option<Vec<OrderStatus>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close: Option<Close>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel: Option<Cancel>,
}

// #[derive(Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct CreateExchangeRequestBody {
//     pub message: Rfq,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub reply_to: Option<String>,
// }

// impl CreateExchangeRequestBody {
//     pub fn from_json_string(json: &str) -> Result<Self> {
//         let request_body = serde_json::from_str::<Self>(json)?;

//         request_body.message.verify()?;

//         Ok(request_body)
//     }
// }

pub fn create_exchange(rfq: &Rfq, reply_to: Option<String>) -> Result<()> {
    let service_endpoint = get_service_endpoint(&rfq.metadata.to)?;
    let create_exchange_endpoint = format!("{}/exchanges", service_endpoint);

    rfq.verify()?;

    send_request::<HttpBody, ()>(
        &create_exchange_endpoint,
        Method::POST,
        Some(&HttpBody {
            message: Some(Message::Rfq(rfq.clone())),
            reply_to,
        }),
        None,
    )?;

    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct SubmitOrderRequestBody {
    pub message: Order,
}

impl SubmitOrderRequestBody {
    pub fn from_json_string(json: &str) -> Result<Self> {
        let request_body = serde_json::from_str::<Self>(json)?;
        request_body.message.verify()?;
        Ok(request_body)
    }
}

pub fn submit_order(order: &Order) -> Result<()> {
    let service_endpoint = get_service_endpoint(&order.metadata.to)?;
    let submit_order_endpoint = format!(
        "{}/exchanges/{}",
        service_endpoint, order.metadata.exchange_id
    );

    order.verify()?;

    send_request::<SubmitOrderRequestBody, ()>(
        &submit_order_endpoint,
        Method::PUT,
        Some(&SubmitOrderRequestBody {
            message: order.clone(),
        }),
        None,
    )?;

    Ok(())
}

pub fn submit_close(_close: &Close) -> Result<()> {
    println!("TbdexHttpClient::submit_close() invoked");
    Ok(())
}

#[derive(Deserialize)]
struct GetExchangeResponse {
    data: Vec<serde_json::Value>,
}

pub fn get_exchange(
    pfi_did_uri: &str,
    bearer_did: &BearerDid,
    exchange_id: &str,
) -> Result<Exchange> {
    let service_endpoint = get_service_endpoint(pfi_did_uri)?;
    let get_exchange_endpoint = format!("{}/exchanges/{}", service_endpoint, exchange_id);

    let access_token = generate_access_token(pfi_did_uri, bearer_did)?;

    let response = send_request::<(), GetExchangeResponse>(
        &get_exchange_endpoint,
        Method::GET,
        None,
        Some(access_token),
    )?
    .ok_or(HttpClientError::ReqwestError(
        "get exchanges response cannot be null".to_string(),
    ))?;

    let mut exchange = Exchange::default();

    for message in response.data {
        let kind = message
            .get("metadata")
            .and_then(|m| m.get("kind"))
            .and_then(|k| k.as_str())
            .ok_or(HttpClientError::ExchangeMapping)?;

        match MessageKind::from_str(kind)? {
            MessageKind::Rfq => {
                let rfq = serde_json::from_value::<Rfq>(message)?;
                rfq.verify()?;
                exchange.rfq = rfq;
            }
            MessageKind::Quote => {
                let quote = serde_json::from_value::<Quote>(message)?;
                quote.verify()?;
                exchange.quote = Some(quote);
            }
            MessageKind::Order => {
                let order = serde_json::from_value::<Order>(message)?;
                order.verify()?;
                exchange.order = Some(order);
            }
            MessageKind::OrderStatus => {
                let order_status = serde_json::from_value::<OrderStatus>(message)?;
                order_status.verify()?;
                if let Some(order_statuses) = &mut exchange.order_statuses {
                    order_statuses.push(order_status);
                } else {
                    exchange.order_statuses = Some(vec![order_status]);
                }
            }
            MessageKind::Close => {
                let close = serde_json::from_value::<Close>(message)?;
                close.verify()?;
                exchange.close = Some(close);
            }
            MessageKind::Cancel => {
                let cancel = serde_json::from_value::<Cancel>(message)?;
                cancel.verify()?;
                exchange.cancel = Some(cancel);
            }
        }
    }

    Ok(exchange)
}

pub fn get_exchanges(_pfi_did: &str, _requestor_did: &BearerDid) -> Result<Vec<String>> {
    println!("TbdexHttpClient::get_exchanges() invoked");
    Ok(vec![])
}

// TODO how can we support multitype w/ serde for message
#[derive(Serialize, Deserialize)]
pub struct SubmitCancelRequestBody {
    pub message: Cancel,
}

impl SubmitCancelRequestBody {
    pub fn from_json_string(json: &str) -> Result<Self> {
        let request_body = serde_json::from_str::<Self>(json)?;
        request_body.message.verify()?;
        Ok(request_body)
    }
}

pub fn submit_cancel(cancel: &Cancel) -> Result<()> {
    let service_endpoint = get_service_endpoint(&cancel.metadata.to)?;
    let submit_order_endpoint = format!(
        "{}/exchanges/{}",
        service_endpoint, cancel.metadata.exchange_id
    );

    cancel.verify()?;

    send_request::<SubmitCancelRequestBody, ()>(
        &submit_order_endpoint,
        Method::PUT,
        Some(&SubmitCancelRequestBody {
            message: cancel.clone(),
        }),
        None,
    )?;

    Ok(())
}

pub mod http_body {
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct HttpBody {
        #[serde(deserialize_with = "deserialize_message")]
        pub message: Option<Message>,
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
    pub enum Message {
        Rfq(Rfq),
        Order(Order),
        OrderStatus(OrderStatus),
        Close(Close),
        Cancel(Cancel),
        Text(String),
    }

    impl Message {
        pub fn get_rfq(&self) -> Option<&Rfq> {
            if let Message::Rfq(rfq) = self {
                Some(rfq)
            } else {
                None
            }
        }

        pub fn get_order(&self) -> Option<&Order> {
            if let Message::Order(order) = self {
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
    ) -> std::result::Result<Option<Message>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MessageVisitor;

        impl<'de> Visitor<'de> for MessageVisitor {
            type Value = Option<Message>;

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
                    return Ok(Some(Message::Rfq(rfq)));
                }
                if let Ok(order) = serde_json::from_value::<Order>(value.clone()) {
                    return Ok(Some(Message::Order(order)));
                }
                if let Ok(order_status) = serde_json::from_value::<OrderStatus>(value.clone()) {
                    return Ok(Some(Message::OrderStatus(order_status)));
                }
                if let Ok(close) = serde_json::from_value::<Close>(value.clone()) {
                    return Ok(Some(Message::Close(close)));
                }
                if let Ok(cancel) = serde_json::from_value::<Cancel>(value.clone()) {
                    return Ok(Some(Message::Cancel(cancel)));
                }
                if let Ok(text) = serde_json::from_value::<String>(value) {
                    return Ok(Some(Message::Text(text)));
                }

                Err(serde::de::Error::custom("unexpected type"))
            }
        }

        deserializer.deserialize_option(MessageVisitor)
    }
}
