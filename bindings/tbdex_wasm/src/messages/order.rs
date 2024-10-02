use crate::{
    errors::{map_err, Result},
    web5::bearer_did::WasmBearerDid,
};
use tbdex::{
    json::{FromJson, ToJson},
    messages::order::Order,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn order_create(
    to: &str,
    from: &str,
    exchange_id: &str,
    protocol: Option<String>,
    external_id: Option<String>,
) -> Result<String> {
    let order = Order::create(to, from, exchange_id, protocol, external_id).map_err(map_err)?;
    order.to_json_string().map_err(map_err)
}

#[wasm_bindgen]
pub fn order_sign(order_json: &str, bearer_did: WasmBearerDid) -> Result<String> {
    let mut order = Order::from_json_string(order_json).map_err(map_err)?;
    order.sign(&bearer_did.into()).map_err(map_err)?;
    Ok(order.signature)
}

#[wasm_bindgen]
pub async fn order_verify(order_json: &str) -> Result<()> {
    let order = Order::from_json_string(order_json).map_err(map_err)?;
    order.verify().await.map_err(map_err)
}
