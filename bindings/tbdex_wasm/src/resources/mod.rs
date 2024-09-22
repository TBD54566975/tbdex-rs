use crate::errors::{map_err, Result};
use std::str::FromStr;
use tbdex::resources::{ResourceKind, ResourceMetadata};
use wasm_bindgen::prelude::wasm_bindgen;

pub mod balance;

#[wasm_bindgen]
pub struct WasmResourceKind {
    inner: ResourceKind,
}

impl From<ResourceKind> for WasmResourceKind {
    fn from(value: ResourceKind) -> Self {
        Self { inner: value }
    }
}

impl From<WasmResourceKind> for ResourceKind {
    fn from(value: WasmResourceKind) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmResourceKind {
    #[wasm_bindgen(constructor)]
    pub fn new(kind: &str) -> Result<WasmResourceKind> {
        let inner = ResourceKind::from_str(kind).map_err(map_err)?;
        Ok(WasmResourceKind { inner })
    }

    #[wasm_bindgen]
    pub fn kind(&self) -> String {
        self.inner.to_string()
    }
}

#[wasm_bindgen]
pub struct WasmResourceMetadata {
    inner: ResourceMetadata,
}

impl From<ResourceMetadata> for WasmResourceMetadata {
    fn from(value: ResourceMetadata) -> Self {
        Self { inner: value }
    }
}

impl From<WasmResourceMetadata> for ResourceMetadata {
    fn from(value: WasmResourceMetadata) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmResourceMetadata {
    #[wasm_bindgen(constructor)]
    pub fn new(
        kind: WasmResourceKind,
        from: String,
        id: String,
        protocol: String,
        created_at: String,
        updated_at: Option<String>,
    ) -> Self {
        Self {
            inner: ResourceMetadata {
                kind: kind.into(),
                from,
                id,
                protocol,
                created_at,
                updated_at,
            },
        }
    }
}
