use super::{get_service_endpoint, Result};
use crate::{
    http_client::{generate_access_token, HttpClientError},
    messages::{
        close::Close, order::Order, order_status::OrderStatus, quote::Quote, rfq::Rfq, MessageKind,
    },
};
use reqwest::blocking::Client;
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
    pub order_statuses: Option<Vec<OrderStatus>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close: Option<Close>,
}

impl Exchange {
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self)?)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateExchangeRequestBody {
    pub rfq: Rfq,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
}

impl CreateExchangeRequestBody {
    pub fn from_json_string(json: &str) -> Result<Self> {
        let request_body = serde_json::from_str::<Self>(json)?;

        request_body.rfq.verify()?;

        Ok(request_body)
    }
}

pub fn create_exchange(rfq: &Rfq, reply_to: Option<String>) -> Result<()> {
    let service_endpoint = get_service_endpoint(&rfq.metadata.to)?;
    let create_exchange_endpoint = format!("{}/exchanges", service_endpoint);

    rfq.verify()?;

    let request_body = serde_json::to_string(&CreateExchangeRequestBody {
        rfq: rfq.clone(),
        reply_to,
    })?;

    // todo handle error responses response.status() and response.text()
    let _response = Client::new()
        .post(create_exchange_endpoint)
        .header("Content-Type", "application/json")
        .body(request_body)
        .send()?;

    Ok(())
}

pub fn submit_order(order: &Order) -> Result<()> {
    let service_endpoint = get_service_endpoint(&order.metadata.to)?;
    let submit_order_endpoint = format!(
        "{}/exchanges/{}",
        service_endpoint, order.metadata.exchange_id
    );

    order.verify()?;

    let request_body = serde_json::to_string(&order)?;
    // todo handle error responses response.status() and response.text()
    let _response = Client::new()
        .put(submit_order_endpoint)
        .header("Content-Type", "application/json")
        .body(request_body)
        .send()?;

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

    let response = Client::new()
        .get(get_exchange_endpoint)
        .bearer_auth(access_token)
        .send()?
        .text()?;

    // TODO handle error response

    let mut exchange = Exchange::default();

    let data = serde_json::from_str::<GetExchangeResponse>(&response)?.data;
    for message in data {
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
        }
    }

    Ok(exchange)
}

pub fn get_exchanges(_pfi_did: &str, _requestor_did: &BearerDid) -> Result<Vec<String>> {
    println!("TbdexHttpClient::get_exchanges() invoked");
    Ok(vec![])
}
