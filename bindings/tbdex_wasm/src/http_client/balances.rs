use crate::{
    errors::{map_err, Result},
    resources::balance::WasmBalance,
    web5::bearer_did::WasmBearerDid,
};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub async fn get_balances(
    pfi_did_uri: &str,
    bearer_did: WasmBearerDid,
) -> Result<Vec<WasmBalance>> {
    Ok(
        tbdex::http_client::balances::get_balances(pfi_did_uri, &bearer_did.into())
            .await
            .map_err(map_err)?
            .into_iter()
            .map(|balance| balance.into())
            .collect(),
    )
}
