pub mod balances;
pub mod exchanges;
pub mod offerings;

use crate::{
    errors::{Result, TbdexError},
    http::ErrorResponseBody,
};
use reqwest::{blocking::Client, Method, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use std::time::{Duration, SystemTime};
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

pub(crate) fn get_service_endpoint(pfi_did_uri: &str) -> Result<String> {
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

fn send_request<T: Serialize, U: DeserializeOwned>(
    url: &str,
    method: Method,
    body: Option<&T>,
    access_token: Option<String>,
) -> Result<Option<U>> {
    let client = Client::new();
    let mut request = client.request(method.clone(), url);

    if let Some(token) = &access_token {
        request = request.bearer_auth(token);
    }

    if let Some(body) = &body {
        request = request.json(body);
    }

    let response = request.send()?;

    let response_status = response.status();
    let response_text = response.text()?;

    crate::log_dbg!(|| {
        format!(
            "httpclient sent request {} {}, has access token {}, with body {}, \
            response status {}, response text {}",
            method,
            url,
            access_token.is_some(),
            match &body {
                Some(b) => serde_json::to_string_pretty(b)
                    .unwrap_or_else(|_| String::from("error serializing the body")),
                None => String::default(),
            },
            response_status,
            match serde_json::from_str::<serde_json::Value>(&response_text) {
                Ok(json) =>
                    serde_json::to_string_pretty(&json).unwrap_or_else(|_| response_text.clone()),
                Err(_) => response_text.clone(),
            }
        )
    });

    if !response_status.is_success() {
        if response_status.as_u16() >= 400 {
            let error_response_body = serde_json::from_str::<ErrorResponseBody>(&response_text)?;
            return Err(error_response_body.into());
        }

        return Err(TbdexError::HttpClient(format!(
            "unsuccessful http response {} {}",
            response_status, response_text
        )));
    }

    if response_status == StatusCode::ACCEPTED {
        return Ok(None);
    }

    let response_body = serde_json::from_str::<U>(&response_text)?;
    Ok(Some(response_body))
}
