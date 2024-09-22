use crate::errors::{map_web5_err, Result};

use super::{document::WasmDocument, jwk::WasmJwk};
use wasm_bindgen::prelude::wasm_bindgen;
use web5::{
    dids::portable_did::PortableDid,
    json::{FromJson, ToJson},
};

#[wasm_bindgen]
pub struct WasmPortableDid {
    inner: PortableDid,
}

impl From<WasmPortableDid> for PortableDid {
    fn from(value: WasmPortableDid) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmPortableDid {
    #[wasm_bindgen(constructor)]
    pub fn new(did_uri: String, document: WasmDocument, private_jwks: Vec<WasmJwk>) -> Self {
        Self {
            inner: PortableDid {
                did_uri,
                document: document.into(),
                private_jwks: private_jwks.into_iter().map(|pj| pj.into()).collect(),
            },
        }
    }

    #[wasm_bindgen]
    pub fn from_json_string(json: &str) -> Result<WasmPortableDid> {
        Ok(Self {
            inner: PortableDid::from_json_string(json).map_err(map_web5_err)?,
        })
    }

    #[wasm_bindgen]
    pub fn to_json_string(&self) -> Result<String> {
        Ok(self.inner.to_json_string().map_err(map_web5_err)?)
    }
}
