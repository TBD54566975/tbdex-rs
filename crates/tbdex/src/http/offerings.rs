use crate::{
    json::{FromJson, ToJson},
    resources::offering::Offering,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetOfferingsResponse {
    pub data: Vec<Offering>,
}
impl FromJson for GetOfferingsResponse {}
impl ToJson for GetOfferingsResponse {}
