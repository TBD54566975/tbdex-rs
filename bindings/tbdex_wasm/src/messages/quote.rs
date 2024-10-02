use super::WasmMessageMetadata;
use crate::{
    errors::{map_err, Result},
    web5::bearer_did::WasmBearerDid,
};
use tbdex::{
    json::{FromJson, ToJson},
    messages::quote::{Quote, QuoteData, QuoteDetails},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmQuote {
    inner: Quote,
}

impl From<WasmQuote> for Quote {
    fn from(value: WasmQuote) -> Self {
        value.inner
    }
}

impl From<Quote> for WasmQuote {
    fn from(value: Quote) -> Self {
        Self { inner: value }
    }
}

#[wasm_bindgen]
impl WasmQuote {
    #[wasm_bindgen(constructor)]
    pub fn new(metadata: WasmMessageMetadata, data: WasmQuoteData, signature: String) -> Self {
        Self {
            inner: Quote {
                metadata: metadata.into(),
                data: data.into(),
                signature,
            },
        }
    }

    pub fn from_json_string(json: &str) -> Result<WasmQuote> {
        Ok(Self {
            inner: Quote::from_json_string(json).map_err(map_err)?,
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
        data: WasmQuoteData,
        protocol: Option<String>,
        external_id: Option<String>,
    ) -> Result<WasmQuote> {
        Ok(WasmQuote {
            inner: Quote::create(to, from, exchange_id, &data.into(), protocol, external_id)
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
    pub fn data(&self) -> WasmQuoteData {
        self.inner.data.clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn signature(&self) -> String {
        self.inner.signature.clone()
    }
}

#[wasm_bindgen]
pub struct WasmQuoteData {
    inner: QuoteData,
}

impl From<QuoteData> for WasmQuoteData {
    fn from(value: QuoteData) -> Self {
        Self { inner: value }
    }
}

impl From<WasmQuoteData> for QuoteData {
    fn from(value: WasmQuoteData) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmQuoteData {
    #[wasm_bindgen(constructor)]
    pub fn new(
        expires_at: String,
        payout_units_per_payin_unit: String,
        payin: WasmQuoteDetails,
        payout: WasmQuoteDetails,
    ) -> Self {
        Self {
            inner: QuoteData {
                expires_at,
                payout_units_per_payin_unit,
                payin: payin.into(),
                payout: payout.into(),
            },
        }
    }

    #[wasm_bindgen(getter)]
    pub fn expires_at(&self) -> String {
        self.inner.expires_at.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn payout_units_per_payin_unit(&self) -> String {
        self.inner.payout_units_per_payin_unit.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn payin(&self) -> WasmQuoteDetails {
        self.inner.payin.clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn payout(&self) -> WasmQuoteDetails {
        self.inner.payout.clone().into()
    }
}

#[wasm_bindgen]
pub struct WasmQuoteDetails {
    inner: QuoteDetails,
}

impl From<QuoteDetails> for WasmQuoteDetails {
    fn from(value: QuoteDetails) -> Self {
        Self { inner: value }
    }
}

impl From<WasmQuoteDetails> for QuoteDetails {
    fn from(value: WasmQuoteDetails) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmQuoteDetails {
    #[wasm_bindgen(constructor)]
    pub fn new(
        currency_code: String,
        subtotal: String,
        total: String,
        fee: Option<String>,
    ) -> Self {
        Self {
            inner: QuoteDetails {
                currency_code,
                subtotal,
                total,
                fee,
            },
        }
    }

    #[wasm_bindgen(getter)]
    pub fn currency_code(&self) -> String {
        self.inner.currency_code.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn subtotal(&self) -> String {
        self.inner.subtotal.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn total(&self) -> String {
        self.inner.total.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn fee(&self) -> Option<String> {
        self.inner.fee.clone()
    }
}
