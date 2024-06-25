use super::{generate_access_token, Result};
use crate::resources::balance::Balance;
use reqwest::blocking::Client;
use serde::Deserialize;
use web5::apid::dids::bearer_did::BearerDid;

#[derive(Deserialize)]
struct GetBalancesResponse {
    data: Vec<Balance>,
}

pub fn get_balances(pfi_did_uri: &str, bearer_did: &BearerDid) -> Result<Vec<Balance>> {
    // TODO resolve pfi did for service endpoint; waiting on did:dht resolution
    let endpoint = "http://localhost:9000/balance";
    // TODO the above

    let access_token = generate_access_token(pfi_did_uri, bearer_did)?;

    let response = Client::new()
        .get(endpoint)
        .bearer_auth(access_token)
        .send()?
        .text()?;

    let balances_response: GetBalancesResponse = serde_json::from_str(&response)?;
    // TODO uncomment with did:dht resolution support
    // for balance in &balances_response.data {
    //     balance.verify()?;
    // }

    Ok(balances_response.data)
}
