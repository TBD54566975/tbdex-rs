pub mod balances;
pub mod exchanges;
pub mod offerings;

use tbdex::{
    http::{ErrorDetail as InnerErrorDetail, ErrorResponseBody as InnerErrorResponseBody},
    json::{FromJson, ToJson},
};

use crate::errors::Result;

pub struct ErrorResponseBody(pub InnerErrorResponseBody);

impl ErrorResponseBody {
    pub fn new(message: String, details: Option<Vec<InnerErrorDetail>>) -> Self {
        Self(InnerErrorResponseBody { message, details })
    }

    pub fn from_json_string(json: &str) -> Result<Self> {
        let inner = InnerErrorResponseBody::from_json_string(json)?;
        Ok(Self(inner))
    }

    pub fn to_json_string(&self) -> Result<String> {
        Ok(self.0.to_json_string()?)
    }

    pub fn get_data(&self) -> InnerErrorResponseBody {
        self.0.clone()
    }
}
