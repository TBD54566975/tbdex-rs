use super::WasmResourceMetadata;
use crate::{
    errors::{map_err, Result},
    web5::bearer_did::WasmBearerDid,
};
use tbdex::{
    json::{FromJson, ToJson},
    resources::balance::{Balance, BalanceData},
};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct WasmBalance {
    inner: Balance,
}

#[wasm_bindgen]
impl WasmBalance {
    #[wasm_bindgen(constructor)]
    pub fn new(metadata: WasmResourceMetadata, data: WasmBalanceData, signature: String) -> Self {
        Self {
            inner: Balance {
                metadata: metadata.into(),
                data: data.into(),
                signature,
            },
        }
    }

    pub fn from_json_string(json: &str) -> Result<WasmBalance> {
        Ok(Self {
            inner: Balance::from_json_string(json).map_err(map_err)?,
        })
    }

    pub fn to_json_string(&self) -> Result<String> {
        self.inner.to_json_string().map_err(map_err)
    }

    #[wasm_bindgen]
    pub fn create(
        from: &str,
        data: WasmBalanceData,
        protocol: Option<String>,
    ) -> Result<WasmBalance> {
        Ok(WasmBalance {
            inner: Balance::create(from, &BalanceData::from(data), protocol).map_err(map_err)?,
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
    pub fn metadata(&self) -> WasmResourceMetadata {
        self.inner.metadata.clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn data(&self) -> WasmBalanceData {
        self.inner.data.clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn signature(&self) -> String {
        self.inner.signature.clone()
    }
}

#[wasm_bindgen]
pub struct WasmBalanceData {
    inner: BalanceData,
}

impl From<BalanceData> for WasmBalanceData {
    fn from(value: BalanceData) -> Self {
        Self { inner: value }
    }
}

impl From<WasmBalanceData> for BalanceData {
    fn from(value: WasmBalanceData) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmBalanceData {
    #[wasm_bindgen(constructor)]
    pub fn new(currency_code: String, available: String) -> Self {
        Self {
            inner: BalanceData {
                currency_code,
                available,
            },
        }
    }

    #[wasm_bindgen(getter)]
    pub fn currency_code(&self) -> String {
        self.inner.currency_code.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn available(&self) -> String {
        self.inner.available.clone()
    }
}
