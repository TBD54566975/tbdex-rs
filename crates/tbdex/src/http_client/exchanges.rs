use super::{get_service_endpoint, send_request, Result};
use crate::{
    http_client::{generate_access_token, HttpClientError},
    messages::{
        cancel::Cancel, close::Close, order::Order, order_status::OrderStatus, quote::Quote,
        rfq::Rfq, MessageKind,
    },
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
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
    pub cancel: Option<Cancel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_statuses: Option<Vec<OrderStatus>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close: Option<Close>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateExchangeRequestBody {
    pub message: Rfq,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
}

impl CreateExchangeRequestBody {
    pub fn from_json_string(json: &str) -> Result<Self> {
        let request_body = serde_json::from_str::<Self>(json)?;

        request_body.message.verify()?;

        Ok(request_body)
    }
}

pub fn create_exchange(rfq: &Rfq, reply_to: Option<String>) -> Result<()> {
    let service_endpoint = get_service_endpoint(&rfq.metadata.to)?;
    let create_exchange_endpoint = format!("{}/exchanges", service_endpoint);

    rfq.verify()?;

    send_request::<CreateExchangeRequestBody, ()>(
        &create_exchange_endpoint,
        Method::POST,
        Some(&CreateExchangeRequestBody {
            message: rfq.clone(),
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
    let submit_cancel_endpoint = format!(
        "{}/exchanges/{}",
        service_endpoint, cancel.metadata.exchange_id
    );

    cancel.verify()?;

    send_request::<SubmitCancelRequestBody, ()>(
        &submit_cancel_endpoint,
        Method::PUT,
        Some(&SubmitCancelRequestBody {
            message: cancel.clone(),
        }),
        None,
    )?;

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
            MessageKind::Cancel => {
                let cancel = serde_json::from_value::<Cancel>(message)?;
                cancel.verify()?;
                exchange.cancel = Some(cancel);
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
        }
    }

    Ok(exchange)
}

pub fn get_exchanges(_pfi_did: &str, _requestor_did: &BearerDid) -> Result<Vec<String>> {
    println!("TbdexHttpClient::get_exchanges() invoked");
    Ok(vec![])
}
