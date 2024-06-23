use web5::apid::dids::bearer_did::BearerDid;

use crate::{
    messages::{close::Close, order::Order, order_status::OrderStatus, quote::Quote, rfq::Rfq},
    resources::{balance::Balance, offering::Offering},
};

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum TbdexHttpClientError {
    #[error("unknown -- temporarily stubbed in")]
    UnknownError,
}

type Result<T> = std::result::Result<T, TbdexHttpClientError>;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Exchange {
    pub rfq: Rfq,
    pub quote: Option<Quote>,
    pub order: Option<Order>,
    pub order_statuses: Option<Vec<OrderStatus>>,
    pub close: Option<Close>,
}

pub fn get_offerings(_pfi_did: String) -> Result<Vec<Offering>> {
    println!("TbdexHttpClient::get_offerings() invoked");
    Ok(vec![])
}

pub fn get_balances(_pfi_did: String, _requestor_did: BearerDid) -> Result<Vec<Balance>> {
    println!("TbdexHttpClient::get_balances() invoked");
    Ok(vec![])
}

pub fn create_exchange(_rfq: Rfq, _reply_to: Option<String>) -> Result<()> {
    println!("TbdexHttpClient::create_exchange() invoked");
    Ok(())
}

pub fn submit_order(_order: Order) -> Result<()> {
    println!("TbdexHttpClient::submit_order() invoked");
    Ok(())
}

pub fn submit_close(_close: Close) -> Result<()> {
    println!("TbdexHttpClient::submit_close() invoked");
    Ok(())
}

pub fn get_exchange(
    _pfi_did: String,
    _requestor_did: BearerDid,
    _exchange_id: String,
) -> Result<Exchange> {
    println!("TbdexHttpClient::get_exchange() invoked");
    Ok(Exchange::default())
}

pub fn get_exchanges(_pfi_did: String, _requestor_did: BearerDid) -> Result<Vec<String>> {
    println!("TbdexHttpClient::get_exchanges() invoked");
    Ok(vec![])
}
