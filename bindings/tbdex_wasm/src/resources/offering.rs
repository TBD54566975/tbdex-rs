use super::WasmResourceMetadata;
use crate::{
    errors::{map_err, Result},
    js::convert_to_object,
    web5::{bearer_did::WasmBearerDid, presentation_definition::WasmPresentationDefinition},
};
use tbdex::{
    json::{FromJson, ToJson},
    resources::offering::{
        CancellationDetails, Offering, OfferingData, PayinDetails, PayinMethod, PayoutDetails,
        PayoutMethod,
    },
};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub struct WasmOffering {
    inner: Offering,
}

impl From<WasmOffering> for Offering {
    fn from(value: WasmOffering) -> Self {
        value.inner
    }
}

impl From<Offering> for WasmOffering {
    fn from(value: Offering) -> Self {
        Self { inner: value }
    }
}

#[wasm_bindgen]
impl WasmOffering {
    #[wasm_bindgen(constructor)]
    pub fn new(metadata: WasmResourceMetadata, data: WasmOfferingData, signature: String) -> Self {
        Self {
            inner: Offering {
                metadata: metadata.into(),
                data: data.into(),
                signature,
            },
        }
    }

    pub fn from_json_string(json: &str) -> Result<WasmOffering> {
        Ok(Self {
            inner: Offering::from_json_string(json).map_err(map_err)?,
        })
    }

    pub fn to_json_string(&self) -> Result<String> {
        self.inner.to_json_string().map_err(map_err)
    }

    pub fn create(
        from: &str,
        data: WasmOfferingData,
        protocol: Option<String>,
    ) -> Result<WasmOffering> {
        Ok(Self {
            inner: Offering::create(from, &data.into(), protocol).map_err(map_err)?,
        })
    }

    pub fn sign(&mut self, bearer_did: WasmBearerDid) -> Result<()> {
        self.inner.sign(&bearer_did.into()).map_err(map_err)
    }

    pub async fn verify(&self) -> Result<()> {
        self.inner.verify().await.map_err(map_err)
    }

    #[wasm_bindgen(getter)]
    pub fn metadata(&self) -> WasmResourceMetadata {
        self.inner.metadata.clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn data(&self) -> WasmOfferingData {
        self.inner.data.clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn signature(&self) -> String {
        self.inner.signature.clone()
    }
}

#[wasm_bindgen]
pub struct WasmOfferingData {
    inner: OfferingData,
}

impl From<WasmOfferingData> for OfferingData {
    fn from(value: WasmOfferingData) -> Self {
        value.inner
    }
}

impl From<OfferingData> for WasmOfferingData {
    fn from(value: OfferingData) -> Self {
        Self { inner: value }
    }
}

#[wasm_bindgen]
impl WasmOfferingData {
    #[wasm_bindgen(constructor)]
    pub fn new(
        description: String,
        payout_units_per_payin_unit: String,
        payin: WasmPayinDetails,
        payout: WasmPayoutDetails,
        required_claims: Option<WasmPresentationDefinition>,
        cancellation: WasmCancellationDetails,
    ) -> Self {
        Self {
            inner: OfferingData {
                description,
                payout_units_per_payin_unit,
                payin: payin.into(),
                payout: payout.into(),
                required_claims: required_claims.map(|rc| rc.into()),
                cancellation: cancellation.into(),
            },
        }
    }

    #[wasm_bindgen(getter)]
    pub fn description(&self) -> String {
        self.inner.description.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn payout_units_per_payin_unit(&self) -> String {
        self.inner.payout_units_per_payin_unit.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn payin(&self) -> WasmPayinDetails {
        WasmPayinDetails {
            inner: self.inner.payin.clone(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn payout(&self) -> WasmPayoutDetails {
        WasmPayoutDetails {
            inner: self.inner.payout.clone(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn required_claims(&self) -> Option<WasmPresentationDefinition> {
        self.inner.required_claims.clone().map(|rc| rc.into())
    }

    #[wasm_bindgen(getter)]
    pub fn cancellation(&self) -> WasmCancellationDetails {
        WasmCancellationDetails {
            inner: self.inner.cancellation.clone(),
        }
    }
}

#[wasm_bindgen]
pub struct WasmPayinDetails {
    inner: PayinDetails,
}

impl From<WasmPayinDetails> for PayinDetails {
    fn from(value: WasmPayinDetails) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmPayinDetails {
    #[wasm_bindgen(constructor)]
    pub fn new(
        currency_code: String,
        methods: Vec<WasmPayinMethod>,
        min: Option<String>,
        max: Option<String>,
    ) -> Self {
        Self {
            inner: PayinDetails {
                currency_code,
                min,
                max,
                methods: methods.into_iter().map(|m| m.into()).collect(),
            },
        }
    }

    #[wasm_bindgen(getter)]
    pub fn currency_code(&self) -> String {
        self.inner.currency_code.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn methods(&self) -> Vec<WasmPayinMethod> {
        self.inner
            .methods
            .clone()
            .into_iter()
            .map(|m| m.into())
            .collect()
    }

    #[wasm_bindgen(getter)]
    pub fn min(&self) -> Option<String> {
        self.inner.min.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn max(&self) -> Option<String> {
        self.inner.max.clone()
    }
}

#[wasm_bindgen]
pub struct WasmPayinMethod {
    inner: PayinMethod,
}

impl From<WasmPayinMethod> for PayinMethod {
    fn from(value: WasmPayinMethod) -> Self {
        value.inner
    }
}

impl From<PayinMethod> for WasmPayinMethod {
    fn from(value: PayinMethod) -> Self {
        Self { inner: value }
    }
}

#[wasm_bindgen]
impl WasmPayinMethod {
    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(constructor)]
    pub fn new(
        kind: String,
        name: Option<String>,
        description: Option<String>,
        group: Option<String>,
        required_payment_details: JsValue,
        fee: Option<String>,
        min: Option<String>,
        max: Option<String>,
    ) -> Result<WasmPayinMethod> {
        let required_payment_details = if required_payment_details.is_undefined() {
            None
        } else {
            Some(
                serde_wasm_bindgen::from_value::<serde_json::Value>(required_payment_details)
                    .map_err(|err| JsValue::from(err.to_string()))?,
            )
        };
        Ok(Self {
            inner: PayinMethod {
                kind,
                name,
                description,
                group,
                required_payment_details,
                fee,
                min,
                max,
            },
        })
    }

    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> String {
        self.inner.kind.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> Option<String> {
        self.inner.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn description(&self) -> Option<String> {
        self.inner.description.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn group(&self) -> Option<String> {
        self.inner.group.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn required_payment_details(&self) -> Result<JsValue> {
        match &self.inner.required_payment_details {
            Some(pd) => {
                let value = serde_wasm_bindgen::to_value(pd)?;
                Ok(convert_to_object(value)?)
            }
            None => Ok(JsValue::UNDEFINED),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn fee(&self) -> Option<String> {
        self.inner.fee.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn min(&self) -> Option<String> {
        self.inner.min.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn max(&self) -> Option<String> {
        self.inner.max.clone()
    }
}

#[wasm_bindgen]
pub struct WasmPayoutDetails {
    inner: PayoutDetails,
}

impl From<WasmPayoutDetails> for PayoutDetails {
    fn from(value: WasmPayoutDetails) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmPayoutDetails {
    #[wasm_bindgen(constructor)]
    pub fn new(
        currency_code: String,
        methods: Vec<WasmPayoutMethod>,
        min: Option<String>,
        max: Option<String>,
    ) -> Self {
        Self {
            inner: PayoutDetails {
                currency_code,
                min,
                max,
                methods: methods.into_iter().map(|m| m.into()).collect(),
            },
        }
    }

    #[wasm_bindgen(getter)]
    pub fn currency_code(&self) -> String {
        self.inner.currency_code.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn methods(&self) -> Vec<WasmPayoutMethod> {
        self.inner
            .methods
            .clone()
            .into_iter()
            .map(|m| m.into())
            .collect()
    }

    #[wasm_bindgen(getter)]
    pub fn min(&self) -> Option<String> {
        self.inner.min.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn max(&self) -> Option<String> {
        self.inner.max.clone()
    }
}

#[wasm_bindgen]
pub struct WasmPayoutMethod {
    inner: PayoutMethod,
}

impl From<WasmPayoutMethod> for PayoutMethod {
    fn from(value: WasmPayoutMethod) -> Self {
        value.inner
    }
}

impl From<PayoutMethod> for WasmPayoutMethod {
    fn from(value: PayoutMethod) -> Self {
        Self { inner: value }
    }
}

#[wasm_bindgen]
impl WasmPayoutMethod {
    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(constructor)]
    pub fn new(
        kind: String,
        estimated_settlement_time: i64,
        name: Option<String>,
        description: Option<String>,
        group: Option<String>,
        required_payment_details: JsValue,
        fee: Option<String>,
        min: Option<String>,
        max: Option<String>,
    ) -> Result<WasmPayoutMethod> {
        let required_payment_details = if required_payment_details.is_undefined() {
            None
        } else {
            Some(
                serde_wasm_bindgen::from_value::<serde_json::Value>(required_payment_details)
                    .map_err(|err| JsValue::from(err.to_string()))?,
            )
        };

        Ok(Self {
            inner: PayoutMethod {
                kind,
                name,
                description,
                group,
                required_payment_details,
                fee,
                min,
                max,
                estimated_settlement_time,
            },
        })
    }

    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> String {
        self.inner.kind.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn estimated_settlement_time(&self) -> i64 {
        self.inner.estimated_settlement_time
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> Option<String> {
        self.inner.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn description(&self) -> Option<String> {
        self.inner.description.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn group(&self) -> Option<String> {
        self.inner.group.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn required_payment_details(&self) -> Result<JsValue> {
        match &self.inner.required_payment_details {
            Some(pd) => {
                let value = serde_wasm_bindgen::to_value(pd)?;
                Ok(convert_to_object(value)?)
            }
            None => Ok(JsValue::UNDEFINED),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn fee(&self) -> Option<String> {
        self.inner.fee.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn min(&self) -> Option<String> {
        self.inner.min.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn max(&self) -> Option<String> {
        self.inner.max.clone()
    }
}

#[wasm_bindgen]
pub struct WasmCancellationDetails {
    inner: CancellationDetails,
}

impl From<WasmCancellationDetails> for CancellationDetails {
    fn from(value: WasmCancellationDetails) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmCancellationDetails {
    #[wasm_bindgen(constructor)]
    pub fn new(enabled: bool, terms_url: Option<String>, terms: Option<String>) -> Self {
        Self {
            inner: CancellationDetails {
                enabled,
                terms_url,
                terms,
            },
        }
    }

    #[wasm_bindgen(getter)]
    pub fn enabled(&self) -> bool {
        self.inner.enabled
    }

    #[wasm_bindgen(getter)]
    pub fn terms_url(&self) -> Option<String> {
        self.inner.terms_url.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn terms(&self) -> Option<String> {
        self.inner.terms.clone()
    }
}
