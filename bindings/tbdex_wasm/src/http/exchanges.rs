use tbdex::{
    http::exchanges::GetExchangeResponseBody,
    json::{FromJson, ToJson},
    messages::Message,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::{
    errors::{map_err, Result},
    js::convert_to_object,
};

#[wasm_bindgen]
pub struct WasmGetExchangeResponseBody {
    inner: GetExchangeResponseBody,
}

#[wasm_bindgen]
impl WasmGetExchangeResponseBody {
    #[wasm_bindgen(constructor)]
    pub fn new(data: Vec<JsValue>) -> Result<WasmGetExchangeResponseBody> {
        let messages: Vec<Message> = data
            .into_iter()
            .map(|m| serde_wasm_bindgen::from_value::<Message>(m).map_err(|e| e.into()))
            .collect::<Result<Vec<Message>>>()?;

        Ok(Self {
            inner: GetExchangeResponseBody { data: messages },
        })
    }

    #[wasm_bindgen(getter)]
    pub fn data(&self) -> Result<JsValue> {
        let js_value = serde_wasm_bindgen::to_value(&self.inner.data)?;
        Ok(convert_to_object(js_value)?)
    }

    pub fn from_json_string(json: &str) -> Result<WasmGetExchangeResponseBody> {
        Ok(Self {
            inner: GetExchangeResponseBody::from_json_string(json).map_err(map_err)?,
        })
    }

    pub fn to_json_string(&self) -> Result<String> {
        self.inner.to_json_string().map_err(map_err)
    }
}
