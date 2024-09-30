use crate::errors::{map_http_std_err, Result};
use async_trait::async_trait;
use http_std::{Client, Error as HttpStdError, FetchOptions, Method, Response};
use js_sys::Promise;
use serde::Deserialize;
use std::{
    collections::HashMap,
    future::Future,
    pin::Pin,
    str::FromStr,
    sync::Arc,
    task::{Context, Poll},
};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        typescript_type = "{ fetch: (url: string, options?: WasmFetchOptions) => Promise<WasmResponse> }"
    )]
    pub type ForeignFetch;

    #[wasm_bindgen(method)]
    fn fetch(this: &ForeignFetch, url: &str, options: Option<WasmFetchOptions>) -> Promise;
}

struct SendJsFuture(JsFuture);

/**
 * TODO
 * [KW]:
 *    this is not thread safe and could cause issues
 *    the solution is to implement message passing across threads
 */
unsafe impl Send for SendJsFuture {}

impl Future for SendJsFuture {
    type Output = std::result::Result<JsValue, JsValue>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        unsafe { self.map_unchecked_mut(|s| &mut s.0) }.poll(cx)
    }
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

#[async_trait]
impl Client for ConcreteForeignFetch {
    async fn fetch(&self, url: &str, options: Option<FetchOptions>) -> http_std::Result<Response> {
        let wasm_options = options.map(WasmFetchOptions::from);

        let wasm_response_promise = self.0.fetch(url, wasm_options);
        let response_jsvalue = SendJsFuture(JsFuture::from(wasm_response_promise))
            .await
            .map_err(|e| {
                HttpStdError::Unknown(format!(
                    "rust future resolution error {}",
                    js_sys::JSON::stringify(&e)
                        .unwrap_or_else(|_| JsValue::from_str("null").into())
                        .as_string()
                        .unwrap_or_else(|| "null".to_string())
                ))
            })?;

        let response =
            serde_wasm_bindgen::from_value::<Response>(response_jsvalue).map_err(|e| {
                HttpStdError::Unknown(format!("rust WasmResponse from JsValue error {}", e))
            })?;

        Ok(response)
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

#[wasm_bindgen]
impl WasmFetchOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(
        method: Option<String>,
        headers: JsValue,
        body: Option<Vec<u8>>,
    ) -> Result<WasmFetchOptions> {
        let method = if let Some(m) = method {
            Some(Method::from_str(&m).map_err(map_http_std_err)?)
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

#[derive(Deserialize)]
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
