mod http_client;
mod messages;
mod resources;

mod errors;

use crate::{
    errors::RustCoreError,
    http_client::{
        create_exchange, get_balances, get_exchange, get_exchanges, get_offerings, submit_close,
        submit_order, Exchange as ExchangeData,
    },
    messages::{
        close::Close,
        order::Order,
        order_status::OrderStatus,
        quote::Quote,
        rfq::{
            data::{
                CreateRfqData as CreateRfqDataData,
                CreateSelectedPayinMethod as CreateSelectedPayinMethodData,
                CreateSelectedPayoutMethod as CreateSelectedPayoutMethodData,
                PrivatePaymentDetails as PrivatePaymentDetailsData, Rfq as RfqData,
                RfqData as RfqDataData, RfqPrivateData as RfqPrivateDataData,
                SelectedPayinMethod as SelectedPayinMethodData,
                SelectedPayoutMethod as SelectedPayoutMethodData,
            },
            Rfq,
        },
    },
    resources::{
        balance::Balance,
        offering::{
            data::{
                Offering as OfferingData, OfferingData as OfferingDataData,
                PayinDetails as PayinDetailsData, PayinMethod as PayinMethodData,
                PayoutDetails as PayoutDetailsData, PayoutMethod as PayoutMethodData,
            },
            Offering,
        },
    },
};
use tbdex::{
    messages::{
        close::{Close as CloseData, CloseData as CloseDataData},
        order::Order as OrderData,
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
use web5::apid::{
    credentials::presentation_definition::{
        Constraints as ConstraintsData, Field as FieldData, Filter as FilterData,
        InputDescriptor as InputDescriptorData, Optionality,
        PresentationDefinition as PresentationDefinitionData,
    },
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
    crypto::key_manager::KeyManager,
    dids::bearer_did::{BearerDid, BearerDidData},
    dsa::Signer,
    errors::RustCoreError as Web5RustCoreError,
};

pub fn hello_world() {
    println!("Hello world")
}

uniffi::include_scaffolding!("tbdex");
