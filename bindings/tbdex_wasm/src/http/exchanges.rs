use tbdex::{
    http::exchanges::GetExchangeResponseBody,
    json::{FromJson, ToJson},
    messages::{Message, MessageKind},
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
    pub fn data(&self) -> Result<Vec<WasmGetExchangeResponseBodyDataItem>> {
        let mut data: Vec<WasmGetExchangeResponseBodyDataItem> = Vec::new();

        for message in &self.inner.data {
            let kind = match message {
                Message::Rfq(_) => MessageKind::Rfq.to_string(),
                Message::Quote(_) => MessageKind::Quote.to_string(),
                Message::Order(_) => MessageKind::Order.to_string(),
                Message::OrderInstructions(_) => MessageKind::OrderInstructions.to_string(),
                Message::Cancel(_) => MessageKind::Cancel.to_string(),
                Message::OrderStatus(_) => MessageKind::OrderStatus.to_string(),
                Message::Close(_) => MessageKind::Close.to_string(),
            };

            data.push(WasmGetExchangeResponseBodyDataItem {
                kind,
                json: message.to_json_string().map_err(map_err)?,
            });
        }

        Ok(data)
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

#[wasm_bindgen]
pub struct WasmGetExchangeResponseBodyDataItem {
    kind: String,
    json: String,
}

#[wasm_bindgen]
impl WasmGetExchangeResponseBodyDataItem {
    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> String {
        self.kind.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn json(&self) -> String {
        self.json.clone()
    }
}
