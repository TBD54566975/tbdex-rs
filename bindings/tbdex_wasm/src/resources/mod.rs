use crate::errors::{map_err, Result};
use std::str::FromStr;
use tbdex::resources::{ResourceKind, ResourceMetadata};
use wasm_bindgen::prelude::wasm_bindgen;

// pub mod balance;
pub mod offering;

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

    #[wasm_bindgen(getter)]
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

    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> WasmResourceKind {
        self.inner.kind.clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn from(&self) -> String {
        self.inner.from.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.inner.id.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn protocol(&self) -> String {
        self.inner.protocol.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn created_at(&self) -> String {
        self.inner.created_at.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn updated_at(&self) -> Option<String> {
        self.inner.updated_at.clone()
    }
}
