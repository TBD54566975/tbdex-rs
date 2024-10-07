use crate::{
    errors::{map_err, Result},
    web5::bearer_did::WasmBearerDid,
};
use tbdex::{
    json::{FromJson, ToJson},
    messages::order_status::{OrderStatus, OrderStatusData},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn order_status_create(
    to: &str,
    from: &str,
    exchange_id: &str,
    data_json: &str,
    protocol: Option<String>,
    external_id: Option<String>,
) -> Result<String> {
    let data = OrderStatusData::from_json_string(data_json).map_err(map_err)?;
    let order_status = OrderStatus::create(to, from, exchange_id, &data, protocol, external_id)
        .map_err(map_err)?;
    order_status.to_json_string().map_err(map_err)
}

#[wasm_bindgen]
pub fn order_status_sign(order_status_json: &str, bearer_did: WasmBearerDid) -> Result<String> {
    let mut order_status = OrderStatus::from_json_string(order_status_json).map_err(map_err)?;
    order_status.sign(&bearer_did.into()).map_err(map_err)?;
    Ok(order_status.signature)
}

#[wasm_bindgen]
pub async fn order_status_verify(order_status_json: &str) -> Result<()> {
    let order_status = OrderStatus::from_json_string(order_status_json).map_err(map_err)?;
    order_status.verify().await.map_err(map_err)
}
