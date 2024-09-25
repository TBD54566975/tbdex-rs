use super::jwk::WasmJwk;
use crate::{
    errors::{map_web5_err, Result},
    web5::signers::WasmSigner,
};
use std::sync::Arc;
use wasm_bindgen::prelude::wasm_bindgen;
use web5::crypto::key_managers::KeyManager;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        typescript_type = "{ importPrivateJwk: (privateJwk: WasmJwk) => WasmJwk, getSigner: (publicJwk: WasmJwk) => WasmSigner }"
    )]
    pub type ForeignKeyManager;

    #[wasm_bindgen(method)]
    fn import_private_jwk(this: &ForeignKeyManager, private_jwk: WasmJwk) -> WasmJwk;

    #[wasm_bindgen(method)]
    fn get_signer(this: &ForeignKeyManager, public_jwk: WasmJwk) -> WasmSigner;
}

pub struct ConcreteForeignKeyManager(ForeignKeyManager);

/**
 * TODO
 * [KW]:
 *    this is not thread safe and could cause issues
 *    the solution is to implement message passing across threads
 */
unsafe impl Send for ConcreteForeignKeyManager {}
unsafe impl Sync for ConcreteForeignKeyManager {}

impl KeyManager for ConcreteForeignKeyManager {
    fn import_private_jwk(
        &self,
        private_jwk: web5::crypto::jwk::Jwk,
    ) -> web5::errors::Result<web5::crypto::jwk::Jwk> {
        Ok(self.0.import_private_jwk(private_jwk.into()).into())
    }

    fn get_signer(
        &self,
        public_jwk: web5::crypto::jwk::Jwk,
    ) -> web5::errors::Result<Arc<dyn web5::crypto::dsa::Signer>> {
        Ok(self.0.get_signer(public_jwk.into()).into())
    }
}

#[wasm_bindgen]
pub struct WasmKeyManager {
    inner: Arc<dyn KeyManager>,
}

impl From<WasmKeyManager> for Arc<dyn KeyManager> {
    fn from(value: WasmKeyManager) -> Self {
        value.inner
    }
}

impl From<Arc<dyn KeyManager>> for WasmKeyManager {
    fn from(value: Arc<dyn KeyManager>) -> Self {
        Self { inner: value }
    }
}

#[wasm_bindgen]
impl WasmKeyManager {
    #[wasm_bindgen(constructor)]
    pub fn new(foreign_key_manager: ForeignKeyManager) -> Self {
        Self {
            inner: Arc::new(ConcreteForeignKeyManager(foreign_key_manager)),
        }
    }

    #[wasm_bindgen]
    pub fn import_private_jwk(&self, private_jwk: WasmJwk) -> Result<WasmJwk> {
        Ok(self
            .inner
            .import_private_jwk(private_jwk.into())
            .map_err(map_web5_err)?
            .into())
    }

    #[wasm_bindgen]
    pub fn get_signer(&self, public_jwk: WasmJwk) -> Result<WasmSigner> {
        Ok(self
            .inner
            .get_signer(public_jwk.into())
            .map_err(map_web5_err)?
            .into())
    }
}
