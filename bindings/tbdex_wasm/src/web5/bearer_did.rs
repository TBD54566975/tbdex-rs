use std::sync::Arc;

use super::{did::WasmDid, document::WasmDocument, portable_did::WasmPortableDid};
use crate::errors::{map_web5_err, Result};
use wasm_bindgen::prelude::wasm_bindgen;
use web5::{
    crypto::key_managers::in_memory_key_manager::InMemoryKeyManager, dids::bearer_did::BearerDid,
};

#[wasm_bindgen]
pub struct WasmBearerDid {
    inner: BearerDid,
}

impl From<WasmBearerDid> for BearerDid {
    fn from(value: WasmBearerDid) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmBearerDid {
    #[wasm_bindgen(constructor)]
    pub fn new(did: WasmDid, document: WasmDocument) -> Self {
        Self {
            inner: BearerDid {
                did: did.into(),
                document: document.into(),
                key_manager: Arc::new(InMemoryKeyManager::new()), // todo
            },
        }
    }

    #[wasm_bindgen]
    pub fn from_portable_did(portable_did: WasmPortableDid) -> Result<WasmBearerDid> {
        Ok(Self {
            inner: BearerDid::from_portable_did(portable_did.into()).map_err(map_web5_err)?,
        })
    }

    // todo key exporter for to_portable_did

    // todo signer for get_signer

    #[wasm_bindgen(getter)]
    pub fn did(&self) -> WasmDid {
        self.inner.did.clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn document(&self) -> WasmDocument {
        self.inner.document.clone().into()
    }

    // todo key_manager
}
