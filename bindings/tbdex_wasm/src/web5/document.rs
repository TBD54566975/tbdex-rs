use super::jwk::WasmJwk;
use crate::errors::{map_web5_err, Result};
use wasm_bindgen::prelude::wasm_bindgen;
use web5::{
    dids::data_model::{
        document::Document, service::Service, verification_method::VerificationMethod,
    },
    json::{FromJson, ToJson},
};

#[wasm_bindgen]
pub struct WasmDocument {
    inner: Document,
}

impl From<WasmDocument> for Document {
    fn from(value: WasmDocument) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmDocument {
    #[wasm_bindgen(constructor)]
    pub fn new(
        id: String,
        context: Option<Vec<String>>,
        controller: Option<Vec<String>>,
        also_known_as: Option<Vec<String>>,
        verification_method: Vec<WasmVerificationMethod>,
        authentication: Option<Vec<String>>,
        assertion_method: Option<Vec<String>>,
        key_agreement: Option<Vec<String>>,
        capability_invocation: Option<Vec<String>>,
        capability_delegation: Option<Vec<String>>,
        service: Option<Vec<WasmService>>,
    ) -> Self {
        Self {
            inner: Document {
                id,
                context,
                controller,
                also_known_as,
                verification_method: verification_method
                    .into_iter()
                    .map(|wvm| wvm.into())
                    .collect(),
                authentication,
                assertion_method,
                key_agreement,
                capability_invocation,
                capability_delegation,
                service: match service {
                    None => None,
                    Some(wss) => Some(wss.into_iter().map(|ws| ws.into()).collect()),
                },
            },
        }
    }

    #[wasm_bindgen]
    pub fn from_json_string(json: &str) -> Result<WasmDocument> {
        Ok(Self {
            inner: Document::from_json_string(json).map_err(map_web5_err)?,
        })
    }

    #[wasm_bindgen]
    pub fn to_json_string(&self) -> Result<String> {
        Ok(self.inner.to_json_string().map_err(map_web5_err)?)
    }
}

#[wasm_bindgen]
pub struct WasmVerificationMethod {
    inner: VerificationMethod,
}

impl From<WasmVerificationMethod> for VerificationMethod {
    fn from(value: WasmVerificationMethod) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmVerificationMethod {
    #[wasm_bindgen(constructor)]
    pub fn new(id: String, r#type: String, controller: String, public_key_jwk: WasmJwk) -> Self {
        Self {
            inner: VerificationMethod {
                id,
                r#type,
                controller,
                public_key_jwk: public_key_jwk.into(),
            },
        }
    }
}

#[wasm_bindgen]
pub struct WasmService {
    inner: Service,
}

impl From<WasmService> for Service {
    fn from(value: WasmService) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmService {
    #[wasm_bindgen(constructor)]
    pub fn new(id: String, r#type: String, service_endpoint: Vec<String>) -> Self {
        Self {
            inner: Service {
                id,
                r#type,
                service_endpoint,
            },
        }
    }
}
