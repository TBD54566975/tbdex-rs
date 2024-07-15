use crate::{errors::Result, resources::offering::Offering};
use std::sync::Arc;
use tbdex::{
    http::{
        offerings::GetOfferingsResponse as InnerGetOfferingsResponse, JsonDeserializer,
        JsonSerializer,
    },
    resources::offering::Offering as InnerOffering,
};

#[derive(Clone)]
pub struct GetOfferingsResponseBodyData {
    pub data: Vec<Arc<Offering>>,
}

pub struct GetOfferingsResponseBody(pub GetOfferingsResponseBodyData);

impl GetOfferingsResponseBody {
    pub fn new(offerings: Vec<Arc<Offering>>) -> Self {
        Self(GetOfferingsResponseBodyData { data: offerings })
    }

    pub fn from_json_string(json: &str) -> Result<Self> {
        let inner = InnerGetOfferingsResponse::from_json_string(json)?;
        let data = inner
            .data
            .iter()
            .map(|o| Arc::new(Offering::from_inner(o.clone())))
            .collect::<Vec<Arc<Offering>>>();
        Ok(Self(GetOfferingsResponseBodyData { data }))
    }

    pub fn to_json_string(&self) -> Result<String> {
        let inner = InnerGetOfferingsResponse {
            data: self
                .0
                .data
                .iter()
                .map(|o| o.to_inner())
                .collect::<Result<Vec<InnerOffering>>>()?,
        };
        Ok(inner.to_json_string()?)
    }

    pub fn get_data(&self) -> GetOfferingsResponseBodyData {
        self.0.clone()
    }
}
