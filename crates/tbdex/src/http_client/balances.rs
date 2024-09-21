use super::{generate_access_token, get_json, get_service_endpoint, Result};
use crate::{http::balances::GetBalancesResponseBody, resources::balance::Balance};
use web5::dids::bearer_did::BearerDid;

pub fn get_balances(pfi_did_uri: &str, bearer_did: &BearerDid) -> Result<Vec<Balance>> {
    let service_endpoint = get_service_endpoint(pfi_did_uri)?;
    let balances_endpoint = format!("{}/balances", service_endpoint);

    let access_token = generate_access_token(pfi_did_uri, bearer_did)?;
    let get_balances_response_body =
        get_json::<GetBalancesResponseBody>(&balances_endpoint, Some(access_token))?;

    for balance in &get_balances_response_body.data {
        balance.verify()?;
    }

    Ok(get_balances_response_body.data)
}
