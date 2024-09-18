use tbdex::messages::rfq::CreateSelectedPayinMethod;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmCreateSelectedPayinMethod {
    inner: CreateSelectedPayinMethod,
}

#[wasm_bindgen]
impl WasmCreateSelectedPayinMethod {
    #[wasm_bindgen(constructor)]
    pub fn new(
        kind: String,
        // _payment_details: Option<JsValue>,
        amount: String,
    ) -> WasmCreateSelectedPayinMethod {
        WasmCreateSelectedPayinMethod {
            inner: CreateSelectedPayinMethod {
                kind,
                payment_details: None, // TODO
                amount,
            },
        }
    }
}
