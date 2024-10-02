use crate::{
    errors::{map_err, Result},
    web5::bearer_did::WasmBearerDid,
};
use tbdex::{
    json::{FromJson, ToJson},
    messages::cancel::{Cancel, CancelData},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn cancel_create(
    to: &str,
    from: &str,
    exchange_id: &str,
    data_json: &str,
    protocol: Option<String>,
    external_id: Option<String>,
) -> Result<String> {
    let data = CancelData::from_json_string(data_json).map_err(map_err)?;
    let cancel =
        Cancel::create(to, from, exchange_id, &data, protocol, external_id).map_err(map_err)?;
    cancel.to_json_string().map_err(map_err)
}

#[wasm_bindgen]
pub fn cancel_sign(cancel_json: &str, bearer_did: WasmBearerDid) -> Result<String> {
    let mut cancel = Cancel::from_json_string(cancel_json).map_err(map_err)?;
    cancel.sign(&bearer_did.into()).map_err(map_err)?;
    Ok(cancel.signature)
}

#[wasm_bindgen]
pub async fn cancel_verify(cancel_json: &str) -> Result<()> {
    let cancel = Cancel::from_json_string(cancel_json).map_err(map_err)?;
    cancel.verify().await.map_err(map_err)
}
