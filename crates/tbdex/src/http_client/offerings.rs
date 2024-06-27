use super::{get_service_endpoint, Result};
use crate::resources::offering::Offering;
use reqwest::blocking::get;
use serde::Deserialize;

#[derive(Deserialize)]
struct GetOfferingsResponse {
    data: Vec<Offering>,
}

pub fn get_offerings(pfi_did_uri: &str) -> Result<Vec<Offering>> {
    let service_endpoint = get_service_endpoint(pfi_did_uri)?;
    let offerings_endpoint = format!("{}/offerings", service_endpoint);
    let response = get(offerings_endpoint)?.text()?;

    // TODO handle error response

    let offerings_response = serde_json::from_str::<GetOfferingsResponse>(&response)?;
    // 🚧 TODO for some reason the call to the latter json schema and sig verification are both failing
    // for offering in &offerings_response.data {
    //     offering.verify()?;
    // }

    Ok(offerings_response.data)
}
