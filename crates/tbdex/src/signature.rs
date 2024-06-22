use josekit::{
    jws::{
        alg::eddsa::EddsaJwsAlgorithm, serialize_compact, JwsAlgorithm, JwsHeader, JwsSigner,
        JwsVerifier,
    },
    JoseError,
};
use serde_json::Error as SerdeJsonError;
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    fmt::{Debug, Formatter},
    sync::Arc,
};
use web5::apid::{
    crypto::jwk::Jwk,
    dids::{
        bearer_did::{BearerDid, BearerDidError},
        resolution::{
            resolution_metadata::ResolutionMetadataError, resolution_result::ResolutionResult,
        },
    },
    dsa::{ed25519::Ed25519Verifier, DsaError, Signer as Web5Signer, Verifier},
};

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum SignatureError {
    #[error("jose error {0}")]
    Jose(String),
    #[error(transparent)]
    ResolutionMetadata(#[from] ResolutionMetadataError),
    #[error(transparent)]
    BearerDid(#[from] BearerDidError),
    #[error("serde json error {0}")]
    SerdeJson(String),
}

impl From<SerdeJsonError> for SignatureError {
    fn from(err: SerdeJsonError) -> Self {
        SignatureError::SerdeJson(err.to_string())
    }
}

impl From<JoseError> for SignatureError {
    fn from(err: JoseError) -> Self {
        SignatureError::Jose(err.to_string())
    }
}

type Result<T> = std::result::Result<T, SignatureError>;

#[derive(Clone)]
struct JosekitSigner {
    pub kid: String,
    pub web5_signer: Arc<dyn Web5Signer>,
}

impl JwsSigner for JosekitSigner {
    fn algorithm(&self) -> &dyn JwsAlgorithm {
        &EddsaJwsAlgorithm::Eddsa
    }

    fn key_id(&self) -> Option<&str> {
        Some(&self.kid)
    }

    fn signature_len(&self) -> usize {
        64
    }

    fn sign(&self, message: &[u8]) -> core::result::Result<Vec<u8>, JoseError> {
        self.web5_signer
            .sign(message)
            // 🚧 improve error message semantics
            .map_err(|err| JoseError::InvalidSignature(err.into()))
    }

    fn box_clone(&self) -> Box<dyn JwsSigner> {
        Box::new(self.clone())
    }
}

impl Debug for JosekitSigner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Signer").field("kid", &self.kid).finish()
    }
}

fn canonicalize_json(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut sorted_map = Map::new();
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            for key in keys {
                sorted_map.insert(key.clone(), canonicalize_json(&map[key]));
            }
            Value::Object(sorted_map)
        }
        _ => value.clone(),
    }
}

fn compute_digest(value: &Value) -> Result<Vec<u8>> {
    let canonical_json = canonicalize_json(value);
    let canonical_string = serde_json::to_string(&canonical_json)?;
    let mut hasher = Sha256::new();
    hasher.update(canonical_string.as_bytes());
    Ok(hasher.finalize().to_vec())
}

pub fn sign(bearer_did: BearerDid, metadata: Value, data: Value) -> Result<String> {
    let key_id = bearer_did.document.verification_method[0].id.clone();
    let web5_signer = bearer_did.get_signer(key_id.clone())?;
    let jose_signer = JosekitSigner {
        kid: key_id,
        web5_signer,
    };

    let mut combined = Map::new();
    combined.insert("metadata".to_string(), metadata);
    combined.insert("data".to_string(), data);

    let digest = compute_digest(&Value::Object(combined))?;

    let compact_jws = serialize_compact(&digest, &JwsHeader::new(), &jose_signer)?;
    let parts: Vec<&str> = compact_jws.split('.').collect();
    let detached_compact_jws = format!("{}..{}", parts[0], parts[2]);

    Ok(detached_compact_jws)
}

#[derive(Clone, Debug)]
pub struct JosekitVerifier {
    pub kid: String,
    pub public_jwk: Jwk,
}

impl JwsVerifier for JosekitVerifier {
    fn algorithm(&self) -> &dyn JwsAlgorithm {
        &EddsaJwsAlgorithm::Eddsa
    }

    fn key_id(&self) -> Option<&str> {
        Some(self.kid.as_str())
    }

    fn verify(&self, message: &[u8], signature: &[u8]) -> core::result::Result<(), JoseError> {
        let verifier = Ed25519Verifier::new(self.public_jwk.clone());
        let result = verifier
            .verify(message, signature)
            .map_err(|e| JoseError::InvalidSignature(e.into()))?;

        match result {
            true => Ok(()),
            false => Err(JoseError::InvalidSignature(
                // 🚧 improve error message semantics
                DsaError::VerificationFailure("ed25519 verification failed".to_string()).into(),
            )),
        }
    }

    fn box_clone(&self) -> Box<dyn JwsVerifier> {
        Box::new(self.clone())
    }
}

fn create_selector<'a>(
    verifiers: &'a HashMap<String, Arc<JosekitVerifier>>,
) -> impl Fn(&JwsHeader) -> core::result::Result<Option<&'a dyn JwsVerifier>, JoseError> + 'a {
    move |header: &JwsHeader| -> core::result::Result<Option<&'a dyn JwsVerifier>, JoseError> {
        let kid = header.key_id().ok_or_else(|| {
            JoseError::InvalidJwsFormat(SignatureError::Jose("missing kid".to_string()).into())
        })?;

        let verifier = verifiers.get(kid).ok_or_else(|| {
            JoseError::InvalidJwsFormat(
                SignatureError::Jose("verification method not found".to_string()).into(),
            )
        })?;

        Ok(Some(&**verifier))
    }
}

use base64::{engine::general_purpose, Engine};

pub fn verify(
    did_uri: &str,
    metadata: Value,
    data: Value,
    detached_compact_jws: String,
) -> Result<()> {
    // re-attach the payload
    let mut combined = Map::new();
    combined.insert("metadata".to_string(), metadata);
    combined.insert("data".to_string(), data);
    let digest = compute_digest(&Value::Object(combined))?;
    let payload = general_purpose::URL_SAFE_NO_PAD.encode(digest);

    let parts: Vec<&str> = detached_compact_jws.split('.').collect();
    if parts.len() != 3 {
        return Err(SignatureError::Jose(
            "detached compact jws wrong number of parts".to_string(),
        ));
    }
    let message = format!("{}.{}.{}", parts[0], payload, parts[2]);

    let resolution_result = ResolutionResult::new(did_uri);
    match resolution_result.resolution_metadata.error {
        Some(e) => Err(SignatureError::ResolutionMetadata(e)),
        None => {
            let document = resolution_result
                .document
                .ok_or(SignatureError::ResolutionMetadata(
                    ResolutionMetadataError::InternalError,
                ))?;

            let verifiers: HashMap<String, Arc<JosekitVerifier>> = document
                .verification_method
                .iter()
                .map(|method| {
                    (
                        method.id.clone(),
                        Arc::new(JosekitVerifier {
                            kid: method.id.clone(),
                            public_jwk: method.public_key_jwk.clone(),
                        }),
                    )
                })
                .collect();

            let selector = create_selector(&verifiers);

            josekit::jws::deserialize_compact_with_selector(message, selector)?;

            Ok(())
        }
    }
}
