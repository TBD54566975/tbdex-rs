mod http_client;
mod messages;
mod resources;

mod errors;

use crate::{
    errors::RustCoreError,
    http_client::{
        balances::get_balances,
        exchanges::{
            create_exchange, get_exchange, get_exchanges, submit_close, submit_order,
            Exchange as ExchangeData,
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
            PaymentInstructions as PaymentInstructionsData, Quote as QuoteData,
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
    },
};
use web5_uniffi_wrapper::{
    credentials::presentation_definition::PresentationDefinition,
    crypto::{dsa::Signer, in_memory_key_manager::InMemoryKeyManager, key_manager::KeyManager},
    dids::bearer_did::{BearerDid, BearerDidData},
    errors::RustCoreError as Web5RustCoreError,
};

// 🚧 TODO temporary hack in place while did:dht resolution is incomplete
pub fn tmp_hack_bearer_did(
    did: DidData,
    document: DocumentData,
    key_manager: std::sync::Arc<dyn KeyManager>,
) -> std::sync::Arc<BearerDid> {
    std::sync::Arc::new(BearerDid(web5::dids::bearer_did::BearerDid {
        did,
        document,
        key_manager: key_manager.to_inner(),
    }))
}

uniffi::include_scaffolding!("tbdex");
