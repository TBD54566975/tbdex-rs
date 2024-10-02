use crate::{
    errors::{map_err, Result},
    web5::bearer_did::WasmBearerDid,
};
use tbdex::{
    json::{FromJson, ToJson},
    messages::order_instructions::{OrderInstructions, OrderInstructionsData},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn order_instructions_create(
    to: &str,
    from: &str,
    exchange_id: &str,
    data_json: &str,
    protocol: Option<String>,
    external_id: Option<String>,
) -> Result<String> {
    let data = OrderInstructionsData::from_json_string(data_json).map_err(map_err)?;
    let order_instructions =
        OrderInstructions::create(to, from, exchange_id, &data, protocol, external_id)
            .map_err(map_err)?;
    order_instructions.to_json_string().map_err(map_err)
}

#[wasm_bindgen]
pub fn order_instructions_sign(
    order_instructions_json: &str,
    bearer_did: WasmBearerDid,
) -> Result<String> {
    let mut order_instructions =
        OrderInstructions::from_json_string(order_instructions_json).map_err(map_err)?;
    order_instructions
        .sign(&bearer_did.into())
        .map_err(map_err)?;
    Ok(order_instructions.signature)
}

#[wasm_bindgen]
pub async fn order_instructions_verify(order_instructions_json: &str) -> Result<()> {
    let order_instructions =
        OrderInstructions::from_json_string(order_instructions_json).map_err(map_err)?;
    order_instructions.verify().await.map_err(map_err)
}
