use crate::{
    errors::{map_err, Result},
    web5::bearer_did::WasmBearerDid,
};
use tbdex::{
    json::{FromJson, ToJson},
    messages::close::{Close, CloseData},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn close_create(
    to: &str,
    from: &str,
    exchange_id: &str,
    data_json: &str,
    protocol: Option<String>,
    external_id: Option<String>,
) -> Result<String> {
    let data = CloseData::from_json_string(data_json).map_err(map_err)?;
    let order =
        Close::create(to, from, exchange_id, &data, protocol, external_id).map_err(map_err)?;
    order.to_json_string().map_err(map_err)
}

#[wasm_bindgen]
pub fn close_sign(close_json: &str, bearer_did: WasmBearerDid) -> Result<String> {
    let mut close = Close::from_json_string(close_json).map_err(map_err)?;
    close.sign(&bearer_did.into()).map_err(map_err)?;
    Ok(close.signature)
}

#[wasm_bindgen]
pub async fn close_verify(close_json: &str) -> Result<()> {
    let close = Close::from_json_string(close_json).map_err(map_err)?;
    close.verify().await.map_err(map_err)
}
