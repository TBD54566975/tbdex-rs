mod http;
mod http_client;
mod messages;
mod resources;

mod errors;

use crate::{
    errors::TbdexSdkError,
    http::{
        balances::{GetBalancesResponseBody, GetBalancesResponseBodyData},
        exchanges::{
            CreateExchangeRequestBody, CreateExchangeRequestBodyData, GetExchangeResponseBody,
            GetExchangeResponseBodyData, GetExchangeResponseBodyDataSerializedMessage,
            GetExchangesResponseBody, GetExchangesResponseBodyData, ReplyToRequestBody,
            ReplyToRequestBodyData, UpdateExchangeRequestBody, UpdateExchangeRequestBodyData,
        },
        offerings::{GetOfferingsResponseBody, GetOfferingsResponseBodyData},
        ErrorResponseBody,
    },
    http_client::{
        balances::get_balances,
        exchanges::{
            create_exchange, get_exchange, get_exchange_ids, submit_cancel, submit_order,
            Exchange as ExchangeData,
        },
        offerings::get_offerings,
    },
    messages::{
        cancel::Cancel,
        close::Close,
        order::Order,
        order_instructions::OrderInstructions,
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
    http::{ErrorDetail as ErrorDetailData, ErrorResponseBody as ErrorResponseBodyData},
    http_client::exchanges::GetExchangeIdsQueryParams as GetExchangeIdsQueryParamsData,
    messages::{
        cancel::{Cancel as CancelData, CancelData as CancelDataData},
        close::{Close as CloseData, CloseData as CloseDataData},
        order::{Order as OrderData, OrderData as OrderDataData},
        order_instructions::{
            OrderInstructions as OrderInstructionsData,
            OrderInstructionsData as OrderInstructionsDataData,
            PaymentInstruction as PaymentInstructionData,
        },
        order_status::{
            OrderStatus as OrderStatusData, OrderStatusData as OrderStatusDataData,
            Status as OrderStatusStatus,
        },
        quote::{Quote as QuoteData, QuoteData as QuoteDataData, QuoteDetails as QuoteDetailsData},
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
