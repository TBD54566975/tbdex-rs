mod messages;
mod resources;

mod errors;

use crate::{
    errors::RustCoreError,
    messages::{quote::Quote, rfq::Rfq, Message},
    resources::{balance::Balance, offering::Offering, Resource},
};
use tbdex::{
    messages::{
        quote::{
            PaymentInstructions as PaymentInstructionsData, Quote as QuoteData,
            QuoteData as QuoteDataData, QuoteDetails as QuoteDetailsData,
        },
        rfq::{
            CreateRfqData as CreateRfqDataData,
            CreateSelectedPayinMethod as CreateSelectedPayinMethodData,
            CreateSelectedPayoutMethod as CreateSelectedPayoutMethodData,
            PrivatePaymentDetails as PrivatePaymentDetailsData, Rfq as RfqData,
            RfqData as RfqDataData, RfqPrivateData as RfqPrivateDataData,
            SelectedPayinMethod as SelectedPayinMethodData,
            SelectedPayoutMethod as SelectedPayoutMethodData,
        },
        MessageKind, MessageMetadata as MessageMetadataData,
    },
    resources::{
        balance::{Balance as BalanceData, BalanceData as BalanceDataData},
        offering::{
            Offering as OfferingData, OfferingData as OfferingDataData,
            PayinDetails as PayinDetailsData, PayinMethod as PayinMethodData,
            PayoutDetails as PayoutDetailsData, PayoutMethod as PayoutMethodData,
        },
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
