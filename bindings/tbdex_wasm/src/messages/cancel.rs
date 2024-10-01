use super::WasmMessageMetadata;
use crate::{
    errors::{map_err, Result},
    web5::bearer_did::WasmBearerDid,
};
use tbdex::{
    json::{FromJson, ToJson},
    messages::cancel::{Cancel, CancelData},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmCancel {
    inner: Cancel,
}

impl From<WasmCancel> for Cancel {
    fn from(value: WasmCancel) -> Self {
        value.inner
    }
}

impl From<Cancel> for WasmCancel {
    fn from(value: Cancel) -> Self {
        Self { inner: value }
    }
}

#[wasm_bindgen]
impl WasmCancel {
    #[wasm_bindgen(constructor)]
    pub fn new(metadata: WasmMessageMetadata, data: WasmCancelData, signature: String) -> Self {
        Self {
            inner: Cancel {
                metadata: metadata.into(),
                data: data.into(),
                signature,
            },
        }
    }

    pub fn from_json_string(json: &str) -> Result<WasmCancel> {
        Ok(Self {
            inner: Cancel::from_json_string(json).map_err(map_err)?,
        })
    }

    pub fn to_json_string(&self) -> Result<String> {
        self.inner.to_json_string().map_err(map_err)
    }

    #[wasm_bindgen]
    pub fn create(
        to: &str,
        from: &str,
        exchange_id: &str,
        data: WasmCancelData,
        protocol: Option<String>,
        external_id: Option<String>,
    ) -> Result<WasmCancel> {
        Ok(WasmCancel {
            inner: Cancel::create(to, from, exchange_id, &data.into(), protocol, external_id)
                .map_err(map_err)?,
        })
    }

    #[wasm_bindgen]
    pub fn sign(&mut self, bearer_did: WasmBearerDid) -> Result<()> {
        self.inner.sign(&bearer_did.into()).map_err(map_err)
    }

    #[wasm_bindgen]
    pub async fn verify(&self) -> Result<()> {
        self.inner.verify().await.map_err(map_err)
    }

    #[wasm_bindgen(getter)]
    pub fn metadata(&self) -> WasmMessageMetadata {
        self.inner.metadata.clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn data(&self) -> WasmCancelData {
        self.inner.data.clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn signature(&self) -> String {
        self.inner.signature.clone()
    }
}

#[wasm_bindgen]
pub struct WasmCancelData {
    inner: CancelData,
}

impl From<CancelData> for WasmCancelData {
    fn from(value: CancelData) -> Self {
        Self { inner: value }
    }
}

impl From<WasmCancelData> for CancelData {
    fn from(value: WasmCancelData) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmCancelData {
    #[wasm_bindgen(constructor)]
    pub fn new(reason: Option<String>) -> Self {
        Self {
            inner: CancelData { reason },
        }
    }

    #[wasm_bindgen(getter)]
    pub fn reason(&self) -> Option<String> {
        self.inner.reason.clone()
    }
}
