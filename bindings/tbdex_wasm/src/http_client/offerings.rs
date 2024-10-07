use crate::errors::{map_err, Result};
use tbdex::json::ToJson;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub async fn get_offerings(pfi_did_uri: &str) -> Result<String> {
    let offerings = tbdex::http_client::offerings::get_offerings(pfi_did_uri)
        .await
        .map_err(map_err)?;
    Ok(offerings.to_json_string().map_err(map_err)?)
}
