use crate::{
    errors::{map_err, Result},
    web5::bearer_did::WasmBearerDid,
};
use tbdex::{
    json::{FromJson, ToJson},
    messages::quote::{Quote, QuoteData},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn quote_create(
    to: &str,
    from: &str,
    exchange_id: &str,
    data_json: &str,
    protocol: Option<String>,
    external_id: Option<String>,
) -> Result<String> {
    let data = QuoteData::from_json_string(data_json).map_err(map_err)?;
    let quote =
        Quote::create(to, from, exchange_id, &data, protocol, external_id).map_err(map_err)?;
    quote.to_json_string().map_err(map_err)
}

#[wasm_bindgen]
pub fn quote_sign(quote_json: &str, bearer_did: WasmBearerDid) -> Result<String> {
    let mut quote = Quote::from_json_string(quote_json).map_err(map_err)?;
    quote.sign(&bearer_did.into()).map_err(map_err)?;
    Ok(quote.signature)
}

#[wasm_bindgen]
pub async fn quote_verify(quote_json: &str) -> Result<()> {
    let quote = Quote::from_json_string(quote_json).map_err(map_err)?;
    quote.verify().await.map_err(map_err)
}
