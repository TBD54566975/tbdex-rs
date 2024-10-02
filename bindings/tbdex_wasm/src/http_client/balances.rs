use crate::{
    errors::{map_err, Result},
    web5::bearer_did::WasmBearerDid,
};
use tbdex::json::ToJson;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub async fn get_balances(pfi_did_uri: &str, bearer_did: WasmBearerDid) -> Result<String> {
    let balances = tbdex::http_client::balances::get_balances(pfi_did_uri, &bearer_did.into())
        .await
        .map_err(map_err)?;

    let mut json = "[".to_string();

    for balance in balances {
        json += &balance.to_json_string().map_err(map_err)?;
    }

    json += "]";

    Ok(json)
}
