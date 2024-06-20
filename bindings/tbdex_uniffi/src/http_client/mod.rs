use crate::{
    errors::Result,
    messages::{close::Close, order::Order, rfq::Rfq, Message, OuterMessage},
    resources::{balance::Balance, offering::Offering},
};
use std::sync::Arc;
use tbdex::http_client::TbdexHttpClient;
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub fn get_offerings(pfi_did: String) -> Result<Vec<Arc<Offering>>> {
    let inner_offerings =
        TbdexHttpClient::get_offerings(pfi_did).map_err(|e| Arc::new(e.into()))?;

    let offerings = inner_offerings
        .into_iter()
        .map(|o| Arc::new(Offering(o)))
        .collect();

    Ok(offerings)
}

pub fn get_balances(pfi_did: String, requestor_did: Arc<BearerDid>) -> Result<Vec<Arc<Balance>>> {
    let inner_balances = TbdexHttpClient::get_balances(pfi_did, requestor_did.0.clone())
        .map_err(|e| Arc::new(e.into()))?;

    let balances = inner_balances
        .into_iter()
        .map(|o| Arc::new(Balance(o)))
        .collect();

    Ok(balances)
}

pub fn create_exchange(rfq: Arc<Rfq>, reply_to: Option<String>) -> Result<()> {
    TbdexHttpClient::create_exchange(rfq.0.clone(), reply_to).map_err(|e| Arc::new(e.into()))?;
    Ok(())
}

pub fn submit_order(order: Arc<Order>) -> Result<()> {
    TbdexHttpClient::submit_order(order.0.clone()).map_err(|e| Arc::new(e.into()))?;
    Ok(())
}

pub fn submit_close(close: Arc<Close>) -> Result<()> {
    TbdexHttpClient::submit_close(close.0.clone()).map_err(|e| Arc::new(e.into()))?;
    Ok(())
}

pub fn get_exchange(
    pfi_did: String,
    requestor_did: Arc<BearerDid>,
    exchange_id: String,
) -> Result<Vec<Arc<dyn Message>>> {
    let inner_messages =
        TbdexHttpClient::get_exchange(pfi_did, requestor_did.0.clone(), exchange_id)
            .map_err(|e| Arc::new(e.into()))?;

    let messages = inner_messages
        .into_iter()
        .map(|m| {
            let outer_message: Arc<dyn Message> = Arc::new(OuterMessage(m));
            outer_message
        })
        .collect();

    Ok(messages)
}

pub fn get_exchanges(pfi_did: String, requestor_did: Arc<BearerDid>) -> Result<Vec<String>> {
    let exchange_ids = TbdexHttpClient::get_exchanges(pfi_did, requestor_did.0.clone())
        .map_err(|e| Arc::new(e.into()))?;
    Ok(exchange_ids)
}
