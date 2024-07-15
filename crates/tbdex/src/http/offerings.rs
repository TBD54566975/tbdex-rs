use super::{JsonDeserializer, JsonSerializer};
use crate::resources::offering::Offering;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetOfferingsResponse {
    pub data: Vec<Offering>,
}
impl JsonDeserializer for GetOfferingsResponse {}
impl JsonSerializer for GetOfferingsResponse {}
