use super::WasmMessageMetadata;
use crate::{
    errors::{map_err, Result},
    js::convert_to_object,
    web5::bearer_did::WasmBearerDid,
};
use tbdex::{
    json::{FromJson, ToJson},
    messages::rfq::{
        CreateRfqData, CreateSelectedPayinMethod, CreateSelectedPayoutMethod,
        PrivatePaymentDetails, Rfq, RfqData, RfqPrivateData, SelectedPayinMethod,
        SelectedPayoutMethod,
    },
    resources::offering::Offering,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmRfq {
    inner: Rfq,
}

impl From<WasmRfq> for Rfq {
    fn from(value: WasmRfq) -> Self {
        value.inner
    }
}

impl From<Rfq> for WasmRfq {
    fn from(value: Rfq) -> Self {
        Self { inner: value }
    }
}

#[wasm_bindgen]
impl WasmRfq {
    #[wasm_bindgen(constructor)]
    pub fn new(
        metadata: WasmMessageMetadata,
        data: WasmRfqData,
        private_data: Option<WasmRfqPrivateData>,
        signature: String,
    ) -> Self {
        Self {
            inner: Rfq {
                metadata: metadata.into(),
                data: data.into(),
                private_data: private_data.map(|pd| pd.into()),
                signature,
            },
        }
    }

    pub fn from_json_string(json: &str) -> Result<WasmRfq> {
        Ok(Self {
            inner: Rfq::from_json_string(json).map_err(map_err)?,
        })
    }

    pub fn to_json_string(&self) -> Result<String> {
        self.inner.to_json_string().map_err(map_err)
    }

    #[wasm_bindgen]
    pub fn create(
        to: &str,
        from: &str,
        create_rfq_data: WasmCreateRfqData,
        protocol: Option<String>,
        external_id: Option<String>,
    ) -> Result<WasmRfq> {
        Ok(WasmRfq {
            inner: Rfq::create(
                to,
                from,
                &CreateRfqData::from(create_rfq_data),
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

    #[wasm_bindgen]
    pub async fn verify_offering_requirements(&self, offering_json: &str) -> Result<()> {
        let offering = Offering::from_json_string(offering_json).map_err(map_err)?;
        self.inner
            .verify_offering_requirements(&offering)
            .await
            .map_err(map_err)
    }

    #[wasm_bindgen]
    pub fn verify_all_private_data(&self) -> Result<()> {
        self.inner.verify_all_private_data().map_err(map_err)
    }

    #[wasm_bindgen]
    pub fn verify_present_private_data(&self) -> Result<()> {
        self.inner.verify_present_private_data().map_err(map_err)
    }

    #[wasm_bindgen(getter)]
    pub fn metadata(&self) -> WasmMessageMetadata {
        self.inner.metadata.clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn data(&self) -> WasmRfqData {
        self.inner.data.clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn private_data(&self) -> Option<WasmRfqPrivateData> {
        self.inner.private_data.clone().map(|pd| pd.into())
    }

    #[wasm_bindgen(getter)]
    pub fn signature(&self) -> String {
        self.inner.signature.clone()
    }
}

#[wasm_bindgen]
pub struct WasmRfqData {
    inner: RfqData,
}

impl From<RfqData> for WasmRfqData {
    fn from(value: RfqData) -> Self {
        Self { inner: value }
    }
}

impl From<WasmRfqData> for RfqData {
    fn from(value: WasmRfqData) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmRfqData {
    #[wasm_bindgen(constructor)]
    pub fn new(
        offering_id: String,
        payin: WasmSelectedPayinMethod,
        payout: WasmSelectedPayoutMethod,
        claims_hash: Option<String>,
    ) -> Self {
        Self {
            inner: RfqData {
                offering_id,
                payin: payin.into(),
                payout: payout.into(),
                claims_hash,
            },
        }
    }

    #[wasm_bindgen(getter)]
    pub fn offering_id(&self) -> String {
        self.inner.offering_id.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn payin(&self) -> WasmSelectedPayinMethod {
        self.inner.payin.clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn payout(&self) -> WasmSelectedPayoutMethod {
        self.inner.payout.clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn claims_hash(&self) -> Option<String> {
        self.inner.claims_hash.clone()
    }
}

#[wasm_bindgen]
pub struct WasmSelectedPayinMethod {
    inner: SelectedPayinMethod,
}

impl From<SelectedPayinMethod> for WasmSelectedPayinMethod {
    fn from(value: SelectedPayinMethod) -> Self {
        Self { inner: value }
    }
}

impl From<WasmSelectedPayinMethod> for SelectedPayinMethod {
    fn from(value: WasmSelectedPayinMethod) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmSelectedPayinMethod {
    #[wasm_bindgen(constructor)]
    pub fn new(kind: String, payment_details_hash: Option<String>, amount: String) -> Self {
        Self {
            inner: SelectedPayinMethod {
                kind,
                payment_details_hash,
                amount,
            },
        }
    }

    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> String {
        self.inner.kind.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn payment_details_hash(&self) -> Option<String> {
        self.inner.payment_details_hash.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn amount(&self) -> String {
        self.inner.amount.clone()
    }
}

#[wasm_bindgen]
pub struct WasmSelectedPayoutMethod {
    inner: SelectedPayoutMethod,
}

impl From<SelectedPayoutMethod> for WasmSelectedPayoutMethod {
    fn from(value: SelectedPayoutMethod) -> Self {
        Self { inner: value }
    }
}

impl From<WasmSelectedPayoutMethod> for SelectedPayoutMethod {
    fn from(value: WasmSelectedPayoutMethod) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmSelectedPayoutMethod {
    #[wasm_bindgen(constructor)]
    pub fn new(kind: String, payment_details_hash: Option<String>) -> Self {
        Self {
            inner: SelectedPayoutMethod {
                kind,
                payment_details_hash,
            },
        }
    }

    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> String {
        self.inner.kind.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn payment_details_hash(&self) -> Option<String> {
        self.inner.payment_details_hash.clone()
    }
}

#[wasm_bindgen]
pub struct WasmRfqPrivateData {
    inner: RfqPrivateData,
}

impl From<RfqPrivateData> for WasmRfqPrivateData {
    fn from(value: RfqPrivateData) -> Self {
        Self { inner: value }
    }
}

impl From<WasmRfqPrivateData> for RfqPrivateData {
    fn from(value: WasmRfqPrivateData) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmRfqPrivateData {
    #[wasm_bindgen(constructor)]
    pub fn new(
        salt: String,
        payin: Option<WasmPrivatePaymentDetails>,
        payout: Option<WasmPrivatePaymentDetails>,
        claims: Option<js_sys::Array>,
    ) -> Self {
        let claims_option = claims.map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_string())
                .collect::<Vec<String>>()
        });

        Self {
            inner: RfqPrivateData {
                salt,
                payin: payin.map(|p| p.into()),
                payout: payout.map(|p| p.into()),
                claims: claims_option,
            },
        }
    }

    #[wasm_bindgen(getter)]
    pub fn salt(&self) -> String {
        self.inner.salt.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn payin(&self) -> Option<WasmPrivatePaymentDetails> {
        self.inner.payin.clone().map(|p| p.into())
    }

    #[wasm_bindgen(getter)]
    pub fn payout(&self) -> Option<WasmPrivatePaymentDetails> {
        self.inner.payout.clone().map(|p| p.into())
    }

    #[wasm_bindgen(getter)]
    pub fn claims(&self) -> Option<js_sys::Array> {
        self.inner.claims.as_ref().map(|claims| {
            claims
                .iter()
                .map(|s| JsValue::from_str(s))
                .collect::<js_sys::Array>()
        })
    }
}

#[wasm_bindgen]
pub struct WasmPrivatePaymentDetails {
    inner: PrivatePaymentDetails,
}

impl From<PrivatePaymentDetails> for WasmPrivatePaymentDetails {
    fn from(value: PrivatePaymentDetails) -> Self {
        Self { inner: value }
    }
}

impl From<WasmPrivatePaymentDetails> for PrivatePaymentDetails {
    fn from(value: WasmPrivatePaymentDetails) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmPrivatePaymentDetails {
    #[wasm_bindgen(constructor)]
    pub fn new(payment_details: JsValue) -> Result<WasmPrivatePaymentDetails> {
        let payment_details = if payment_details.is_null() || payment_details.is_undefined() {
            None
        } else {
            Some(serde_wasm_bindgen::from_value(payment_details)?)
        };

        Ok(Self {
            inner: PrivatePaymentDetails { payment_details },
        })
    }

    #[wasm_bindgen(getter)]
    pub fn payment_details(&self) -> Result<JsValue> {
        match &self.inner.payment_details {
            Some(pd) => {
                let value = serde_wasm_bindgen::to_value(pd)?;
                Ok(convert_to_object(value)?)
            }
            None => Ok(JsValue::UNDEFINED),
        }
    }
}

#[wasm_bindgen]
pub struct WasmCreateRfqData {
    inner: CreateRfqData,
}

impl From<CreateRfqData> for WasmCreateRfqData {
    fn from(value: CreateRfqData) -> Self {
        Self { inner: value }
    }
}

impl From<WasmCreateRfqData> for CreateRfqData {
    fn from(value: WasmCreateRfqData) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmCreateRfqData {
    #[wasm_bindgen(constructor)]
    pub fn new(
        offering_id: String,
        payin: WasmCreateSelectedPayinMethod,
        payout: WasmCreateSelectedPayoutMethod,
        claims: js_sys::Array,
    ) -> Self {
        let claims_vec = claims.iter().filter_map(|v| v.as_string()).collect();

        Self {
            inner: CreateRfqData {
                offering_id,
                payin: payin.into(),
                payout: payout.into(),
                claims: claims_vec,
            },
        }
    }

    #[wasm_bindgen(getter)]
    pub fn offering_id(&self) -> String {
        self.inner.offering_id.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn payin(&self) -> WasmCreateSelectedPayinMethod {
        self.inner.payin.clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn payout(&self) -> WasmCreateSelectedPayoutMethod {
        self.inner.payout.clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn claims(&self) -> js_sys::Array {
        self.inner
            .claims
            .iter()
            .map(|s| JsValue::from_str(s))
            .collect::<js_sys::Array>()
    }
}

#[wasm_bindgen]
pub struct WasmCreateSelectedPayinMethod {
    inner: CreateSelectedPayinMethod,
}

impl From<CreateSelectedPayinMethod> for WasmCreateSelectedPayinMethod {
    fn from(value: CreateSelectedPayinMethod) -> Self {
        Self { inner: value }
    }
}

impl From<WasmCreateSelectedPayinMethod> for CreateSelectedPayinMethod {
    fn from(value: WasmCreateSelectedPayinMethod) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmCreateSelectedPayinMethod {
    #[wasm_bindgen(constructor)]
    pub fn new(
        kind: String,
        payment_details: JsValue,
        amount: String,
    ) -> Result<WasmCreateSelectedPayinMethod> {
        let payment_details = if payment_details.is_null() || payment_details.is_undefined() {
            None
        } else {
            Some(serde_wasm_bindgen::from_value(payment_details)?)
        };

        Ok(Self {
            inner: CreateSelectedPayinMethod {
                kind,
                payment_details,
                amount,
            },
        })
    }

    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> String {
        self.inner.kind.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn payment_details(&self) -> Result<JsValue> {
        match &self.inner.payment_details {
            Some(pd) => {
                let value = serde_wasm_bindgen::to_value(pd)?;
                Ok(convert_to_object(value)?)
            }
            None => Ok(JsValue::UNDEFINED),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn amount(&self) -> String {
        self.inner.amount.clone()
    }
}

#[wasm_bindgen]
pub struct WasmCreateSelectedPayoutMethod {
    inner: CreateSelectedPayoutMethod,
}

impl From<CreateSelectedPayoutMethod> for WasmCreateSelectedPayoutMethod {
    fn from(value: CreateSelectedPayoutMethod) -> Self {
        Self { inner: value }
    }
}

impl From<WasmCreateSelectedPayoutMethod> for CreateSelectedPayoutMethod {
    fn from(value: WasmCreateSelectedPayoutMethod) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmCreateSelectedPayoutMethod {
    #[wasm_bindgen(constructor)]
    pub fn new(kind: String, payment_details: JsValue) -> Result<WasmCreateSelectedPayoutMethod> {
        let payment_details = if payment_details.is_null() || payment_details.is_undefined() {
            None
        } else {
            Some(serde_wasm_bindgen::from_value(payment_details)?)
        };

        Ok(Self {
            inner: CreateSelectedPayoutMethod {
                kind,
                payment_details,
            },
        })
    }

    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> String {
        self.inner.kind.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn payment_details(&self) -> Result<JsValue> {
        match &self.inner.payment_details {
            Some(pd) => {
                let value = serde_wasm_bindgen::to_value(pd)?;
                Ok(convert_to_object(value)?)
            }
            None => Ok(JsValue::UNDEFINED),
        }
    }
}
