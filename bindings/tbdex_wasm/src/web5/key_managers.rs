use crate::{
    errors::{map_err, map_web5_err, Result},
    web5::signers::WasmSigner,
};
use std::sync::Arc;
use wasm_bindgen::prelude::wasm_bindgen;
use web5::crypto::{
    jwk::Jwk,
    key_managers::{in_memory_key_manager::InMemoryKeyManager, KeyManager},
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        typescript_type = "{ import_private_jwk: (private_jwk_json: string) => string, get_signer: (public_jwk_json: string) => WasmSigner }"
    )]
    pub type ForeignKeyManager;

    #[wasm_bindgen(method)]
    fn import_private_jwk(this: &ForeignKeyManager, private_jwk_json: &str) -> String;

    #[wasm_bindgen(method)]
    fn get_signer(this: &ForeignKeyManager, public_jwk_json: &str) -> WasmSigner;
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
    fn import_private_jwk(&self, private_jwk: Jwk) -> web5::errors::Result<Jwk> {
        let private_jwk_json = serde_json::to_string(&private_jwk)?;
        let public_jwk_json = self.0.import_private_jwk(&private_jwk_json);
        let public_jwk = serde_json::from_str::<Jwk>(&public_jwk_json)?;
        Ok(public_jwk)
    }

    fn get_signer(
        &self,
        public_jwk: Jwk,
    ) -> web5::errors::Result<Arc<dyn web5::crypto::dsa::Signer>> {
        let public_jwk_json = serde_json::to_string(&public_jwk)?;
        Ok(self.0.get_signer(&public_jwk_json).into())
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
    pub fn import_private_jwk(&self, private_jwk_json: &str) -> Result<String> {
        let private_jwk =
            serde_json::from_str::<Jwk>(private_jwk_json).map_err(|e| map_err(e.into()))?;
        let public_jwk = self
            .inner
            .import_private_jwk(private_jwk.into())
            .map_err(map_web5_err)?;
        let public_jwk_json = serde_json::to_string(&public_jwk).map_err(|e| map_err(e.into()))?;

        Ok(public_jwk_json)
    }

    #[wasm_bindgen]
    pub fn get_signer(&self, public_jwk_json: &str) -> Result<WasmSigner> {
        let public_jwk =
            serde_json::from_str::<Jwk>(public_jwk_json).map_err(|e| map_err(e.into()))?;
        Ok(self
            .inner
            .get_signer(public_jwk.into())
            .map_err(map_web5_err)?
            .into())
    }
}

#[wasm_bindgen]
pub fn new_in_memory_key_manager() -> WasmKeyManager {
    let in_memory_key_manager = InMemoryKeyManager::new();
    WasmKeyManager {
        inner: Arc::new(in_memory_key_manager),
    }
}
