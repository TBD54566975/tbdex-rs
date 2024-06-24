use super::{generate_access_token, Result};
use crate::jose::Signer;
use crate::resources::balance::Balance;
use reqwest::blocking::Client;
use serde::Deserialize;
use web5::apid::crypto::key_managers::key_manager::KeyManager;
use web5::apid::dids::bearer_did::BearerDid;

use web5::apid::crypto::{jwk::Jwk, key_managers::in_memory_key_manager::InMemoryKeyManager};

#[derive(Deserialize)]
struct GetBalancesResponse {
    data: Vec<Balance>,
}

pub fn get_balances(pfi_did: String, _requestor_did: BearerDid) -> Result<Vec<Balance>> {
    // TODO did:dht resolution not functional
    let key_manager = InMemoryKeyManager::new();
    let public_jwk = key_manager
        .import_private_jwk(Jwk {
            crv: "Ed25519".to_string(),
            alg: "EdDSA".to_string(),
            kty: "OKP".to_string(),
            x: "kW2-CfY0XmGTVLurk7BJ14Mqc4L-oJpD3jH5ZmwxyUw".to_string(),
            y: None,
            d: Some("jVOdpSIN-DhddW_XVnDipukuzu6-8zieXQtkECZYJ04".to_string()),
        })
        .unwrap();
    let web5_signer = key_manager.get_signer(public_jwk).unwrap();
    let client_did_uri =
        "did:dht:1fs5hnxsgtxgdr4wzqi38cnj46b1whhn94ojwo66g8hsc5bt3fgy#0".to_string();
    let jose_signer = Signer {
        kid: client_did_uri.clone(),
        web5_signer,
    };
    let endpoint = "http://localhost:9000/balances";
    // TODO all of the above

    let access_token = generate_access_token(&pfi_did, &client_did_uri, jose_signer)?;

    let response = Client::new()
        .get(endpoint)
        .bearer_auth(access_token)
        .send()?
        .text()?;

    let balances_response: GetBalancesResponse = serde_json::from_str(&response)?;

    Ok(balances_response.data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn can_get_balances() {
        // TODO replace the below with did:dht once implemented
        let did_uri = "did:jwk:eyJrdHkiOiJPS1AiLCJ1c2UiOiJzaWciLCJjcnYiOiJFZDI1NTE5Iiwia2lkIjoiVnRTSFhQbEtEdzFFRW9PajVYTjNYV2hqU1BZVk52WC1lNHZqUk8weVlKQSIsIngiOiJpejcwc3ZTTHhOWmhzRHhlSlFfam5PVmJYM0tGTmtjQmNNaldqWm1YRXNBIiwiYWxnIjoiRWREU0EifQ";
        let key_manager = InMemoryKeyManager::new();
        let bearer_did = BearerDid::new(did_uri, Arc::new(key_manager)).unwrap();
        // TODO the above

        let pfi_did = "did:dht:swit41ctrddy1s38c5j46yfgbxmwo1emau71zo5hn1tws1g63hiy".to_string();
        let balances = get_balances(pfi_did, bearer_did).unwrap();
        assert_ne!(0, balances.len());
    }
}
