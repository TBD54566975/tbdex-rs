use crate::{
    errors::Result,
    messages::{close::Close, order::Order, order_status::OrderStatus, quote::Quote, rfq::Rfq},
};
use std::sync::{Arc, RwLock};
use tbdex::http_client::exchanges::{
    CreateExchangeRequestBody as InnerCreateExchangeRequestBody, Exchange as InnerExchange,
    SubmitOrderRequestBody as InnerSubmitOrderRequestBody,
};
use web5_uniffi_wrapper::dids::bearer_did::BearerDid;

pub struct Exchange {
    pub rfq: Arc<Rfq>,
    pub quote: Option<Arc<Quote>>,
    pub order: Option<Arc<Order>>,
    pub order_statuses: Option<Vec<Arc<OrderStatus>>>,
    pub close: Option<Arc<Close>>,
}

impl Exchange {
    pub fn from_inner(inner: InnerExchange) -> Self {
        Self {
            rfq: Arc::new(Rfq(Arc::new(RwLock::new(inner.rfq.clone())))),
            quote: inner
                .quote
                .as_ref()
                .map(|q| Arc::new(Quote(Arc::new(RwLock::new(q.clone()))))),
            order: inner
                .order
                .as_ref()
                .map(|o| Arc::new(Order(Arc::new(RwLock::new(o.clone()))))),
            order_statuses: inner.order_statuses.as_ref().map(|os| {
                os.iter()
                    .map(|o| Arc::new(OrderStatus(Arc::new(RwLock::new(o.clone())))))
                    .collect::<Vec<_>>()
            }),
            close: inner
                .close
                .as_ref()
                .map(|c| Arc::new(Close(Arc::new(RwLock::new(c.clone()))))),
        }
    }
}

pub fn create_exchange(rfq: Arc<Rfq>, reply_to: Option<String>) -> Result<()> {
    tbdex::http_client::exchanges::create_exchange(&rfq.to_inner()?, reply_to)?;
    Ok(())
}

pub fn submit_order(order: Arc<Order>) -> Result<()> {
    tbdex::http_client::exchanges::submit_order(&order.get_data()?)?;
    Ok(())
}

pub fn submit_close(close: Arc<Close>) -> Result<()> {
    tbdex::http_client::exchanges::submit_close(&close.get_data()?)?;
    Ok(())
}

pub fn get_exchange(
    pfi_did_uri: String,
    bearer_did: Arc<BearerDid>,
    exchange_id: String,
) -> Result<Exchange> {
    let inner_exchange = tbdex::http_client::exchanges::get_exchange(
        &pfi_did_uri,
        &bearer_did.0.clone(),
        &exchange_id,
    )?;

    Ok(Exchange::from_inner(inner_exchange))
}

pub fn get_exchanges(pfi_did_uri: String, bearer_did: Arc<BearerDid>) -> Result<Vec<String>> {
    let exchange_ids =
        tbdex::http_client::exchanges::get_exchanges(&pfi_did_uri, &bearer_did.0.clone())?;
    Ok(exchange_ids)
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

    pub fn get_data(&self) -> CreateExchangeRequestBodyData {
        self.0.clone()
    }
}

#[derive(Clone)]
pub struct SubmitOrderRequestBodyData {
    pub message: Arc<Order>,
}

pub struct SubmitOrderRequestBody(pub SubmitOrderRequestBodyData);

impl SubmitOrderRequestBody {
    pub fn from_json_string(json: &str) -> Result<Self> {
        let inner = InnerSubmitOrderRequestBody::from_json_string(json)?;
        let message = Order::from_inner(inner.message);
        Ok(Self(SubmitOrderRequestBodyData {
            message: Arc::new(message),
        }))
    }

    pub fn get_data(&self) -> SubmitOrderRequestBodyData {
        self.0.clone()
    }
}

pub mod request {
    use super::*;
    use tbdex::{
        http_client::request::{Body as InnerBody, Message},
        messages::MessageKind,
    };

    #[derive(Clone)]
    pub struct BodyData {
        pub json_serialized_message: String,
        pub reply_to: Option<String>,
        // not in APID, but useful on bound side to determine deserialized type for json_serialized_message
        pub kind: MessageKind,
    }

    pub struct Body(pub InnerBody);

    impl Body {
        pub fn new(json_serialized_message: &str, reply_to: Option<String>) -> Result<Self> {
            let message = serde_json::from_str::<Message>(json_serialized_message)?;
            Ok(Self(InnerBody { message, reply_to }))
        }

        pub fn from_json_string(json: &str) -> Result<Self> {
            let inner_body = InnerBody::from_json_string(json)?;
            Ok(Self(inner_body))
        }

        pub fn get_data(&self) -> Result<BodyData> {
            let (kind, json_serialized_message) = match &self.0.message {
                Message::Rfq(rfq) => (MessageKind::Rfq, rfq.to_json()?),
                Message::Quote(quote) => (MessageKind::Quote, quote.to_json()?),
                Message::Order(order) => (MessageKind::Order, order.to_json()?),
                Message::Cancel(cancel) => (MessageKind::Cancel, cancel.to_json()?),
                Message::OrderStatus(order_status) => {
                    (MessageKind::OrderStatus, order_status.to_json()?)
                }
                Message::Close(close) => (MessageKind::Close, close.to_json()?),
                // TODO for error response
                Message::Text(txt) => (MessageKind::Rfq, txt.clone()),
            };

            Ok(BodyData {
                kind,
                json_serialized_message,
                reply_to: self.0.reply_to.clone(),
            })
        }

        pub fn to_json(&self) -> Result<String> {
            Ok(self.0.to_json()?)
        }
    }
}
