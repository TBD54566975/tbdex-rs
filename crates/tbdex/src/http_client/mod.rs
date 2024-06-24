pub mod balances;
pub mod exchanges;
pub mod offerings;

use crate::jose::Signer;
use josekit::{jwt::JwtPayload, JoseError as JosekitError};
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeJsonError;
use std::time::{Duration, SystemTime};
use uuid::Uuid;
use web5::apid::dids::bearer_did::BearerDidError;

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum HttpClientError {
    #[error("reqwest error {0}")]
    ReqwestError(String),
    #[error("serde json error {0}")]
    SerdeJson(String),
    #[error(transparent)]
    BearerDid(#[from] BearerDidError),
    #[error("jose error {0}")]
    Jose(String),
}

impl From<ReqwestError> for HttpClientError {
    fn from(err: ReqwestError) -> Self {
        HttpClientError::ReqwestError(err.to_string())
    }
}

impl From<SerdeJsonError> for HttpClientError {
    fn from(err: SerdeJsonError) -> Self {
        HttpClientError::SerdeJson(err.to_string())
    }
}

impl From<JosekitError> for HttpClientError {
    fn from(err: JosekitError) -> Self {
        HttpClientError::Jose(err.to_string())
    }
}

type Result<T> = std::result::Result<T, HttpClientError>;

fn generate_access_token(
    pfi_did_uri: &str,
    client_did_uri: &str,
    jose_signer: Signer,
) -> Result<String> {
    let now = SystemTime::now();
    let exp = now + Duration::from_secs(60);

    let mut payload = JwtPayload::new();
    payload.set_audience(vec![pfi_did_uri]);
    payload.set_issuer(client_did_uri);
    payload.set_issued_at(&now);
    payload.set_expires_at(&exp);
    payload.set_jwt_id(Uuid::new_v4().to_string());

    let access_token = jose_signer.sign_jwt(&payload)?;

    Ok(access_token)
}
