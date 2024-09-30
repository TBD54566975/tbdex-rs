use crate::{
    errors::Result,
    messages::{
        cancel::Cancel, close::Close, order::Order, order_instructions::OrderInstructions,
        order_status::OrderStatus, quote::Quote, rfq::Rfq,
    },
};
use futures::executor::block_on;
use std::sync::{Arc, RwLock};
use tbdex::http_client::exchanges::{Exchange as InnerExchange, GetExchangeIdsQueryParams};
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub struct Exchange {
    pub rfq: Arc<Rfq>,
    pub quote: Option<Arc<Quote>>,
    pub order: Option<Arc<Order>>,
    pub order_instructions: Option<Arc<OrderInstructions>>,
    pub cancel: Option<Arc<Cancel>>,
    pub order_statuses: Option<Vec<Arc<OrderStatus>>>,
    pub close: Option<Arc<Close>>,
}

impl Exchange {
    pub fn from_inner(inner: InnerExchange) -> Self {
        Self {
            rfq: Arc::new(Rfq(Arc::new(RwLock::new((*inner.rfq).clone())))),
            quote: inner
                .quote
                .map(|q| Arc::new(Quote(Arc::new(RwLock::new((*q).clone()))))),
            order: inner
                .order
                .map(|o| Arc::new(Order(Arc::new(RwLock::new((*o).clone()))))),
            order_instructions: inner
                .order_instructions
                .map(|oi| Arc::new(OrderInstructions(Arc::new(RwLock::new((*oi).clone()))))),
            cancel: inner
                .cancel
                .map(|c| Arc::new(Cancel(Arc::new(RwLock::new((*c).clone()))))),
            order_statuses: inner.order_statuses.map(|oss| {
                oss.into_iter()
                    .map(|os| Arc::new(OrderStatus(Arc::new(RwLock::new((*os).clone())))))
                    .collect()
            }),
            close: inner
                .close
                .map(|c| Arc::new(Close(Arc::new(RwLock::new((*c).clone()))))),
        }
    }
}

pub fn create_exchange(rfq: Arc<Rfq>, reply_to: Option<String>) -> Result<()> {
    block_on(tbdex::http_client::exchanges::create_exchange(
        &rfq.to_inner()?,
        reply_to,
    ))?;
    Ok(())
}

pub fn submit_order(order: Arc<Order>) -> Result<()> {
    block_on(tbdex::http_client::exchanges::submit_order(
        &order.get_data()?,
    ))?;
    Ok(())
}

pub fn submit_cancel(cancel: Arc<Cancel>) -> Result<()> {
    block_on(tbdex::http_client::exchanges::submit_cancel(
        &cancel.get_data()?,
    ))?;
    Ok(())
}

pub fn get_exchange(
    pfi_did_uri: String,
    bearer_did: Arc<BearerDid>,
    exchange_id: String,
) -> Result<Exchange> {
    let inner_exchange = block_on(tbdex::http_client::exchanges::get_exchange(
        &pfi_did_uri,
        &bearer_did.0.clone(),
        &exchange_id,
    ))?;

    Ok(Exchange::from_inner(inner_exchange))
}

pub fn get_exchange_ids(
    pfi_did_uri: String,
    bearer_did: Arc<BearerDid>,
    query_params: Option<GetExchangeIdsQueryParams>,
) -> Result<Vec<String>> {
    let exchange_ids = block_on(tbdex::http_client::exchanges::get_exchange_ids(
        &pfi_did_uri,
        &bearer_did.0.clone(),
        query_params,
    ))?;
    Ok(exchange_ids)
}
