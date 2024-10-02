use crate::{
    errors::{map_err, Result},
    web5::bearer_did::WasmBearerDid,
};
use tbdex::{
    json::{FromJson, ToJson},
    resources::balance::{Balance, BalanceData},
};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn balance_create(from: &str, data_json: &str, protocol: Option<String>) -> Result<String> {
    let data = BalanceData::from_json_string(data_json).map_err(map_err)?;
    let balance = Balance::create(from, &data, protocol).map_err(map_err)?;
    Ok(balance.to_json_string().map_err(map_err)?)
}

#[wasm_bindgen]
pub fn balance_sign(balance_json: &str, bearer_did: WasmBearerDid) -> Result<String> {
    let mut balance = Balance::from_json_string(balance_json).map_err(map_err)?;
    balance.sign(&bearer_did.into()).map_err(map_err)?;
    Ok(balance.signature)
}

#[wasm_bindgen]
pub async fn balance_verify(balance_json: &str) -> Result<()> {
    let balance = Balance::from_json_string(balance_json).map_err(map_err)?;
    balance.verify().await.map_err(map_err)
}
