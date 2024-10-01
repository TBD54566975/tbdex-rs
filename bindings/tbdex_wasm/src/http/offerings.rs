use crate::{
    errors::{map_err, Result},
    resources::offering::WasmOffering,
};
use tbdex::{
    http::offerings::GetOfferingsResponseBody,
    json::{FromJson, ToJson},
};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct WasmGetOfferingsResponseBody {
    inner: GetOfferingsResponseBody,
}

#[wasm_bindgen]
impl WasmGetOfferingsResponseBody {
    #[wasm_bindgen(constructor)]
    pub fn new(data: Vec<WasmOffering>) -> Self {
        Self {
            inner: GetOfferingsResponseBody {
                data: data.into_iter().map(|o| o.into()).collect(),
            },
        }
    }

    #[wasm_bindgen(getter)]
    pub fn data(&self) -> Vec<WasmOffering> {
        self.inner
            .data
            .clone()
            .into_iter()
            .map(|o| o.into())
            .collect()
    }

    pub fn from_json_string(json: &str) -> Result<WasmGetOfferingsResponseBody> {
        Ok(Self {
            inner: GetOfferingsResponseBody::from_json_string(json).map_err(map_err)?,
        })
    }

    pub fn to_json_string(&self) -> Result<String> {
        self.inner.to_json_string().map_err(map_err)
    }
}
