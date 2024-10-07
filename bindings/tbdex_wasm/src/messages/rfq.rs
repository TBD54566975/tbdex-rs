use crate::{
    errors::{map_err, Result},
    web5::bearer_did::WasmBearerDid,
};
use tbdex::{
    json::{FromJson, ToJson},
    messages::rfq::{CreateRfqData, Rfq},
    resources::offering::Offering,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn rfq_create(
    to: &str,
    from: &str,
    create_rfq_data_json: &str,
    protocol: Option<String>,
    external_id: Option<String>,
) -> Result<String> {
    let create_rfq_data = CreateRfqData::from_json_string(create_rfq_data_json).map_err(map_err)?;
    let rfq = Rfq::create(to, from, &create_rfq_data, protocol, external_id).map_err(map_err)?;
    rfq.to_json_string().map_err(map_err)
}

#[wasm_bindgen]
pub fn rfq_sign(rfq_json: &str, bearer_did: WasmBearerDid) -> Result<String> {
    let mut rfq = Rfq::from_json_string(rfq_json).map_err(map_err)?;
    rfq.sign(&bearer_did.into()).map_err(map_err)?;
    Ok(rfq.signature)
}

#[wasm_bindgen]
pub async fn rfq_verify(rfq_json: &str) -> Result<()> {
    let rfq = Rfq::from_json_string(rfq_json).map_err(map_err)?;
    rfq.verify().await.map_err(map_err)
}

#[wasm_bindgen]
pub async fn rfq_verify_offering_requirements(rfq_json: &str, offering_json: &str) -> Result<()> {
    let rfq = Rfq::from_json_string(rfq_json).map_err(map_err)?;
    let offering = Offering::from_json_string(offering_json).map_err(map_err)?;
    rfq.verify_offering_requirements(&offering)
        .await
        .map_err(map_err)
}

#[wasm_bindgen]
pub fn rfq_verify_all_private_data(rfq_json: &str) -> Result<()> {
    let rfq = Rfq::from_json_string(rfq_json).map_err(map_err)?;
    rfq.verify_all_private_data().map_err(map_err)
}

#[wasm_bindgen]
pub fn rfq_verify_present_private_data(rfq_json: &str) -> Result<()> {
    let rfq = Rfq::from_json_string(rfq_json).map_err(map_err)?;
    rfq.verify_present_private_data().map_err(map_err)
}
