use super::{key_managers::WasmKeyManager, signers::WasmSigner};
use crate::errors::{map_web5_err, Result};
use wasm_bindgen::prelude::wasm_bindgen;
use web5::{
    dids::{bearer_did::BearerDid, data_model::document::Document, did::Did},
    json::FromJson,
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
    pub fn new(
        did_uri: &str,
        document_json: &str,
        key_manager: WasmKeyManager,
    ) -> Result<WasmBearerDid> {
        Ok(Self {
            inner: BearerDid {
                did: Did::parse(did_uri).map_err(map_web5_err)?,
                document: Document::from_json_string(document_json).map_err(map_web5_err)?,
                key_manager: key_manager.into(),
            },
        })
    }

    #[wasm_bindgen]
    pub fn get_signer(&self, verification_method_id: &str) -> Result<WasmSigner> {
        Ok(self
            .inner
            .get_signer(verification_method_id)
            .map_err(map_web5_err)?
            .into())
    }
}
