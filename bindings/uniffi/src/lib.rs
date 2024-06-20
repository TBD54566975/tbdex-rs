use tbdex::resources::{resource_metadata::ResourceMetadata as ResourceMetadataData, ResourceKind};
use web5::apid::{
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
    crypto::key_manager::KeyManager,
    dids::bearer_did::{BearerDid, BearerDidData},
    dsa::Signer,
    errors::RustCoreError as Web5RustCoreError,
};

pub fn hello_world() {
    println!("Hello world")
}

uniffi::include_scaffolding!("tbdex");
