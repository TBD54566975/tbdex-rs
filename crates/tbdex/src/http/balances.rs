use crate::{
    json::{FromJson, ToJson},
    resources::balance::Balance,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetBalancesResponse {
    pub data: Vec<Balance>,
}
impl FromJson for GetBalancesResponse {}
impl ToJson for GetBalancesResponse {}
