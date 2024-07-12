mod http_client;
mod messages;
mod resources;

mod errors;

use crate::{
    errors::RustCoreError,
    http_client::{
        balances::get_balances,
        exchanges::{
            create_exchange, get_exchange, get_exchanges,
            request::{Body as HttpRequestBody, BodyData as HttpRequestBodyData},
            submit_close, submit_order, CreateExchangeRequestBody, CreateExchangeRequestBodyData,
            Exchange as ExchangeData, SubmitOrderRequestBody, SubmitOrderRequestBodyData,
        },
        offerings::get_offerings,
    },
    messages::{
        close::Close,
        order::Order,
        order_status::OrderStatus,
        quote::Quote,
        rfq::{data::Rfq as RfqData, Rfq},
    },
    resources::{
        balance::Balance,
        offering::{data::Offering as OfferingData, Offering},
    },
};
use tbdex::{
    messages::{
        close::{Close as CloseData, CloseData as CloseDataData},
        order::{Order as OrderData, OrderData as OrderDataData},
        order_status::{OrderStatus as OrderStatusData, OrderStatusData as OrderStatusDataData},
        quote::{
            PaymentInstruction as PaymentInstructionData, Quote as QuoteData,
            QuoteData as QuoteDataData, QuoteDetails as QuoteDetailsData,
        },
        MessageKind, MessageMetadata as MessageMetadataData,
    },
    resources::{
        balance::{Balance as BalanceData, BalanceData as BalanceDataData},
        ResourceKind, ResourceMetadata as ResourceMetadataData,
    },
};
use web5::{
    crypto::jwk::Jwk as JwkData,
    dids::{
        data_model::{
            document::Document as DocumentData, service::Service as ServiceData,
            verification_method::VerificationMethod as VerificationMethodData,
        },
        did::Did as DidData,
        portable_did::PortableDid as PortableDidData,
    },
};
use web5_uniffi_wrapper::{
    credentials::presentation_definition::PresentationDefinition,
    crypto::{dsa::Signer, in_memory_key_manager::InMemoryKeyManager, key_manager::KeyManager},
    dids::{
        bearer_did::{BearerDid, BearerDidData},
        portable_did::PortableDid,
    },
    errors::RustCoreError as Web5RustCoreError,
};

uniffi::include_scaffolding!("tbdex");
