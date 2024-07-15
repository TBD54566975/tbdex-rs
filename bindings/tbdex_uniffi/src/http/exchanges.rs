use crate::{
    errors::{Result, RustCoreError},
    messages::rfq::Rfq,
};
use std::sync::Arc;
use tbdex::{
    http::{
        exchanges::{
            CreateExchangeRequestBody as InnerCreateExchangeRequestBody,
            GetExchangeResponseBody as InnerGetExchangeResponseBody,
            UpdateExchangeRequestBody as InnerUpdateExchangeRequestBody, WalletUpdateMessage,
        },
        JsonDeserializer, JsonSerializer,
    },
    messages::{Message, MessageKind},
};

#[derive(Clone)]
pub struct GetExchangeResponseBodyDataSerializedMessage {
    // not in APID but needed on bound side to deserialize the json_serialized
    pub kind: MessageKind,
    pub json_serialized: String,
}

#[derive(Clone)]
pub struct GetExchangeResponseBodyData {
    pub data: Vec<GetExchangeResponseBodyDataSerializedMessage>,
}

pub struct GetExchangeResponseBody(pub GetExchangeResponseBodyData);

impl GetExchangeResponseBody {
    pub fn from_json_string(json: &str) -> Result<Self> {
        let inner = InnerGetExchangeResponseBody::from_json_string(json)?;
        let data = inner
            .data
            .iter()
            .map(|i| {
                let json_serialized = i.to_json_string()?;
                Ok(GetExchangeResponseBodyDataSerializedMessage {
                    kind: match i {
                        Message::Rfq(_) => MessageKind::Rfq,
                        Message::Quote(_) => MessageKind::Quote,
                        Message::Order(_) => MessageKind::Order,
                        Message::Cancel(_) => MessageKind::Cancel,
                        Message::OrderStatus(_) => MessageKind::OrderStatus,
                        Message::Close(_) => MessageKind::Close,
                    },
                    json_serialized,
                })
            })
            .collect::<Result<Vec<GetExchangeResponseBodyDataSerializedMessage>>>()?;
        Ok(Self(GetExchangeResponseBodyData { data }))
    }

    pub fn to_json_string(&self) -> Result<String> {
        let inner = InnerGetExchangeResponseBody {
            data: self
                .0
                .data
                .iter()
                .map(|i| {
                    Message::from_json_string(&i.json_serialized)
                        .map_err(|e| RustCoreError::from(e))
                })
                .collect::<Result<Vec<Message>>>()?,
        };
        Ok(inner.to_json_string()?)
    }

    pub fn get_data(&self) -> GetExchangeResponseBodyData {
        self.0.clone()
    }
}

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

#[derive(Clone)]
pub struct UpdateExchangeRequestBodyData {
    pub kind: MessageKind, // not in APID but useful for bound code to deserialize json_serialized_message
    pub json_serialized_message: String,
}

pub struct UpdateExchangeRequestBody(pub UpdateExchangeRequestBodyData);

impl UpdateExchangeRequestBody {
    pub fn from_json_string(json: &str) -> Result<Self> {
        let inner = InnerUpdateExchangeRequestBody::from_json_string(json)?;
        let kind = match inner.message {
            WalletUpdateMessage::Order(_) => MessageKind::Order,
            WalletUpdateMessage::Cancel(_) => MessageKind::Cancel,
        };
        Ok(Self(UpdateExchangeRequestBodyData {
            kind,
            json_serialized_message: inner.message.to_json_string()?,
        }))
    }

    pub fn to_json_string(&self) -> Result<String> {
        let inner =
            InnerUpdateExchangeRequestBody::from_json_string(&self.0.json_serialized_message)?;
        Ok(inner.to_json_string()?)
    }

    pub fn get_data(&self) -> UpdateExchangeRequestBodyData {
        self.0.clone()
    }
}
