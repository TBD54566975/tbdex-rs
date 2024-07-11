use crate::{
    errors::Result,
    messages::{close::Close, order::Order, order_status::OrderStatus, quote::Quote, rfq::Rfq},
};
use std::sync::{Arc, RwLock};
use tbdex::http_client::exchanges::{
    http_body::{HttpBody as InnerHttpBody, HttpBodyMessage as InnerHttpBodyMessage},
    Exchange as InnerExchange,
};
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub struct Exchange {
    pub rfq: Arc<Rfq>,
    pub quote: Option<Arc<Quote>>,
    pub order: Option<Arc<Order>>,
    pub order_statuses: Option<Vec<Arc<OrderStatus>>>,
    pub close: Option<Arc<Close>>,
    // todo cancel
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

pub fn submit_close(close: Arc<Close>) -> Result<()> {
    tbdex::http_client::exchanges::submit_close(&close.get_data()?)?;
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

#[derive(Default, Clone)]
pub struct HttpBodyMessageData {
    pub rfq: Option<Arc<Rfq>>,
    pub quote: Option<Arc<Quote>>,
    pub order: Option<Arc<Order>>,
    pub order_statuses: Option<Vec<Arc<OrderStatus>>>,
    pub close: Option<Arc<Close>>,
    // todo cancel
}

impl HttpBodyMessageData {
    fn from_inner(inner: Option<InnerHttpBodyMessage>) -> Option<Self> {
        if let Some(inner_http_body_message) = inner {
            Some(HttpBodyMessageData {
                rfq: if let Some(inner_rfq) = inner_http_body_message.as_rfq() {
                    Some(Arc::new(Rfq::from_inner(inner_rfq.clone())))
                } else {
                    None
                },
                quote: if let Some(inner_quote) = inner_http_body_message.as_quote() {
                    Some(Arc::new(Quote::from_inner(inner_quote.clone())))
                } else {
                    None
                },
                order: None,
                order_statuses: None,
                close: None,
            })
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct HttpBodyData {
    pub message: Option<HttpBodyMessageData>,
    pub reply_to: Option<String>,
}

pub struct HttpBody(pub HttpBodyData);

impl HttpBody {
    pub fn from_json_string(json: &str) -> Result<Self> {
        let inner = InnerHttpBody::from_json_string(json)?;

        Ok(Self(HttpBodyData {
            message: HttpBodyMessageData::from_inner(inner.message),
            reply_to: inner.reply_to,
        }))
    }

    pub fn get_data(&self) -> HttpBodyData {
        self.0.clone()
    }
}
