use super::WasmMessageMetadata;
use crate::{
    errors::{map_err, Result},
    web5::bearer_did::WasmBearerDid,
};
use tbdex::{
    json::{FromJson, ToJson},
    messages::order_instructions::{OrderInstructions, OrderInstructionsData, PaymentInstruction},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmOrderInstructions {
    inner: OrderInstructions,
}

impl From<WasmOrderInstructions> for OrderInstructions {
    fn from(value: WasmOrderInstructions) -> Self {
        value.inner
    }
}

impl From<OrderInstructions> for WasmOrderInstructions {
    fn from(value: OrderInstructions) -> Self {
        Self { inner: value }
    }
}

#[wasm_bindgen]
impl WasmOrderInstructions {
    #[wasm_bindgen(constructor)]
    pub fn new(
        metadata: WasmMessageMetadata,
        data: WasmOrderInstructionsData,
        signature: String,
    ) -> Self {
        Self {
            inner: OrderInstructions {
                metadata: metadata.into(),
                data: data.into(),
                signature,
            },
        }
    }

    pub fn from_json_string(json: &str) -> Result<WasmOrderInstructions> {
        Ok(Self {
            inner: OrderInstructions::from_json_string(json).map_err(map_err)?,
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
        data: WasmOrderInstructionsData,
        protocol: Option<String>,
        external_id: Option<String>,
    ) -> Result<WasmOrderInstructions> {
        Ok(WasmOrderInstructions {
            inner: OrderInstructions::create(
                to,
                from,
                exchange_id,
                &data.into(),
                protocol,
                external_id,
            )
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
    pub fn data(&self) -> WasmOrderInstructionsData {
        self.inner.data.clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn signature(&self) -> String {
        self.inner.signature.clone()
    }
}

#[wasm_bindgen]
pub struct WasmOrderInstructionsData {
    inner: OrderInstructionsData,
}

impl From<OrderInstructionsData> for WasmOrderInstructionsData {
    fn from(value: OrderInstructionsData) -> Self {
        Self { inner: value }
    }
}

impl From<WasmOrderInstructionsData> for OrderInstructionsData {
    fn from(value: WasmOrderInstructionsData) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmOrderInstructionsData {
    #[wasm_bindgen(constructor)]
    pub fn new(payin: WasmPaymentInstruction, payout: WasmPaymentInstruction) -> Self {
        Self {
            inner: OrderInstructionsData {
                payin: payin.into(),
                payout: payout.into(),
            },
        }
    }

    #[wasm_bindgen(getter)]
    pub fn payin(&self) -> WasmPaymentInstruction {
        self.inner.payin.clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn payout(&self) -> WasmPaymentInstruction {
        self.inner.payout.clone().into()
    }
}

#[wasm_bindgen]
pub struct WasmPaymentInstruction {
    inner: PaymentInstruction,
}

impl From<PaymentInstruction> for WasmPaymentInstruction {
    fn from(value: PaymentInstruction) -> Self {
        Self { inner: value }
    }
}

impl From<WasmPaymentInstruction> for PaymentInstruction {
    fn from(value: WasmPaymentInstruction) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmPaymentInstruction {
    #[wasm_bindgen(constructor)]
    pub fn new(link: Option<String>, instruction: Option<String>) -> Self {
        Self {
            inner: PaymentInstruction { link, instruction },
        }
    }

    #[wasm_bindgen(getter)]
    pub fn link(&self) -> Option<String> {
        self.inner.link.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn instruction(&self) -> Option<String> {
        self.inner.instruction.clone()
    }
}
