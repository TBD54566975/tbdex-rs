use super::Result;
use crate::resources::offering::Offering;
use reqwest::blocking::get;
use serde::Deserialize;

#[derive(Deserialize)]
struct GetOfferingsResponse {
    data: Vec<Offering>,
}

pub fn get_offerings(_pfi_did_uri: &str) -> Result<Vec<Offering>> {
    // TODO resolve pfi did for service endpoint; waiting on did:dht resolution
    let endpoint = "http://localhost:9000/offerings";
    // TODO the above

    let response = get(endpoint)?.text()?;

    // TODO handle error response

    let offerings_response = serde_json::from_str::<GetOfferingsResponse>(&response)?;
    // TODO uncomment with did:dht resolution support
    // for offering in &offerings_response.data {
    //     offering.verify()?;
    // }

    Ok(offerings_response.data)
}
