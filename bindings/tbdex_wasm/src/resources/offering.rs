use crate::{
    errors::{map_err, Result},
    web5::presentation_definition::WasmPresentationDefinition,
};
use serde_wasm_bindgen::from_value;
use tbdex::{
    json::FromJson,
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

#[wasm_bindgen]
impl WasmOffering {
    pub fn create(
        from: &str,
        data: WasmOfferingData,
        protocol: Option<String>,
    ) -> Result<WasmOffering> {
        Ok(Self {
            inner: Offering::create(from, &data.into(), protocol).map_err(map_err)?,
        })
    }

    pub fn from_json_string(json: &str) -> Result<WasmOffering> {
        Ok(Self {
            inner: Offering::from_json_string(json).map_err(map_err)?,
        })
    }

    pub fn verify(&self) -> Result<()> {
        self.inner.verify().map_err(map_err)
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
                required_claims: required_claims.and_then(|rc| Some(rc.into())),
                cancellation: cancellation.into(),
            },
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

#[wasm_bindgen]
impl WasmPayinMethod {
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
                from_value::<serde_json::Value>(required_payment_details)
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

#[wasm_bindgen]
impl WasmPayoutMethod {
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
                from_value::<serde_json::Value>(required_payment_details)
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
}
