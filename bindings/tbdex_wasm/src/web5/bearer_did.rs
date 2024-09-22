use super::portable_did::WasmPortableDid;
use crate::errors::{map_web5_err, Result};
use wasm_bindgen::prelude::wasm_bindgen;
use web5::dids::bearer_did::BearerDid;

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
    #[wasm_bindgen]
    pub fn from_portable_did(portable_did: WasmPortableDid) -> Result<WasmBearerDid> {
        Ok(Self {
            inner: BearerDid::from_portable_did(portable_did.into()).map_err(map_web5_err)?,
        })
    }

    // todo key exporter for to_portable_did

    // todo signer for get_signer
}
