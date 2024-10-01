use std::sync::Arc;

use tbdex::http_client::exchanges::Exchange;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::messages::{
    cancel::WasmCancel, close::WasmClose, order::WasmOrder,
    order_instructions::WasmOrderInstructions, order_status::WasmOrderStatus, quote::WasmQuote,
    rfq::WasmRfq,
};

#[wasm_bindgen]
pub struct WasmExchange {
    inner: Exchange,
}

#[wasm_bindgen]
impl WasmExchange {
    #[wasm_bindgen(constructor)]
    pub fn new(
        rfq: WasmRfq,
        quote: Option<WasmQuote>,
        order: Option<WasmOrder>,
        order_instructions: Option<WasmOrderInstructions>,
        cancel: Option<WasmCancel>,
        order_statuses: Option<Vec<WasmOrderStatus>>,
        close: Option<WasmClose>,
    ) -> Self {
        Self {
            inner: Exchange {
                rfq: Arc::new(rfq.into()),
                quote: quote.map(|q| Arc::new(q.into())),
                order: order.map(|o| Arc::new(o.into())),
                order_instructions: order_instructions.map(|oi| Arc::new(oi.into())),
                cancel: cancel.map(|c| Arc::new(c.into())),
                order_statuses: order_statuses
                    .map(|oss| oss.into_iter().map(|os| Arc::new(os.into())).collect()),
                close: close.map(|c| Arc::new(c.into())),
            },
        }
    }

    #[wasm_bindgen(getter)]
    pub fn rfq(&self) -> WasmRfq {
        (*self.inner.rfq).clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn quote(&self) -> Option<WasmQuote> {
        self.inner.quote.as_deref().map(|q| q.clone().into())
    }

    #[wasm_bindgen(getter)]
    pub fn order(&self) -> Option<WasmOrder> {
        self.inner.order.as_deref().map(|o| o.clone().into())
    }

    #[wasm_bindgen(getter)]
    pub fn order_instructions(&self) -> Option<WasmOrderInstructions> {
        self.inner
            .order_instructions
            .as_deref()
            .map(|oi| oi.clone().into())
    }

    #[wasm_bindgen(getter)]
    pub fn cancel(&self) -> Option<WasmCancel> {
        self.inner.cancel.as_deref().map(|c| c.clone().into())
    }

    #[wasm_bindgen(getter)]
    pub fn order_statuses(&self) -> Option<Vec<WasmOrderStatus>> {
        self.inner
            .order_statuses
            .clone()
            .map(|oss| oss.into_iter().map(|os| (*os).clone().into()).collect())
    }

    #[wasm_bindgen(getter)]
    pub fn close(&self) -> Option<WasmClose> {
        self.inner.close.as_deref().map(|c| c.clone().into())
    }
}
