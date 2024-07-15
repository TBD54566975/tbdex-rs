use crate::{
    messages::{rfq::Rfq, MessageError},
    resources::{balance::Balance, offering::Offering},
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Error as SerdeJsonError;

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum HttpError {
    #[error("serde json error {0}")]
    SerdeJson(String),
    #[error(transparent)]
    Message(#[from] MessageError),
}

impl From<SerdeJsonError> for HttpError {
    fn from(err: SerdeJsonError) -> Self {
        HttpError::SerdeJson(err.to_string())
    }
}

type Result<T> = std::result::Result<T, HttpError>;

// TODO consider utilizing this throughout the entire codebase (move to a global space)
pub trait JsonDeserializer: Sized + DeserializeOwned {
    fn from_json_string(json: &str) -> Result<Self> {
        serde_json::from_str(json).map_err(HttpError::from)
    }
}

pub trait JsonSerializer: Serialize {
    fn to_json_string(&self) -> Result<String> {
        serde_json::to_string(self).map_err(HttpError::from)
    }
}

#[derive(Serialize, Deserialize)]
pub struct GetOfferingsResponse {
    pub data: Vec<Offering>,
}
impl JsonDeserializer for GetOfferingsResponse {}
impl JsonSerializer for GetOfferingsResponse {}

#[derive(Serialize, Deserialize)]
pub struct GetBalancesResponse {
    pub data: Vec<Balance>,
}
impl JsonDeserializer for GetBalancesResponse {}
impl JsonSerializer for GetBalancesResponse {}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateExchangeRequestBody {
    pub message: Rfq,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
}
impl JsonDeserializer for CreateExchangeRequestBody {}
impl JsonSerializer for CreateExchangeRequestBody {}
