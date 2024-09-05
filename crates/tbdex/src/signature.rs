use crate::errors::{Result, TbdexError};
use crate::jose::{Signer, Verifier};
use base64::{engine::general_purpose, Engine};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use web5::dids::bearer_did::BearerDid;
use web5::dids::resolution::resolution_result::ResolutionResult;
use web5::errors::Web5Error;

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

    // default to first VM
    let key_id = bearer_did.document.verification_method[0].id.clone();
    let web5_signer = bearer_did.get_signer(&key_id)?;
    let jose_signer = Signer {
        kid: key_id,
        web5_signer,
    };
    let detached_compact_jws = jose_signer.sign_detached_compact_jws(&digest)?;

    Ok(detached_compact_jws)
}

pub fn verify(
    did_uri: &str,
    metadata: &Value,
    data: &Value,
    detached_compact_jws: &str,
) -> Result<()> {
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
    let message = format!("{}.{}.{}", parts[0], payload, parts[2]);

    let resolution_result = ResolutionResult::resolve(did_uri);
    match resolution_result.resolution_metadata.error {
        Some(e) => Err(TbdexError::Web5Error(Web5Error::Resolution(e))),
        None => {
            let document = resolution_result.document.ok_or(TbdexError::Jose(
                "did resolution result must contain document".to_string(),
            ))?;

            Verifier::verify_compact_jws(document, message)?;

            Ok(())
        }
    }
}
