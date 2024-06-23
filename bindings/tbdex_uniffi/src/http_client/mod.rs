use crate::{
    errors::Result,
    messages::{close::Close, order::Order, order_status::OrderStatus, quote::Quote, rfq::Rfq},
    resources::{balance::Balance, offering::Offering},
};
use std::sync::{Arc, RwLock};
use tbdex::http_client::Exchange as InnerExchange;
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub struct Exchange {
    pub rfq: Arc<Rfq>,
    pub quote: Option<Arc<Quote>>,
    pub order: Option<Arc<Order>>,
    pub order_statuses: Option<Vec<Arc<OrderStatus>>>,
    pub close: Option<Arc<Close>>,
}

impl Exchange {
    pub fn from_inner(inner: InnerExchange) -> Self {
        Self {
            rfq: Arc::new(Rfq(Arc::new(RwLock::new(inner.rfq.clone())))),
            quote: match &inner.quote {
                None => None,
                Some(q) => Some(Arc::new(Quote(Arc::new(RwLock::new(q.clone()))))),
            },
            order: match &inner.order {
                None => None,
                Some(o) => Some(Arc::new(Order(Arc::new(RwLock::new(o.clone()))))),
            },
            order_statuses: match &inner.order_statuses {
                None => None,
                Some(os) => Some(
                    os.iter()
                        .map(|o| Arc::new(OrderStatus(Arc::new(RwLock::new(o.clone())))))
                        .collect::<Vec<_>>(),
                ),
            },
            close: match &inner.close {
                None => None,
                Some(c) => Some(Arc::new(Close(Arc::new(RwLock::new(c.clone()))))),
            },
        }
    }
}

pub fn get_offerings(pfi_did: String) -> Result<Vec<Arc<Offering>>> {
    let inner_offerings =
        tbdex::http_client::get_offerings(pfi_did).map_err(|e| Arc::new(e.into()))?;

    let offerings = inner_offerings
        .into_iter()
        .map(|o| Arc::new(Offering(Arc::new(RwLock::new(o)))))
        .collect();

    Ok(offerings)
}

pub fn get_balances(pfi_did: String, requestor_did: Arc<BearerDid>) -> Result<Vec<Arc<Balance>>> {
    let inner_balances = tbdex::http_client::get_balances(pfi_did, requestor_did.0.clone())
        .map_err(|e| Arc::new(e.into()))?;

    let balances = inner_balances
        .into_iter()
        .map(|b| Arc::new(Balance(Arc::new(RwLock::new(b)))))
        .collect();

    Ok(balances)
}

pub fn create_exchange(rfq: Arc<Rfq>, reply_to: Option<String>) -> Result<()> {
    tbdex::http_client::create_exchange(rfq.to_inner()?, reply_to)
        .map_err(|e| Arc::new(e.into()))?;
    Ok(())
}

pub fn submit_order(order: Arc<Order>) -> Result<()> {
    tbdex::http_client::submit_order(order.get_data()?).map_err(|e| Arc::new(e.into()))?;
    Ok(())
}

pub fn submit_close(close: Arc<Close>) -> Result<()> {
    tbdex::http_client::submit_close(close.get_data()?).map_err(|e| Arc::new(e.into()))?;
    Ok(())
}

pub fn get_exchange(
    pfi_did: String,
    requestor_did: Arc<BearerDid>,
    exchange_id: String,
) -> Result<Exchange> {
    let inner_exchange =
        tbdex::http_client::get_exchange(pfi_did, requestor_did.0.clone(), exchange_id)
            .map_err(|e| Arc::new(e.into()))?;

    Ok(Exchange::from_inner(inner_exchange))
}

pub fn get_exchanges(pfi_did: String, requestor_did: Arc<BearerDid>) -> Result<Vec<String>> {
    let exchange_ids = tbdex::http_client::get_exchanges(pfi_did, requestor_did.0.clone())
        .map_err(|e| Arc::new(e.into()))?;
    Ok(exchange_ids)
}
