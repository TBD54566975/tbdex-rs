use crate::errors::{map_err, Result};
use std::str::FromStr;
use tbdex::messages::{MessageKind, MessageMetadata};
use wasm_bindgen::prelude::wasm_bindgen;

pub mod cancel;
pub mod order_instructions;
pub mod order_status;
pub mod order;
pub mod quote;
pub mod rfq;

#[wasm_bindgen]
pub struct WasmMessageMetadata {
    inner: MessageMetadata,
}

impl From<MessageMetadata> for WasmMessageMetadata {
    fn from(value: MessageMetadata) -> Self {
        Self { inner: value }
    }
}

impl From<WasmMessageMetadata> for MessageMetadata {
    fn from(value: WasmMessageMetadata) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmMessageMetadata {
    #[wasm_bindgen(constructor)]
    pub fn new(
        from: String,
        to: String,
        kind: String,
        id: String,
        exchange_id: String,
        external_id: Option<String>,
        protocol: String,
        created_at: String,
    ) -> Result<WasmMessageMetadata> {
        Ok(Self {
            inner: MessageMetadata {
                from,
                to,
                kind: MessageKind::from_str(&kind).map_err(map_err)?,
                id,
                exchange_id,
                external_id,
                protocol,
                created_at,
            },
        })
    }

    #[wasm_bindgen(getter)]
    pub fn from(&self) -> String {
        self.inner.from.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn to(&self) -> String {
        self.inner.to.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> String {
        self.inner.kind.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.inner.id.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn exchange_id(&self) -> String {
        self.inner.exchange_id.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn external_id(&self) -> Option<String> {
        self.inner.external_id.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn protocol(&self) -> String {
        self.inner.protocol.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn created_at(&self) -> String {
        self.inner.created_at.clone()
    }
}
