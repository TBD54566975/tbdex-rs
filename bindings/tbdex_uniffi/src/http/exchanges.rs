use crate::{errors::Result, messages::rfq::Rfq};
use std::sync::Arc;
use tbdex::http::{
    CreateExchangeRequestBody as InnerCreateExchangeRequestBody, JsonDeserializer, JsonSerializer,
};

#[derive(Clone)]
pub struct CreateExchangeRequestBodyData {
    pub message: Arc<Rfq>,
    pub reply_to: Option<String>,
}

pub struct CreateExchangeRequestBody(pub CreateExchangeRequestBodyData);

impl CreateExchangeRequestBody {
    pub fn from_json_string(json: &str) -> Result<Self> {
        let inner = InnerCreateExchangeRequestBody::from_json_string(json)?;
        let rfq = Rfq::from_inner(inner.message);
        Ok(Self(CreateExchangeRequestBodyData {
            message: Arc::new(rfq),
            reply_to: inner.reply_to,
        }))
    }

    pub fn to_json_string(&self) -> Result<String> {
        let inner = InnerCreateExchangeRequestBody {
            message: self.0.message.to_inner()?,
            reply_to: self.0.reply_to.clone(),
        };
        Ok(inner.to_json_string()?)
    }

    pub fn get_data(&self) -> CreateExchangeRequestBodyData {
        self.0.clone()
    }
}
