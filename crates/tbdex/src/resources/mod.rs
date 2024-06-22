pub mod balance;
pub mod offering;

use crate::signature::SignatureError;
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeJsonError;
use type_safe_id::{DynamicType, Error as TypeIdError, TypeSafeId};
use web5::apid::dids::bearer_did::{BearerDid, BearerDidError};

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum ResourceError {
    #[error("serde json error {0}")]
    SerdeJsonError(String),
    #[error("typeid error {0}")]
    TypeIdError(String),
    #[error(transparent)]
    BearerDidError(#[from] BearerDidError),
    #[error(transparent)]
    SignatureError(#[from] SignatureError),
}

impl From<SerdeJsonError> for ResourceError {
    fn from(err: SerdeJsonError) -> Self {
        ResourceError::SerdeJsonError(err.to_string())
    }
}

impl From<TypeIdError> for ResourceError {
    fn from(err: TypeIdError) -> Self {
        ResourceError::TypeIdError(err.to_string())
    }
}

type Result<T> = std::result::Result<T, ResourceError>;

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ResourceKind {
    Offering,
    Balance,
}

impl ResourceKind {
    pub fn typesafe_id(&self) -> Result<String> {
        let serialized_kind = serde_json::to_string(&self)?;
        let dynamic_type = DynamicType::new(serialized_kind.trim_matches('"'))?;
        Ok(TypeSafeId::new_with_type(dynamic_type).to_string())
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMetadata {
    pub kind: ResourceKind,
    pub from: String,
    pub id: String,
    pub protocol: String,
    pub created_at: String,
    pub updated_at: Option<String>,
}

pub trait Resource: Send + Sync {
    fn sign(&mut self, bearer_did: BearerDid) -> Result<()>;
    fn verify(&self) -> Result<()>;
}
