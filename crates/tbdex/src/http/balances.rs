use super::{JsonDeserializer, JsonSerializer};
use crate::resources::balance::Balance;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetBalancesResponse {
    pub data: Vec<Balance>,
}
impl JsonDeserializer for GetBalancesResponse {}
impl JsonSerializer for GetBalancesResponse {}
