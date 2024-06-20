mod resources;

mod errors;

use crate::{
    errors::RustCoreError,
    resources::{balance::Balance, offering::Offering, Resource},
};
use tbdex::resources::{
    balance::{Balance as BalanceData, BalanceData as BalanceDataData},
    offering::{
        Offering as OfferingData, OfferingData as OfferingDataData,
        PayinDetails as PayinDetailsData, PayinMethod as PayinMethodData,
        PayoutDetails as PayoutDetailsData, PayoutMethod as PayoutMethodData,
    },
    resource_metadata::ResourceMetadata as ResourceMetadataData,
    ResourceKind,
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
