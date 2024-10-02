use crate::{
    errors::{map_err, Result},
    resources::offering::WasmOffering,
};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub async fn get_offerings(pfi_did_uri: &str) -> Result<Vec<WasmOffering>> {
    Ok(tbdex::http_client::offerings::get_offerings(pfi_did_uri)
        .await
        .map_err(map_err)?
        .into_iter()
        .map(|offering| offering.into())
        .collect())
}
