use crate::{
    errors::{map_err, Result},
    resources::balance::WasmBalance,
};
use tbdex::{
    http::balances::GetBalancesResponseBody,
    json::{FromJson, ToJson},
};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct WasmGetBalancesResponseBody {
    inner: GetBalancesResponseBody,
}

#[wasm_bindgen]
impl WasmGetBalancesResponseBody {
    #[wasm_bindgen(constructor)]
    pub fn new(data: Vec<WasmBalance>) -> Self {
        Self {
            inner: GetBalancesResponseBody {
                data: data.into_iter().map(|o| o.into()).collect(),
            },
        }
    }

    #[wasm_bindgen(getter)]
    pub fn data(&self) -> Vec<WasmBalance> {
        self.inner
            .data
            .clone()
            .into_iter()
            .map(|o| o.into())
            .collect()
    }

    pub fn from_json_string(json: &str) -> Result<WasmGetBalancesResponseBody> {
        Ok(Self {
            inner: GetBalancesResponseBody::from_json_string(json).map_err(map_err)?,
        })
    }

    pub fn to_json_string(&self) -> Result<String> {
        self.inner.to_json_string().map_err(map_err)
    }
}
