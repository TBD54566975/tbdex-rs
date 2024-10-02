use crate::{
    errors::{map_err, Result},
    messages::rfq::WasmRfq,
};
use tbdex::{
    http::exchanges::{
        CreateExchangeRequestBody, GetExchangeResponseBody, GetExchangesResponseBody,
        ReplyToMessage, ReplyToRequestBody, UpdateExchangeRequestBody, WalletUpdateMessage,
    },
    json::{FromJson, ToJson},
    messages::{Message, MessageKind},
};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub struct WasmGetExchangeResponseBody {
    inner: GetExchangeResponseBody,
}

#[wasm_bindgen]
impl WasmGetExchangeResponseBody {
    #[wasm_bindgen(constructor)]
    pub fn new(data: Vec<JsValue>) -> Result<WasmGetExchangeResponseBody> {
        let messages: Vec<Message> = data
            .into_iter()
            .map(|m| serde_wasm_bindgen::from_value::<Message>(m).map_err(|e| e.into()))
            .collect::<Result<Vec<Message>>>()?;

        Ok(Self {
            inner: GetExchangeResponseBody { data: messages },
        })
    }

    #[wasm_bindgen(getter)]
    pub fn data(&self) -> Result<Vec<WasmJsonSerializedMessage>> {
        let mut data: Vec<WasmJsonSerializedMessage> = Vec::new();

        for message in &self.inner.data {
            let kind = match message {
                Message::Rfq(_) => MessageKind::Rfq.to_string(),
                Message::Quote(_) => MessageKind::Quote.to_string(),
                Message::Order(_) => MessageKind::Order.to_string(),
                Message::OrderInstructions(_) => MessageKind::OrderInstructions.to_string(),
                Message::Cancel(_) => MessageKind::Cancel.to_string(),
                Message::OrderStatus(_) => MessageKind::OrderStatus.to_string(),
                Message::Close(_) => MessageKind::Close.to_string(),
            };

            data.push(WasmJsonSerializedMessage {
                kind,
                json: message.to_json_string().map_err(map_err)?,
            });
        }

        Ok(data)
    }

    pub fn from_json_string(json: &str) -> Result<WasmGetExchangeResponseBody> {
        Ok(Self {
            inner: GetExchangeResponseBody::from_json_string(json).map_err(map_err)?,
        })
    }

    pub fn to_json_string(&self) -> Result<String> {
        self.inner.to_json_string().map_err(map_err)
    }
}

#[wasm_bindgen]
pub struct WasmJsonSerializedMessage {
    kind: String,
    json: String,
}

#[wasm_bindgen]
impl WasmJsonSerializedMessage {
    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> String {
        self.kind.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn json(&self) -> String {
        self.json.clone()
    }
}

#[wasm_bindgen]
pub struct WasmGetExchangesResponseBody {
    inner: GetExchangesResponseBody,
}

#[wasm_bindgen]
impl WasmGetExchangesResponseBody {
    #[wasm_bindgen(constructor)]
    pub fn new(data: Vec<String>) -> Self {
        Self {
            inner: GetExchangesResponseBody { data },
        }
    }

    #[wasm_bindgen(getter)]
    pub fn data(&self) -> Vec<String> {
        self.inner.data.clone()
    }

    pub fn from_json_string(json: &str) -> Result<WasmGetExchangesResponseBody> {
        Ok(Self {
            inner: GetExchangesResponseBody::from_json_string(json).map_err(map_err)?,
        })
    }

    pub fn to_json_string(&self) -> Result<String> {
        self.inner.to_json_string().map_err(map_err)
    }
}

#[wasm_bindgen]
pub struct WasmCreateExchangeRequestBody {
    inner: CreateExchangeRequestBody,
}

#[wasm_bindgen]
impl WasmCreateExchangeRequestBody {
    #[wasm_bindgen(constructor)]
    pub fn new(message: WasmRfq, reply_to: Option<String>) -> Self {
        Self {
            inner: CreateExchangeRequestBody {
                message: message.into(),
                reply_to,
            },
        }
    }

    #[wasm_bindgen(getter)]
    pub fn message(&self) -> WasmRfq {
        self.inner.message.clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn reply_to(&self) -> Option<String> {
        self.inner.reply_to.clone()
    }

    pub fn from_json_string(json: &str) -> Result<WasmCreateExchangeRequestBody> {
        Ok(Self {
            inner: CreateExchangeRequestBody::from_json_string(json).map_err(map_err)?,
        })
    }

    pub fn to_json_string(&self) -> Result<String> {
        self.inner.to_json_string().map_err(map_err)
    }
}

#[wasm_bindgen]
pub struct WasmUpdateExchangeRequestBody {
    inner: UpdateExchangeRequestBody,
}

#[wasm_bindgen]
impl WasmUpdateExchangeRequestBody {
    #[wasm_bindgen(constructor)]
    pub fn new(message: JsValue) -> Result<WasmUpdateExchangeRequestBody> {
        let message = serde_wasm_bindgen::from_value::<WalletUpdateMessage>(message)?;
        Ok(Self {
            inner: UpdateExchangeRequestBody { message },
        })
    }

    #[wasm_bindgen(getter)]
    pub fn data(&self) -> Result<WasmJsonSerializedMessage> {
        let kind = match self.inner.message {
            WalletUpdateMessage::Order(_) => MessageKind::Order.to_string(),
            WalletUpdateMessage::Cancel(_) => MessageKind::Cancel.to_string(),
        };
        let json = self.inner.message.to_json_string().map_err(map_err)?;

        Ok(WasmJsonSerializedMessage { kind, json })
    }

    pub fn from_json_string(json: &str) -> Result<WasmUpdateExchangeRequestBody> {
        Ok(Self {
            inner: UpdateExchangeRequestBody::from_json_string(json).map_err(map_err)?,
        })
    }

    pub fn to_json_string(&self) -> Result<String> {
        self.inner.to_json_string().map_err(map_err)
    }
}

#[wasm_bindgen]
pub struct WasmReplyToRequestBody {
    inner: ReplyToRequestBody,
}

#[wasm_bindgen]
impl WasmReplyToRequestBody {
    #[wasm_bindgen(constructor)]
    pub fn new(message: JsValue) -> Result<WasmReplyToRequestBody> {
        let message = serde_wasm_bindgen::from_value::<ReplyToMessage>(message)?;
        Ok(Self {
            inner: ReplyToRequestBody { message },
        })
    }

    #[wasm_bindgen(getter)]
    pub fn data(&self) -> Result<WasmJsonSerializedMessage> {
        let kind = match self.inner.message {
            ReplyToMessage::Quote(_) => MessageKind::Quote.to_string(),
            ReplyToMessage::OrderInstructions(_) => MessageKind::OrderInstructions.to_string(),
            ReplyToMessage::OrderStatus(_) => MessageKind::OrderStatus.to_string(),
            ReplyToMessage::Close(_) => MessageKind::Close.to_string(),
        };
        let json = self.inner.message.to_json_string().map_err(map_err)?;

        Ok(WasmJsonSerializedMessage { kind, json })
    }

    pub fn from_json_string(json: &str) -> Result<WasmReplyToRequestBody> {
        Ok(Self {
            inner: ReplyToRequestBody::from_json_string(json).map_err(map_err)?,
        })
    }

    pub fn to_json_string(&self) -> Result<String> {
        self.inner.to_json_string().map_err(map_err)
    }
}
