pub mod offering;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use type_safe_id::{DynamicType, TypeSafeId};

/// An enum representing all possible [`Resource`] kinds.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ResourceKind {
    Offering,
}

/// A struct representing the metadata present on every [`Resource`].
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMetadata {
    /// The resource's ID
    pub id: TypeSafeId<DynamicType>,
    /// This defines the data property's type (e.g. offering)
    pub kind: ResourceKind,
    /// The author's DID
    pub from: String,
    /// ISO 8601 timestamp
    pub created_at: DateTime<Utc>,
    /// ISO 8601 timestamp
    pub updated_at: DateTime<Utc>,
}

/// A struct representing the structure and common functionality available to all Resources.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource<T> {
    /// An object containing fields about the Resource
    pub metadata: ResourceMetadata,
    /// The actual Resource content
    pub data: T,
    /// The signature that verifies the authenticity and integrity of the Resource
    pub signature: Option<String>,
}

/// Errors that can occur when working with [`Resource`]s.
#[derive(thiserror::Error, Debug)]
pub enum ResourceError {
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    TypeSafeIdError(#[from] type_safe_id::Error),
}

impl ResourceKind {
    /// Returns the [`TypeSafeId`] of the [`ResourceKind`].
    pub fn typesafe_id(&self) -> Result<TypeSafeId<DynamicType>, ResourceError> {
        let serialized_kind = to_string(&self)?;
        let dynamic_type = DynamicType::new(serialized_kind.trim_matches('"'))?;
        Ok(TypeSafeId::new_with_type(dynamic_type))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resource_kind_typesafe_id() {
        let offering_id = ResourceKind::Offering.typesafe_id().unwrap();

        assert!(offering_id.to_string().starts_with("offering_"));

        assert_eq!(0, 1)
    }
}
