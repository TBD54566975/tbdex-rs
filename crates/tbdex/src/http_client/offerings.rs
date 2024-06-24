use super::Result;
use crate::resources::offering::Offering;
use reqwest::blocking::get;
use serde::Deserialize;

#[derive(Deserialize)]
struct GetOfferingsResponse {
    data: Vec<Offering>,
}

pub fn get_offerings(_pfi_did: String) -> Result<Vec<Offering>> {
    // TODO resolve pfi did for service endpoint; waiting on did:dht resolution
    let endpoint = "http://localhost:9000/offerings";
    // TODO the above

    let response = get(endpoint)?.text()?;

    let offerings_response: GetOfferingsResponse = serde_json::from_str(&response)?;

    Ok(offerings_response.data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_offerings() {
        let offerings = get_offerings("pfi_did".to_string()).unwrap();
        assert_ne!(0, offerings.len())

        // let formatted_response: serde_json::Value = serde_json::from_str(&response)?;
        // println!("{}", serde_json::to_string_pretty(&formatted_response)?);
    }
}
