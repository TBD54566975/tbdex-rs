use crate::{
    errors::Result,
    messages::{close::Close, order::Order, order_status::OrderStatus, quote::Quote, rfq::Rfq},
};
use std::sync::{Arc, RwLock};
use tbdex::http_client::exchanges::{
    CreateExchangeRequestBody as InnerCreateExchangeRequestBody, Exchange as InnerExchange,
};
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

#[derive(Clone)]
pub struct ExchangeData {
    pub rfq: Arc<Rfq>,
    pub quote: Option<Arc<Quote>>,
    pub order: Option<Arc<Order>>,
    pub order_statuses: Option<Vec<Arc<OrderStatus>>>,
    pub close: Option<Arc<Close>>,
}

pub struct Exchange(pub ExchangeData);

impl Exchange {
    pub fn from_inner(inner: InnerExchange) -> Self {
        Self(ExchangeData {
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
        })
    }

    pub fn to_json(&self) -> Result<String> {
        let inner_exchange = InnerExchange {
            rfq: self.0.rfq.to_inner()?,
            quote: match &self.0.quote {
                None => None,
                Some(q) => Some(q.to_inner()?),
            },
            order: match &self.0.order {
                None => None,
                Some(o) => Some(o.to_inner()?),
            },
            order_statuses: match &self.0.order_statuses {
                None => None,
                Some(os) => {
                    let order_statuses: Result<Vec<_>> =
                        os.iter().map(|os| os.to_inner()).collect();
                    Some(order_statuses?)
                }
            },
            close: match &self.0.close {
                None => None,
                Some(c) => Some(c.to_inner()?),
            },
        };

        Ok(inner_exchange.to_json()?)
    }

    pub fn get_data(&self) -> ExchangeData {
        self.0.clone()
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
) -> Result<Arc<Exchange>> {
    let inner_exchange = tbdex::http_client::exchanges::get_exchange(
        &pfi_did_uri,
        &bearer_did.0.clone(),
        &exchange_id,
    )?;

    Ok(Arc::new(Exchange::from_inner(inner_exchange)))
}

pub fn get_exchanges(pfi_did_uri: String, bearer_did: Arc<BearerDid>) -> Result<Vec<String>> {
    let exchange_ids =
        tbdex::http_client::exchanges::get_exchanges(&pfi_did_uri, &bearer_did.0.clone())?;
    Ok(exchange_ids)
}

#[derive(Clone)]
pub struct CreateExchangeRequestBodyData {
    pub rfq: Arc<Rfq>,
    pub reply_to: Option<String>,
}

pub struct CreateExchangeRequestBody(pub CreateExchangeRequestBodyData);

impl CreateExchangeRequestBody {
    pub fn from_json_string(json: &str) -> Result<Self> {
        let inner = InnerCreateExchangeRequestBody::from_json_string(json)?;
        let rfq = Rfq::from_inner(inner.rfq);
        Ok(Self(CreateExchangeRequestBodyData {
            rfq: Arc::new(rfq),
            reply_to: inner.reply_to,
        }))
    }

    pub fn get_data(&self) -> CreateExchangeRequestBodyData {
        self.0.clone()
    }
}
