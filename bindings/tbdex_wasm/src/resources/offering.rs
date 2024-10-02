use crate::{
    errors::{map_err, Result},
    web5::bearer_did::WasmBearerDid,
};
use tbdex::{
    json::{FromJson, ToJson},
    resources::offering::{Offering, OfferingData},
};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn offering_create(from: &str, data_json: &str, protocol: Option<String>) -> Result<String> {
    let data = OfferingData::from_json_string(data_json).map_err(map_err)?;
    let offering = Offering::create(from, &data, protocol).map_err(map_err)?;
    offering.to_json_string().map_err(map_err)
}

#[wasm_bindgen]
pub fn offering_sign(offering_json: &str, bearer_did: WasmBearerDid) -> Result<String> {
    let mut offering = Offering::from_json_string(offering_json).map_err(map_err)?;
    offering.sign(&bearer_did.into()).map_err(map_err)?;
    Ok(offering.signature)
}

#[wasm_bindgen]
pub async fn offering_verify(offering_json: &str) -> Result<()> {
    let offering = Offering::from_json_string(offering_json).map_err(map_err)?;
    offering.verify().await.map_err(map_err)
}
