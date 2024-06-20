use josekit::{
    jws::{alg::eddsa::EddsaJwsAlgorithm, serialize_compact, JwsAlgorithm, JwsHeader, JwsSigner},
    JoseError,
};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use std::{
    fmt::{Debug, Formatter},
    sync::Arc,
};
use web5::apid::{dids::bearer_did::BearerDid, dsa::Signer as Web5Signer};

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

    fn sign(&self, message: &[u8]) -> Result<Vec<u8>, JoseError> {
        self.web5_signer
            .sign(message)
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

impl Clone for JosekitSigner {
    fn clone(&self) -> Self {
        Self {
            kid: self.kid.clone(),
            web5_signer: self.web5_signer.clone(),
        }
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

fn compute_digest(value: &Value) -> Vec<u8> {
    let canonical_json = canonicalize_json(value);
    let canonical_string = serde_json::to_string(&canonical_json).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(canonical_string.as_bytes());
    hasher.finalize().to_vec()
}

pub fn sign(bearer_did: BearerDid, metadata: Value, data: Value) -> String {
    let key_id = bearer_did.document.verification_method[0].id.clone();
    let web5_signer = bearer_did.get_signer(key_id.clone()).unwrap(); // 🚧
    let jose_signer = JosekitSigner {
        kid: key_id,
        web5_signer,
    };

    let mut combined = Map::new();
    combined.insert("metadata".to_string(), metadata);
    combined.insert("data".to_string(), data);

    let digest = compute_digest(&Value::Object(combined));

    let compact_jws = serialize_compact(&digest, &JwsHeader::new(), &jose_signer).unwrap(); // 🚧
    let parts: Vec<&str> = compact_jws.split('.').collect();
    let detached_compact_jws = format!("{}..{}", parts[0], parts[2]);

    detached_compact_jws
}
