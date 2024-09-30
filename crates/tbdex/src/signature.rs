use crate::errors::{Result, TbdexError};
use base64::{engine::general_purpose, Engine};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use web5::{dids::bearer_did::BearerDid, jose::Jws};

fn compute_digest(value: &Value) -> Result<Vec<u8>> {
    let canonical_string = serde_jcs::to_string(value)?;
    let mut hasher = Sha256::new();
    hasher.update(canonical_string.as_bytes());
    Ok(hasher.finalize().to_vec())
}

pub fn sign(bearer_did: &BearerDid, metadata: &Value, data: &Value) -> Result<String> {
    let mut combined = Map::new();
    combined.insert("metadata".to_string(), metadata.clone());
    combined.insert("data".to_string(), data.clone());

    let digest = compute_digest(&Value::Object(combined))?;

    // TODO verification method defaults to first
    let jws = Jws::from_payload(&digest, bearer_did, None)?;

    Ok(jws.detached_compact_jws)
}

pub fn verify(metadata: &Value, data: &Value, detached_compact_jws: &str) -> Result<()> {
    // re-attach the payload
    let mut combined = Map::new();
    combined.insert("metadata".to_string(), metadata.clone());
    combined.insert("data".to_string(), data.clone());
    let digest = compute_digest(&Value::Object(combined))?;
    let payload = general_purpose::URL_SAFE_NO_PAD.encode(digest);

    let parts: Vec<&str> = detached_compact_jws.split('.').collect();
    if parts.len() != 3 {
        return Err(TbdexError::Jose(format!(
            "detached compact jws wrong number of parts {}",
            parts.len()
        )));
    }
    let compact_jws = format!("{}.{}.{}", parts[0], payload, parts[2]);

    let _ = Jws::from_compact_jws(&compact_jws, true)?;

    Ok(())
}
