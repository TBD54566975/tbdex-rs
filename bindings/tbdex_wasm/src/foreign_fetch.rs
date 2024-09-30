use crate::errors::{map_err, Result};
use http_std::{Client, FetchOptions, Method, Response};
use js_sys::Promise;
use std::{collections::HashMap, sync::Arc};
use tbdex::errors::TbdexError;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        typescript_type = "{ fetch: (url: string, options?: WasmFetchOptions) => Promise<WasmResponse> }"
    )]
    pub type ForeignFetchAsync;

    #[wasm_bindgen(method)]
    fn fetch(this: &ForeignFetchAsync, url: &str, options: Option<WasmFetchOptions>) -> Promise;
}

#[wasm_bindgen]
pub async fn proof_of_concept_foreign_fetch_async(foreign_fetch: ForeignFetchAsync) -> Result<()> {
    let promise = foreign_fetch.fetch("https://example.com", None);
    let response_js = JsFuture::from(promise).await?;
    // let response: WasmResponse = response_js.into();

    web_sys::console::log_1(&JsValue::from_str(&format!(
        "http response code {:?}",
        response_js
    )));

    Ok(())
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        typescript_type = "{ fetch: (url: string, options?: WasmFetchOptions) => WasmResponse }"
    )]
    pub type ForeignFetch;

    #[wasm_bindgen(method)]
    fn fetch(this: &ForeignFetch, url: &str, options: Option<WasmFetchOptions>) -> WasmResponse;
}

#[wasm_bindgen]
pub fn proof_of_concept_foreign_fetch(foreign_fetch: ForeignFetch) {
    let response = foreign_fetch.fetch("https://example.com", None);
    web_sys::console::log_1(&JsValue::from_str(&format!(
        "http response code {}",
        response.status_code()
    )));
}

pub struct ConcreteForeignFetch(ForeignFetch);

/**
 * TODO
 * [KW]:
 *    this is not thread safe and could cause issues
 *    the solution is to implement message passing across threads
 */
unsafe impl Send for ConcreteForeignFetch {}
unsafe impl Sync for ConcreteForeignFetch {}

impl Client for ConcreteForeignFetch {
    fn fetch(&self, url: &str, options: Option<FetchOptions>) -> http_std::Result<Response> {
        let wasm_options = options.map(WasmFetchOptions::from);
        let wasm_response = self.0.fetch(url, wasm_options);
        Ok(wasm_response.into())
    }
}

#[wasm_bindgen]
pub fn set_http_client(foreign_fetch: ForeignFetch) {
    http_std::set_client(Arc::new(ConcreteForeignFetch(foreign_fetch)))
}

#[wasm_bindgen]
pub struct WasmFetchOptions {
    inner: FetchOptions,
}

impl From<FetchOptions> for WasmFetchOptions {
    fn from(value: FetchOptions) -> Self {
        WasmFetchOptions { inner: value }
    }
}

// TODO move to web5-rs
// impl FromStr for Method {
//     type Err = TbdexError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s.to_ascii_uppercase().as_ref() {
//             "GET" => Ok(Method::Get),
//             "POST" => Ok(Method::Post),
//             "PUT" => Ok(Method::Put),
//             _ => return Err(TbdexError::HttpClient(format!("unknown method {}", s))),
//         }
//     }
// }
fn method_from_str(method: &str) -> Result<Method> {
    match method.to_ascii_uppercase().as_ref() {
        "GET" => Ok(Method::Get),
        "POST" => Ok(Method::Post),
        "PUT" => Ok(Method::Put),
        _ => Err(map_err(TbdexError::HttpClient(format!(
            "unknown method {}",
            method
        )))),
    }
}

#[wasm_bindgen]
impl WasmFetchOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(
        method: Option<String>,
        headers: JsValue,
        body: Option<Vec<u8>>,
    ) -> Result<WasmFetchOptions> {
        let method = if let Some(m) = method {
            Some(method_from_str(&m)?)
        } else {
            None
        };

        let headers = if headers.is_undefined() {
            None
        } else {
            serde_wasm_bindgen::from_value(headers).unwrap_or(Some(HashMap::new()))
        };

        Ok(Self {
            inner: FetchOptions {
                method,
                headers,
                body,
            },
        })
    }

    #[wasm_bindgen(getter)]
    pub fn method(&self) -> Option<String> {
        self.inner.method.as_ref().map(|m| m.to_string())
    }

    #[wasm_bindgen(getter)]
    pub fn headers(&self) -> JsValue {
        match &self.inner.headers {
            Some(map) => serde_wasm_bindgen::to_value(map).unwrap_or(JsValue::undefined()),
            None => JsValue::undefined(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn body(&self) -> Option<Vec<u8>> {
        self.inner.body.clone()
    }
}

#[wasm_bindgen]
pub struct WasmResponse {
    inner: Response,
}

impl From<WasmResponse> for Response {
    fn from(value: WasmResponse) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmResponse {
    #[wasm_bindgen(constructor)]
    pub fn new(status_code: u16, headers: JsValue, body: Vec<u8>) -> Self {
        Self {
            inner: Response {
                status_code,
                headers: serde_wasm_bindgen::from_value(headers).unwrap_or(HashMap::new()),
                body,
            },
        }
    }

    #[wasm_bindgen(getter)]
    pub fn status_code(&self) -> u16 {
        self.inner.status_code
    }

    #[wasm_bindgen(getter)]
    pub fn headers(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.inner.headers).unwrap_or(JsValue::undefined())
    }

    // TODO body should be optional in web5-rs
    #[wasm_bindgen(getter)]
    pub fn body(&self) -> Vec<u8> {
        self.inner.body.clone()
    }
}
