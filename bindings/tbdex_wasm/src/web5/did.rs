use crate::errors::{map_err, map_web5_err, Result};
use wasm_bindgen::prelude::wasm_bindgen;
use web5::dids::did::Did;

#[wasm_bindgen]
pub fn parse_did(uri: &str) -> Result<String> {
    let did = Did::parse(uri).map_err(map_web5_err)?;
    let did_json = serde_json::to_string(&did).map_err(|e| map_err(e.into()))?;
    Ok(did_json)
}
