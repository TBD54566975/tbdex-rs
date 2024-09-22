use super::WasmResourceMetadata;
use crate::{
    errors::{map_err, Result},
    web5::bearer_did::WasmBearerDid,
};
use tbdex::resources::balance::{Balance, BalanceData};
use wasm_bindgen::prelude::wasm_bindgen;

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
}

#[wasm_bindgen]
pub struct WasmBalance {
    inner: Balance,
}

impl From<Balance> for WasmBalance {
    fn from(value: Balance) -> Self {
        Self { inner: value }
    }
}

impl From<WasmBalance> for Balance {
    fn from(value: WasmBalance) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmBalance {
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
        Ok(self.inner.sign(&bearer_did.into()).map_err(map_err)?)
    }

    #[wasm_bindgen]
    pub fn verify(&self) -> Result<()> {
        Ok(self.inner.verify().map_err(map_err)?)
    }
}
