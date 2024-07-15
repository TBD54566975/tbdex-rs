use crate::{
    errors::Result,
    messages::{
        cancel::Cancel, close::Close, order::Order, order_status::OrderStatus, quote::Quote,
        rfq::Rfq,
    },
};
use std::sync::{Arc, RwLock};
use tbdex::http_client::exchanges::{
    Exchange as InnerExchange, SubmitCancelRequestBody as InnerSubmitCancelRequestBody,
    SubmitOrderRequestBody as InnerSubmitOrderRequestBody,
};
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub struct Exchange {
    pub rfq: Arc<Rfq>,
    pub quote: Option<Arc<Quote>>,
    pub order: Option<Arc<Order>>,
    pub cancel: Option<Arc<Cancel>>,
    pub order_statuses: Option<Vec<Arc<OrderStatus>>>,
    pub close: Option<Arc<Close>>,
}

impl Exchange {
    pub fn from_inner(inner: InnerExchange) -> Self {
        Self {
            rfq: Arc::new(Rfq(Arc::new(RwLock::new(inner.rfq.clone())))),
            quote: inner
                .quote
                .as_ref()
                .map(|q| Arc::new(Quote(Arc::new(RwLock::new(q.clone()))))),
            order: inner
                .order
                .as_ref()
                .map(|o| Arc::new(Order(Arc::new(RwLock::new(o.clone()))))),
            cancel: inner
                .cancel
                .as_ref()
                .map(|o| Arc::new(Cancel(Arc::new(RwLock::new(o.clone()))))),
            order_statuses: inner.order_statuses.as_ref().map(|os| {
                os.iter()
                    .map(|o| Arc::new(OrderStatus(Arc::new(RwLock::new(o.clone())))))
                    .collect::<Vec<_>>()
            }),
            close: inner
                .close
                .as_ref()
                .map(|c| Arc::new(Close(Arc::new(RwLock::new(c.clone()))))),
        }
    }
}

pub fn create_exchange(rfq: Arc<Rfq>, reply_to: Option<String>) -> Result<()> {
    tbdex::http_client::exchanges::create_exchange(&rfq.to_inner()?, reply_to)?;
    Ok(())
}

pub fn submit_order(order: Arc<Order>) -> Result<()> {
    tbdex::http_client::exchanges::submit_order(&order.get_data()?)?;
    Ok(())
}

pub fn submit_cancel(cancel: Arc<Cancel>) -> Result<()> {
    tbdex::http_client::exchanges::submit_cancel(&cancel.get_data()?)?;
    Ok(())
}

pub fn get_exchange(
    pfi_did_uri: String,
    bearer_did: Arc<BearerDid>,
    exchange_id: String,
) -> Result<Exchange> {
    let inner_exchange = tbdex::http_client::exchanges::get_exchange(
        &pfi_did_uri,
        &bearer_did.0.clone(),
        &exchange_id,
    )?;

    Ok(Exchange::from_inner(inner_exchange))
}

pub fn get_exchanges(pfi_did_uri: String, bearer_did: Arc<BearerDid>) -> Result<Vec<String>> {
    let exchange_ids =
        tbdex::http_client::exchanges::get_exchanges(&pfi_did_uri, &bearer_did.0.clone())?;
    Ok(exchange_ids)
}

#[derive(Clone)]
pub struct SubmitOrderRequestBodyData {
    pub message: Arc<Order>,
}

pub struct SubmitOrderRequestBody(pub SubmitOrderRequestBodyData);

impl SubmitOrderRequestBody {
    pub fn from_json_string(json: &str) -> Result<Self> {
        let inner = InnerSubmitOrderRequestBody::from_json_string(json)?;
        let message = Order::from_inner(inner.message);
        Ok(Self(SubmitOrderRequestBodyData {
            message: Arc::new(message),
        }))
    }

    pub fn get_data(&self) -> SubmitOrderRequestBodyData {
        self.0.clone()
    }
}

#[derive(Clone)]
pub struct SubmitCancelRequestBodyData {
    pub message: Arc<Cancel>,
}

pub struct SubmitCancelRequestBody(pub SubmitCancelRequestBodyData);

impl SubmitCancelRequestBody {
    pub fn from_json_string(json: &str) -> Result<Self> {
        let inner = InnerSubmitCancelRequestBody::from_json_string(json)?;
        let message = Cancel::from_inner(inner.message);
        Ok(Self(SubmitCancelRequestBodyData {
            message: Arc::new(message),
        }))
    }

    pub fn get_data(&self) -> SubmitCancelRequestBodyData {
        self.0.clone()
    }
}
