use crate::{
    errors::{map_err, Result},
    web5::bearer_did::WasmBearerDid,
};
use tbdex::{
    http_client::exchanges::GetExchangeIdsQueryParams,
    json::{FromJson, ToJson},
    messages::{cancel::Cancel, order::Order, rfq::Rfq},
};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub async fn create_exchange(rfq_json: &str, reply_to: Option<String>) -> Result<()> {
    let rfq = Rfq::from_json_string(rfq_json).map_err(map_err)?;
    tbdex::http_client::exchanges::create_exchange(&rfq.into(), reply_to)
        .await
        .map_err(map_err)
}

#[wasm_bindgen]
pub async fn submit_order(order_json: &str) -> Result<()> {
    let order = Order::from_json_string(order_json).map_err(map_err)?;
    tbdex::http_client::exchanges::submit_order(&order)
        .await
        .map_err(map_err)
}

#[wasm_bindgen]
pub async fn submit_cancel(cancel_json: &str) -> Result<()> {
    let cancel = Cancel::from_json_string(cancel_json).map_err(map_err)?;
    tbdex::http_client::exchanges::submit_cancel(&cancel)
        .await
        .map_err(map_err)
}

#[wasm_bindgen]
pub async fn get_exchange(
    pfi_did_uri: &str,
    bearer_did: WasmBearerDid,
    exchange_id: &str,
) -> Result<String> {
    tbdex::http_client::exchanges::get_exchange(pfi_did_uri, &bearer_did.into(), exchange_id)
        .await
        .map_err(map_err)?
        .to_json_string()
        .map_err(map_err)
}

#[wasm_bindgen]
pub async fn get_exchange_ids(
    pfi_did_uri: &str,
    requestor_did: WasmBearerDid,
    pagination_offset: Option<i64>,
    pagination_limit: Option<i64>,
) -> Result<Vec<String>> {
    tbdex::http_client::exchanges::get_exchange_ids(
        pfi_did_uri,
        &requestor_did.into(),
        Some(GetExchangeIdsQueryParams {
            pagination_offset,
            pagination_limit,
        }),
    )
    .await
    .map_err(map_err)
}
