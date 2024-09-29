use super::WasmMessageMetadata;
use crate::{
    errors::{map_err, Result},
    web5::bearer_did::WasmBearerDid,
};
use tbdex::{
    json::{FromJson, ToJson},
    messages::order::{Order, OrderData},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmOrder {
    inner: Order,
}

#[wasm_bindgen]
impl WasmOrder {
    #[wasm_bindgen(constructor)]
    pub fn new(metadata: WasmMessageMetadata, data: WasmOrderData, signature: String) -> Self {
        Self {
            inner: Order {
                metadata: metadata.into(),
                data: data.into(),
                signature,
            },
        }
    }

    pub fn from_json_string(json: &str) -> Result<WasmOrder> {
        Ok(Self {
            inner: Order::from_json_string(json).map_err(map_err)?,
        })
    }

    pub fn to_json_string(&self) -> Result<String> {
        Ok(self.inner.to_json_string().map_err(map_err)?)
    }

    #[wasm_bindgen]
    pub fn create(
        to: &str,
        from: &str,
        exchange_id: &str,
        protocol: Option<String>,
        external_id: Option<String>,
    ) -> Result<WasmOrder> {
        Ok(WasmOrder {
            inner: Order::create(to, from, exchange_id, protocol, external_id).map_err(map_err)?,
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

    #[wasm_bindgen(getter)]
    pub fn metadata(&self) -> WasmMessageMetadata {
        self.inner.metadata.clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn data(&self) -> WasmOrderData {
        self.inner.data.clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn signature(&self) -> String {
        self.inner.signature.clone()
    }
}

#[wasm_bindgen]
pub struct WasmOrderData {
    inner: OrderData,
}

impl From<OrderData> for WasmOrderData {
    fn from(value: OrderData) -> Self {
        Self { inner: value }
    }
}

impl From<WasmOrderData> for OrderData {
    fn from(value: WasmOrderData) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmOrderData {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: OrderData {},
        }
    }
}
