pub mod balances;
pub mod exchanges;
pub mod offerings;

use crate::errors::{Result, TbdexError};
use http_std::FetchOptions;
use serde::{de::DeserializeOwned, Serialize};
use chrono::{Duration, Utc};
use std::time::SystemTime;
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
    let now = Utc::now();
    let exp = now + Duration::seconds(60);

    let now_system_time: SystemTime = now.into();
    let exp_system_time: SystemTime = exp.into();

    let claims = &JwtClaims {
        aud: Some(vec![pfi_did_uri.to_string()]),
        iss: Some(bearer_did.did.uri.clone()),
        iat: Some(now_system_time),
        exp: Some(exp_system_time),
        jti: Some(Uuid::new_v4().to_string()),
        ..Default::default()
    };
    // TODO default to first vm
    let jwt = Jwt::from_claims(claims, bearer_did, None)?;

    Ok(jwt.compact_jws)
}

async fn get_service_endpoint(pfi_did_uri: &str) -> Result<String> {
    let resolution_result = ResolutionResult::resolve(pfi_did_uri).await;

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

pub(crate) async fn get_json<T: DeserializeOwned>(
    url: &str,
    access_token: Option<String>,
) -> Result<T> {
    let options = access_token.map(|access_token| FetchOptions {
        headers: Some(
            [(
                "Authorization".to_string(),
                format!("Bearer {}", access_token),
            )]
            .into_iter()
            .collect(),
        ),
        ..Default::default()
    });
    let response = http_std::fetch(url, options).await?;

    if !(200..300).contains(&response.status_code) {
        return Err(TbdexError::Http(format!(
            "http error status code {} for url {}",
            response.status_code, url
        )));
    }

    let json = serde_json::from_slice::<T>(&response.body)?;

    Ok(json)
}

pub(crate) async fn post_json<T: Serialize>(url: &str, body: &T) -> Result<()> {
    let body = serde_json::to_vec(body)?;

    let response = http_std::fetch(
        url,
        Some(FetchOptions {
            method: Some(http_std::Method::Post),
            headers: Some(
                [("Content-Type".to_string(), "application/json".to_string())]
                    .into_iter()
                    .collect(),
            ),
            body: Some(body),
        }),
    )
    .await?;

    if !(200..300).contains(&response.status_code) {
        return Err(TbdexError::Http(format!(
            "http error status code {} for url {}",
            response.status_code, url
        )));
    }

    Ok(())
}

pub(crate) async fn put_json<T: Serialize>(url: &str, body: &T) -> Result<()> {
    let body = serde_json::to_vec(body)?;

    let response = http_std::fetch(
        url,
        Some(FetchOptions {
            method: Some(http_std::Method::Put),
            headers: Some(
                [("Content-Type".to_string(), "application/json".to_string())]
                    .into_iter()
                    .collect(),
            ),
            body: Some(body),
        }),
    )
    .await?;

    if !(200..300).contains(&response.status_code) {
        return Err(TbdexError::Http(format!(
            "http error status code {} for url {}",
            response.status_code, url
        )));
    }

    Ok(())
}
