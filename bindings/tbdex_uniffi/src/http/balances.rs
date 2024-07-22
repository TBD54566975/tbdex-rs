use crate::{errors::Result, resources::balance::Balance};
use std::sync::Arc;
use tbdex::{
    http::balances::GetBalancesResponseBody as InnerGetBalancesResponseBody,
    json::{FromJson, ToJson},
    resources::balance::Balance as InnerBalance,
};

#[derive(Clone)]
pub struct GetBalancesResponseBodyData {
    pub data: Vec<Arc<Balance>>,
}

pub struct GetBalancesResponseBody(pub GetBalancesResponseBodyData);

impl GetBalancesResponseBody {
    pub fn new(balances: Vec<Arc<Balance>>) -> Self {
        Self(GetBalancesResponseBodyData { data: balances })
    }

    pub fn from_json_string(json: &str) -> Result<Self> {
        let inner = InnerGetBalancesResponseBody::from_json_string(json)?;
        let data = inner
            .data
            .iter()
            .map(|o| Arc::new(Balance::from_inner(o.clone())))
            .collect::<Vec<Arc<Balance>>>();
        Ok(Self(GetBalancesResponseBodyData { data }))
    }

    pub fn to_json_string(&self) -> Result<String> {
        let inner = InnerGetBalancesResponseBody {
            data: self
                .0
                .data
                .iter()
                .map(|o| o.to_inner())
                .collect::<Result<Vec<InnerBalance>>>()?,
        };
        Ok(inner.to_json_string()?)
    }

    pub fn get_data(&self) -> GetBalancesResponseBodyData {
        self.0.clone()
    }
}
