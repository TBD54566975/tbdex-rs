use crate::{
    errors::{map_err, Result},
    messages::{
        cancel::WasmCancel, close::WasmClose, order::WasmOrder,
        order_instructions::WasmOrderInstructions, order_status::WasmOrderStatus, quote::WasmQuote,
        rfq::WasmRfq,
    },
    web5::bearer_did::WasmBearerDid,
};
use std::sync::Arc;
use tbdex::http_client::exchanges::{Exchange, GetExchangeIdsQueryParams};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub async fn create_exchange(rfq: WasmRfq, reply_to: Option<String>) -> Result<()> {
    Ok(
        tbdex::http_client::exchanges::create_exchange(&rfq.into(), reply_to)
            .await
            .map_err(map_err)?,
    )
}

#[wasm_bindgen]
pub async fn submit_order(order: WasmOrder) -> Result<()> {
    Ok(tbdex::http_client::exchanges::submit_order(&order.into())
        .await
        .map_err(map_err)?)
}

#[wasm_bindgen]
pub async fn submit_cancel(cancel: WasmCancel) -> Result<()> {
    Ok(tbdex::http_client::exchanges::submit_cancel(&cancel.into())
        .await
        .map_err(map_err)?)
}

#[wasm_bindgen]
pub async fn get_exchange(
    pfi_did_uri: &str,
    bearer_did: WasmBearerDid,
    exchange_id: &str,
) -> Result<WasmExchange> {
    Ok(
        tbdex::http_client::exchanges::get_exchange(pfi_did_uri, &bearer_did.into(), exchange_id)
            .await
            .map_err(map_err)?
            .into(),
    )
}

#[wasm_bindgen]
pub async fn get_exchange_ids(
    pfi_did_uri: &str,
    requestor_did: WasmBearerDid,
    pagination_offset: Option<i64>,
    pagination_limit: Option<i64>,
) -> Result<Vec<String>> {
    Ok(tbdex::http_client::exchanges::get_exchange_ids(
        pfi_did_uri,
        &requestor_did.into(),
        Some(GetExchangeIdsQueryParams {
            pagination_offset,
            pagination_limit,
        }),
    )
    .await
    .map_err(map_err)?)
}

#[wasm_bindgen]
pub struct WasmExchange {
    inner: Exchange,
}

impl From<WasmExchange> for Exchange {
    fn from(value: WasmExchange) -> Self {
        value.inner
    }
}

impl From<Exchange> for WasmExchange {
    fn from(value: Exchange) -> Self {
        Self { inner: value }
    }
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
