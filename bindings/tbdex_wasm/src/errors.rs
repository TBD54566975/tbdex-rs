use serde::Serialize;
use serde_wasm_bindgen::to_value;
use tbdex::errors::TbdexError;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web5::errors::Web5Error;

pub type Result<T> = std::result::Result<T, JsValue>;

#[wasm_bindgen]
#[derive(Serialize)]
pub struct WasmTbdexError {
    variant: String,
    message: String,
    is_tbdex_error: bool,
}

#[wasm_bindgen]
impl WasmTbdexError {
    #[wasm_bindgen(getter)]
    pub fn variant(&self) -> String {
        self.variant.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn message(&self) -> String {
        self.message.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn is_web5_error(&self) -> bool {
        self.is_tbdex_error
    }
}

pub fn map_err(err: TbdexError) -> JsValue {
    let msg = format!("{:?}", err);
    let variant = msg.split('(').next().unwrap_or("Unknown").to_string();

    let js_error = WasmTbdexError {
        variant,
        message: err.to_string(),
        is_tbdex_error: true,
    };

    to_value(&js_error).unwrap_or_else(|_| JsValue::from_str("failed to serialize error"))
}

pub fn map_web5_err(err: Web5Error) -> JsValue {
    map_err(TbdexError::Web5Error(err))
}
