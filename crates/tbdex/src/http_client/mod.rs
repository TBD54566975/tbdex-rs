pub mod balances;
pub mod exchanges;
pub mod offerings;

use crate::errors::{Result, TbdexError};
use lazy_static::lazy_static;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};
use uuid::Uuid;
use web5::{
    dids::{
        bearer_did::BearerDid,
        resolution::{
            resolution_metadata::ResolutionMetadataError, resolution_result::ResolutionResult,
        },
    },
    errors::Web5Error,
    jose::{Jwt, JwtClaims},
};

// todo use generalized feature flag, not target_arch, b/c we'll do injection for all foreign bindings
#[cfg(not(target_arch = "wasm32"))]
use reqwest::Error as ReqwestError;

#[cfg(not(target_arch = "wasm32"))]
impl From<ReqwestError> for TbdexError {
    fn from(err: ReqwestError) -> Self {
        TbdexError::Http(err.to_string())
    }
}

fn generate_access_token(pfi_did_uri: &str, bearer_did: &BearerDid) -> Result<String> {
    let now = SystemTime::now();
    let exp = now + Duration::from_secs(60);

    let claims = &JwtClaims {
        aud: Some(vec![pfi_did_uri.to_string()]),
        iss: Some(bearer_did.did.uri.clone()),
        iat: Some(now),
        exp: Some(exp),
        jti: Some(Uuid::new_v4().to_string()),
        ..Default::default()
    };
    // TODO default to first vm
    let jwt = Jwt::from_claims(claims, bearer_did, None)?;

    Ok(jwt.compact_jws)
}

fn get_service_endpoint(pfi_did_uri: &str) -> Result<String> {
    let resolution_result = ResolutionResult::resolve(pfi_did_uri);

    let endpoint = match &resolution_result.document {
        None => {
            return Err(match resolution_result.resolution_metadata.error {
                Some(e) => TbdexError::Web5Error(Web5Error::Resolution(e)),
                None => TbdexError::Web5Error(Web5Error::Resolution(
                    ResolutionMetadataError::InternalError,
                )),
            })
        }
        Some(d) => match &d.service {
            None => {
                return Err(TbdexError::HttpClient(format!(
                    "missing service endpoint {}",
                    pfi_did_uri
                )))
            }
            Some(s) => s
                .iter()
                .find(|s| s.r#type == *"PFI")
                .ok_or(TbdexError::HttpClient(format!(
                    "missing service endpoint {}",
                    pfi_did_uri
                )))?
                .service_endpoint
                .first()
                .ok_or(TbdexError::HttpClient(format!(
                    "missing service endpoint {}",
                    pfi_did_uri
                )))?
                .clone(),
        },
    };

    Ok(endpoint)
}

fn add_pagination(
    endpoint: &str,
    pagination_offset: Option<i64>,
    pagination_limit: Option<i64>,
) -> String {
    let mut query_string = String::new();

    if let Some(offset) = pagination_offset {
        query_string.push_str(&format!("?page[offset]={}", offset));
    }

    if let Some(limit) = pagination_limit {
        if query_string.is_empty() {
            query_string.push_str(&format!("?page[limit]={}", limit));
        } else {
            query_string.push_str(&format!("&page[limit]={}", limit));
        }
    }

    format!("{}{}", endpoint, query_string)
}

pub struct HttpResponse {
    pub status_code: u16,
    pub body: Vec<u8>,
}

pub trait HttpClient: Sync + Send {
    fn get(&self, url: &str, access_token: Option<String>) -> Result<HttpResponse>;
    fn post(&self, url: &str, body: &[u8]) -> Result<HttpResponse>;
    fn put(&self, url: &str, body: &[u8]) -> Result<HttpResponse>;
}

// ---

#[cfg(not(target_arch = "wasm32"))]
lazy_static! {
    pub static ref HTTP_CLIENT: Mutex<Arc<dyn HttpClient>> = Mutex::new(Arc::new(RustHttpClient));
}

#[cfg(target_arch = "wasm32")]
lazy_static! {
    pub static ref HTTP_CLIENT: Mutex<Arc<dyn HttpClient>> =
        Mutex::new(Arc::new(ForeignEmptyHttpClient));
}

pub fn set_http_client(client: Arc<dyn HttpClient>) {
    let mut global_client = HTTP_CLIENT.lock().unwrap();
    *global_client = client;
}

pub fn get_http_client() -> Arc<dyn HttpClient> {
    let client = HTTP_CLIENT.lock().unwrap();
    client.clone()
}

// ---

pub(crate) fn get_json<T: DeserializeOwned>(url: &str, access_token: Option<String>) -> Result<T> {
    let http_client = get_http_client();
    let http_response = http_client.get(url, access_token)?;

    if !(200..300).contains(&http_response.status_code) {
        return Err(TbdexError::Http(format!(
            "http error status code {} for url {}",
            http_response.status_code, url
        )));
    }

    let json = serde_json::from_slice::<T>(&http_response.body)?;

    Ok(json)
}

pub(crate) fn post_json<T: Serialize>(url: &str, body: &T) -> Result<()> {
    let body = serde_json::to_vec(body)?;

    let http_client = get_http_client();
    let http_response = http_client.post(url, &body)?;

    if !(200..300).contains(&http_response.status_code) {
        return Err(TbdexError::Http(format!(
            "http error status code {} for url {}",
            http_response.status_code, url
        )));
    }

    Ok(())
}

pub(crate) fn put_json<T: Serialize>(url: &str, body: &T) -> Result<()> {
    let body = serde_json::to_vec(body)?;

    let http_client = get_http_client();
    let http_response = http_client.put(url, &body)?;

    if !(200..300).contains(&http_response.status_code) {
        return Err(TbdexError::Http(format!(
            "http error status code {} for url {}",
            http_response.status_code, url
        )));
    }

    Ok(())
}

// ---

#[cfg(not(target_arch = "wasm32"))]
pub(crate) struct RustHttpClient;

#[cfg(not(target_arch = "wasm32"))]
impl HttpClient for RustHttpClient {
    fn get(&self, url: &str, access_token: Option<String>) -> Result<HttpResponse> {
        let client = reqwest::blocking::Client::new();
        let mut request = client.request(reqwest::Method::GET, url);
        if let Some(access_token) = &access_token {
            request = request.bearer_auth(access_token);
        }

        let response = request.send()?;

        let status_code = response.status().as_u16();
        let body = response.bytes()?.to_vec();

        Ok(HttpResponse { status_code, body })
    }

    fn post(&self, url: &str, body: &[u8]) -> Result<HttpResponse> {
        let client = reqwest::blocking::Client::new();

        let response = client
            .post(url)
            .header("Content-Type", "application/json")
            .body(body.to_vec())
            .send()?;

        let status_code = response.status().as_u16();
        let body = response.bytes()?.to_vec();

        Ok(HttpResponse { status_code, body })
    }

    fn put(&self, url: &str, body: &[u8]) -> Result<HttpResponse> {
        let client = reqwest::blocking::Client::new();

        let response = client
            .put(url)
            .header("Content-Type", "application/json")
            .body(body.to_vec())
            .send()?;

        let status_code = response.status().as_u16();
        let body = response.bytes()?.to_vec();

        Ok(HttpResponse { status_code, body })
    }
}

// ---

#[cfg(target_arch = "wasm32")]
pub struct ForeignEmptyHttpClient;

#[cfg(target_arch = "wasm32")]
impl HttpClient for ForeignEmptyHttpClient {
    fn get(&self, url: &str, access_token: Option<String>) -> Result<HttpResponse> {
        Err(TbdexError::Http("http client not set".to_string()))
    }

    fn post(&self, url: &str, body: &[u8]) -> Result<HttpResponse> {
        Err(TbdexError::Http("http client not set".to_string()))
    }

    fn put(&self, url: &str, body: &[u8]) -> Result<HttpResponse> {
        Err(TbdexError::Http("http client not set".to_string()))
    }
}
