use crate::resources::balance::WasmBalance;
use tbdex::http::balances::GetBalancesResponseBody;
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
                data: data.into_iter().map(|b| b.into()).collect(),
            },
        }
    }
}
