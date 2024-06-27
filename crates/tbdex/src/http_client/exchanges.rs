use super::Result;
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

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
struct CreateExchangeRequest {
    rfq: Rfq,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to: Option<String>,
}

pub fn create_exchange(rfq: &Rfq, reply_to: Option<String>) -> Result<()> {
    // TODO resolve pfi did for service endpoint; waiting on did:dht resolution
    let endpoint = "http://localhost:9000/exchanges";
    // TODO the above

    // TODO uncomment with did:dht resolution support
    // rfq.verify()?;

    let request_body = serde_json::to_string(&CreateExchangeRequest {
        rfq: rfq.clone(),
        reply_to,
    })?;

    // todo handle error responses response.status() and response.text()
    let _response = Client::new()
        .post(endpoint)
        .header("Content-Type", "application/json")
        .body(request_body)
        .send()?;

    Ok(())
}

pub fn submit_order(order: &Order) -> Result<()> {
    // TODO resolve pfi did for service endpoint; waiting on did:dht resolution
    let endpoint = format!(
        "http://localhost:9000/exchanges/{}",
        order.metadata.exchange_id
    );
    // TODO the above

    // TODO uncomment with did:dht resolution support
    // order.verify()?;

    let request_body = serde_json::to_string(&order)?;
    // todo handle error responses response.status() and response.text()
    let _response = Client::new()
        .put(endpoint)
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
    // TODO resolve pfi did for service endpoint; waiting on did:dht resolution
    let endpoint = format!("http://localhost:9000/exchanges/{}", exchange_id);
    // TODO the above

    let access_token = generate_access_token(pfi_did_uri, bearer_did)?;

    let response = Client::new()
        .get(endpoint)
        .bearer_auth(access_token)
        .send()?
        .text()?;

    // TODO handle error response

    let mut exchange = Exchange::default();

    // TODO cryptographic verifications

    let data = serde_json::from_str::<GetExchangeResponse>(&response)?.data;
    for message in data {
        let kind = message
            .get("metadata")
            .and_then(|m| m.get("kind"))
            .and_then(|k| k.as_str())
            .ok_or(HttpClientError::ExchangeMapping)?;

        match MessageKind::from_str(kind)? {
            MessageKind::Rfq => exchange.rfq = serde_json::from_value(message)?,
            MessageKind::Quote => exchange.quote = Some(serde_json::from_value(message)?),
            MessageKind::Order => exchange.order = Some(serde_json::from_value(message)?),
            MessageKind::OrderStatus => {
                let order_status = serde_json::from_value::<OrderStatus>(message)?;
                if let Some(order_statuses) = &mut exchange.order_statuses {
                    order_statuses.push(order_status);
                } else {
                    exchange.order_statuses = Some(vec![order_status]);
                }
            }
            MessageKind::Close => exchange.close = Some(serde_json::from_value(message)?),
        }
    }

    Ok(exchange)
}

pub fn get_exchanges(_pfi_did: &str, _requestor_did: &BearerDid) -> Result<Vec<String>> {
    println!("TbdexHttpClient::get_exchanges() invoked");
    Ok(vec![])
}
