use super::WasmMessageMetadata;
use crate::{
    errors::{map_err, Result},
    web5::bearer_did::WasmBearerDid,
};
use std::str::FromStr;
use tbdex::{
    json::{FromJson, ToJson},
    messages::order_status::{OrderStatus, OrderStatusData, Status},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmOrderStatus {
    inner: OrderStatus,
}

#[wasm_bindgen]
impl WasmOrderStatus {
    #[wasm_bindgen(constructor)]
    pub fn new(
        metadata: WasmMessageMetadata,
        data: WasmOrderStatusData,
        signature: String,
    ) -> Result<WasmOrderStatus> {
        Ok(Self {
            inner: OrderStatus {
                metadata: metadata.into(),
                data: data.into(),
                signature,
            },
        })
    }

    pub fn from_json_string(json: &str) -> Result<WasmOrderStatus> {
        Ok(Self {
            inner: OrderStatus::from_json_string(json).map_err(map_err)?,
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
        data: WasmOrderStatusData,
        protocol: Option<String>,
        external_id: Option<String>,
    ) -> Result<WasmOrderStatus> {
        let data: OrderStatusData = data.into();
        Ok(WasmOrderStatus {
            inner: OrderStatus::create(to, from, exchange_id, &data, protocol, external_id)
                .map_err(map_err)?,
        })
    }

    #[wasm_bindgen]
    pub fn sign(&mut self, bearer_did: WasmBearerDid) -> Result<()> {
        self.inner.sign(&bearer_did.into()).map_err(map_err)
    }

    #[wasm_bindgen]
    pub fn verify(&self) -> Result<()> {
        self.inner.verify().map_err(map_err)
    }

    #[wasm_bindgen(getter)]
    pub fn metadata(&self) -> WasmMessageMetadata {
        self.inner.metadata.clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn data(&self) -> WasmOrderStatusData {
        self.inner.data.clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn signature(&self) -> String {
        self.inner.signature.clone()
    }
}

#[wasm_bindgen]
pub struct WasmOrderStatusData {
    inner: OrderStatusData,
}

impl From<OrderStatusData> for WasmOrderStatusData {
    fn from(value: OrderStatusData) -> Self {
        Self { inner: value }
    }
}

impl From<WasmOrderStatusData> for OrderStatusData {
    fn from(value: WasmOrderStatusData) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmOrderStatusData {
    #[wasm_bindgen(constructor)]
    pub fn new(status: String, details: Option<String>) -> Result<WasmOrderStatusData> {
        let status_enum = Status::from_str(&status).map_err(map_err)?;
        Ok(Self {
            inner: OrderStatusData {
                status: status_enum,
                details,
            },
        })
    }

    #[wasm_bindgen(getter)]
    pub fn status(&self) -> String {
        self.inner.status.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn details(&self) -> Option<String> {
        self.inner.details.clone()
    }
}
