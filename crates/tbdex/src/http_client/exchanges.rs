use std::sync::Arc;

use super::{add_pagination, get_json, get_service_endpoint, post_json, put_json, Result};
use crate::http::exchanges::GetExchangesResponseBody;
use crate::json::{FromJson, ToJson};
use crate::{
    http::exchanges::{
        CreateExchangeRequestBody, GetExchangeResponseBody, UpdateExchangeRequestBody,
        WalletUpdateMessage,
    },
    http_client::generate_access_token,
    messages::{
        cancel::Cancel, close::Close, order::Order, order_instructions::OrderInstructions,
        order_status::OrderStatus, quote::Quote, rfq::Rfq, Message,
    },
};
use serde::{Deserialize, Serialize};
use web5::dids::bearer_did::BearerDid;

#[derive(Clone, Default, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Exchange {
    pub rfq: Arc<Rfq>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<Arc<Quote>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Arc<Order>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_instructions: Option<Arc<OrderInstructions>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel: Option<Arc<Cancel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_statuses: Option<Vec<Arc<OrderStatus>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close: Option<Arc<Close>>,
}

impl FromJson for Exchange {}
impl ToJson for Exchange {}

pub async fn create_exchange(rfq: &Rfq, reply_to: Option<String>) -> Result<()> {
    let service_endpoint = get_service_endpoint(&rfq.metadata.to).await?;
    let create_exchange_endpoint = format!("{}/exchanges", service_endpoint);

    rfq.verify().await?;

    post_json(
        &create_exchange_endpoint,
        &CreateExchangeRequestBody {
            message: rfq.clone(),
            reply_to,
        },
    )
    .await?;

    Ok(())
}

pub async fn submit_order(order: &Order) -> Result<()> {
    let service_endpoint = get_service_endpoint(&order.metadata.to).await?;
    let submit_order_endpoint = format!(
        "{}/exchanges/{}",
        service_endpoint, order.metadata.exchange_id
    );

    order.verify().await?;

    put_json(
        &submit_order_endpoint,
        &UpdateExchangeRequestBody {
            message: WalletUpdateMessage::Order(Arc::new(order.clone())),
        },
    )
    .await?;

    Ok(())
}

pub async fn submit_cancel(cancel: &Cancel) -> Result<()> {
    let service_endpoint = get_service_endpoint(&cancel.metadata.to).await?;
    let submit_cancel_endpoint = format!(
        "{}/exchanges/{}",
        service_endpoint, cancel.metadata.exchange_id
    );

    cancel.verify().await?;

    put_json(
        &submit_cancel_endpoint,
        &UpdateExchangeRequestBody {
            message: WalletUpdateMessage::Cancel(Arc::new(cancel.clone())),
        },
    )
    .await?;

    Ok(())
}

pub async fn get_exchange(
    pfi_did_uri: &str,
    bearer_did: &BearerDid,
    exchange_id: &str,
) -> Result<Exchange> {
    let service_endpoint = get_service_endpoint(pfi_did_uri).await?;
    let get_exchange_endpoint = format!("{}/exchanges/{}", service_endpoint, exchange_id);

    let access_token = generate_access_token(pfi_did_uri, bearer_did)?;
    let get_exchange_response_body =
        get_json::<GetExchangeResponseBody>(&get_exchange_endpoint, Some(access_token)).await?;

    let mut exchange = Exchange::default();

    for message in get_exchange_response_body.data {
        match message {
            Message::Rfq(rfq) => {
                exchange.rfq = rfq;
            }
            Message::Quote(quote) => {
                exchange.quote = Some(quote);
            }
            Message::Order(order) => {
                exchange.order = Some(order);
            }
            Message::OrderInstructions(order_instructions) => {
                exchange.order_instructions = Some(order_instructions);
            }
            Message::Cancel(cancel) => {
                exchange.cancel = Some(cancel);
            }
            Message::OrderStatus(order_status) => {
                if let Some(order_statuses) = &mut exchange.order_statuses {
                    order_statuses.push(order_status);
                } else {
                    exchange.order_statuses = Some(vec![order_status]);
                }
            }
            Message::Close(close) => {
                exchange.close = Some(close);
            }
        }
    }

    Ok(exchange)
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct GetExchangeIdsQueryParams {
    pub pagination_offset: Option<i64>,
    pub pagination_limit: Option<i64>,
}

pub async fn get_exchange_ids(
    pfi_did: &str,
    requestor_did: &BearerDid,
    query_params: Option<GetExchangeIdsQueryParams>,
) -> Result<Vec<String>> {
    let service_endpoint = get_service_endpoint(pfi_did).await?;
    let get_exchanges_endpoint = format!("{}/exchanges", service_endpoint);

    let get_exchanges_endpoint = if let Some(params) = query_params {
        add_pagination(
            &get_exchanges_endpoint,
            params.pagination_offset,
            params.pagination_limit,
        )
    } else {
        get_exchanges_endpoint
    };

    let access_token = generate_access_token(pfi_did, requestor_did)?;
    let get_exchanges_response_body =
        get_json::<GetExchangesResponseBody>(&get_exchanges_endpoint, Some(access_token)).await?;

    Ok(get_exchanges_response_body.data)
}
