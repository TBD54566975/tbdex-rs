use super::WasmMessageMetadata;
use crate::{
    errors::{map_err, Result},
    web5::bearer_did::WasmBearerDid,
};
use tbdex::{
    json::{FromJson, ToJson},
    messages::close::{Close, CloseData},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmClose {
    inner: Close,
}

impl From<WasmClose> for Close {
    fn from(value: WasmClose) -> Self {
        value.inner
    }
}

impl From<Close> for WasmClose {
    fn from(value: Close) -> Self {
        Self { inner: value }
    }
}

#[wasm_bindgen]
impl WasmClose {
    #[wasm_bindgen(constructor)]
    pub fn new(metadata: WasmMessageMetadata, data: WasmCloseData, signature: String) -> Self {
        Self {
            inner: Close {
                metadata: metadata.into(),
                data: data.into(),
                signature,
            },
        }
    }

    pub fn from_json_string(json: &str) -> Result<WasmClose> {
        Ok(Self {
            inner: Close::from_json_string(json).map_err(map_err)?,
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
        data: WasmCloseData,
        protocol: Option<String>,
        external_id: Option<String>,
    ) -> Result<WasmClose> {
        Ok(WasmClose {
            inner: Close::create(to, from, exchange_id, &data.into(), protocol, external_id)
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
    pub fn data(&self) -> WasmCloseData {
        self.inner.data.clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn signature(&self) -> String {
        self.inner.signature.clone()
    }
}

#[wasm_bindgen]
pub struct WasmCloseData {
    inner: CloseData,
}

impl From<CloseData> for WasmCloseData {
    fn from(value: CloseData) -> Self {
        Self { inner: value }
    }
}

impl From<WasmCloseData> for CloseData {
    fn from(value: WasmCloseData) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmCloseData {
    #[wasm_bindgen(constructor)]
    pub fn new(reason: Option<String>, success: Option<bool>) -> Self {
        Self {
            inner: CloseData { reason, success },
        }
    }

    #[wasm_bindgen(getter)]
    pub fn reason(&self) -> Option<String> {
        self.inner.reason.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn success(&self) -> Option<bool> {
        self.inner.success
    }
}
