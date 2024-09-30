use std::sync::Arc;
use wasm_bindgen::prelude::wasm_bindgen;
use web5::crypto::dsa::Signer;

use crate::errors::{map_web5_err, Result};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "{ sign: (payload: Uint8Array) => Uint8Array }")]
    pub type ForeignSigner;

    #[wasm_bindgen(method)]
    fn sign(this: &ForeignSigner, payload: &[u8]) -> Vec<u8>;
}

pub struct ConcreteForeignSigner(ForeignSigner);

/**
 * TODO
 * [KW]:
 *    this is not thread safe and could cause issues
 *    the solution is to implement message passing across threads
 */
unsafe impl Send for ConcreteForeignSigner {}
unsafe impl Sync for ConcreteForeignSigner {}

impl Signer for ConcreteForeignSigner {
    fn sign(&self, payload: &[u8]) -> web5::errors::Result<Vec<u8>> {
        Ok(self.0.sign(payload))
    }
}

#[wasm_bindgen]
pub struct WasmSigner {
    inner: Arc<dyn Signer>,
}

impl From<WasmSigner> for Arc<dyn Signer> {
    fn from(value: WasmSigner) -> Self {
        value.inner
    }
}

impl From<Arc<dyn Signer>> for WasmSigner {
    fn from(value: Arc<dyn Signer>) -> Self {
        Self { inner: value }
    }
}

#[wasm_bindgen]
impl WasmSigner {
    #[wasm_bindgen(constructor)]
    pub fn new(foreign_signer: ForeignSigner) -> Self {
        Self {
            inner: Arc::new(ConcreteForeignSigner(foreign_signer)),
        }
    }

    #[wasm_bindgen]
    pub fn sign(&self, payload: &[u8]) -> Result<Vec<u8>> {
        self.inner.sign(payload).map_err(map_web5_err)
    }
}
