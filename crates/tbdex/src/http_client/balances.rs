use super::{generate_access_token, get_service_endpoint, Result};
use crate::resources::balance::Balance;
use reqwest::blocking::Client;
use serde::Deserialize;
use web5::dids::bearer_did::BearerDid;

#[derive(Deserialize)]
struct GetBalancesResponse {
    data: Vec<Balance>,
}

pub fn get_balances(pfi_did_uri: &str, bearer_did: &BearerDid) -> Result<Vec<Balance>> {
    let service_endpoint = get_service_endpoint(pfi_did_uri)?;
    let balances_endpoint = format!("{}/balances", service_endpoint);

    let access_token = generate_access_token(pfi_did_uri, bearer_did)?;

    let response = Client::new()
        .get(balances_endpoint)
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
