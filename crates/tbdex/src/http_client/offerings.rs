use super::{get_json, get_service_endpoint, Result};
use crate::{http::offerings::GetOfferingsResponseBody, resources::offering::Offering};

pub fn get_offerings(pfi_did_uri: &str) -> Result<Vec<Offering>> {
    let service_endpoint = get_service_endpoint(pfi_did_uri)?;
    let offerings_endpoint = format!("{}/offerings", service_endpoint);
    let get_offerings_response_body =
        get_json::<GetOfferingsResponseBody>(&offerings_endpoint, None)?;

    for offering in &get_offerings_response_body.data {
        offering.verify()?;
    }

    Ok(get_offerings_response_body.data)
}
