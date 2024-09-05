use crate::{
    errors::{Result, TbdexError},
    messages::rfq::Rfq,
};
use std::sync::Arc;
use tbdex::{
    http::exchanges::{
        CreateExchangeRequestBody as InnerCreateExchangeRequestBody,
        GetExchangeResponseBody as InnerGetExchangeResponseBody,
        GetExchangesResponseBody as InnerGetExchangesResponseBody, ReplyToMessage,
        ReplyToRequestBody as InnerReplyToRequestBody,
        UpdateExchangeRequestBody as InnerUpdateExchangeRequestBody, WalletUpdateMessage,
    },
    json::{FromJson, ToJson},
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
    pub fn new(response_body: GetExchangeResponseBodyData) -> Self {
        Self(response_body)
    }

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
                        Message::OrderInstructions(_) => MessageKind::OrderInstructions,
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
                .map(|i| Message::from_json_string(&i.json_serialized).map_err(TbdexError::from))
                .collect::<Result<Vec<Message>>>()?,
        };
        Ok(inner.to_json_string()?)
    }

    pub fn get_data(&self) -> GetExchangeResponseBodyData {
        self.0.clone()
    }
}

#[derive(Clone)]
pub struct GetExchangesResponseBodyData {
    pub data: Vec<String>,
}

pub struct GetExchangesResponseBody(pub GetExchangesResponseBodyData);

impl GetExchangesResponseBody {
    pub fn new(data: Vec<String>) -> Self {
        Self(GetExchangesResponseBodyData { data })
    }

    pub fn from_json_string(json: &str) -> Result<Self> {
        let inner = InnerGetExchangesResponseBody::from_json_string(json)?;
        Ok(Self(GetExchangesResponseBodyData { data: inner.data }))
    }

    pub fn to_json_string(&self) -> Result<String> {
        let inner = InnerGetExchangesResponseBody {
            data: self.0.data.clone(),
        };
        Ok(inner.to_json_string()?)
    }

    pub fn get_data(&self) -> GetExchangesResponseBodyData {
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
    pub fn new(message: Arc<Rfq>, reply_to: Option<String>) -> Self {
        Self(CreateExchangeRequestBodyData { message, reply_to })
    }

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
    pub fn new(kind: MessageKind, json_serialized_message: String) -> Self {
        Self(UpdateExchangeRequestBodyData {
            kind,
            json_serialized_message,
        })
    }

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
        let message = WalletUpdateMessage::from_json_string(&self.0.json_serialized_message)?;
        let inner = InnerUpdateExchangeRequestBody { message };
        Ok(inner.to_json_string()?)
    }

    pub fn get_data(&self) -> UpdateExchangeRequestBodyData {
        self.0.clone()
    }
}

#[derive(Clone)]
pub struct ReplyToRequestBodyData {
    pub kind: MessageKind, // not in APID but useful for bound code to deserialize json_serialized_message
    pub json_serialized_message: String,
}

pub struct ReplyToRequestBody(pub ReplyToRequestBodyData);

impl ReplyToRequestBody {
    pub fn new(kind: MessageKind, json_serialized_message: String) -> Self {
        Self(ReplyToRequestBodyData {
            kind,
            json_serialized_message,
        })
    }

    pub fn from_json_string(json: &str) -> Result<Self> {
        let inner = InnerReplyToRequestBody::from_json_string(json)?;
        let kind = match inner.message {
            ReplyToMessage::Quote(_) => MessageKind::Quote,
            ReplyToMessage::OrderInstructions(_) => MessageKind::OrderInstructions,
            ReplyToMessage::OrderStatus(_) => MessageKind::OrderStatus,
            ReplyToMessage::Close(_) => MessageKind::Close,
        };
        Ok(Self(ReplyToRequestBodyData {
            kind,
            json_serialized_message: inner.message.to_json_string()?,
        }))
    }

    pub fn to_json_string(&self) -> Result<String> {
        let message = ReplyToMessage::from_json_string(&self.0.json_serialized_message)?;
        let inner = InnerReplyToRequestBody { message };
        Ok(inner.to_json_string()?)
    }

    pub fn get_data(&self) -> ReplyToRequestBodyData {
        self.0.clone()
    }
}
