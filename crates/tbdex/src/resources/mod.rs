pub mod balance;
pub mod offering;

use std::{fmt, str::FromStr};

use crate::errors::{Result, TbdexError};
use serde::{Deserialize, Serialize};
use type_safe_id::{DynamicType, TypeSafeId};
use uuid::Uuid;

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ResourceKind {
    Offering,
    Balance,
}

impl FromStr for ResourceKind {
    type Err = TbdexError;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "offering" => Ok(ResourceKind::Offering),
            "balance" => Ok(ResourceKind::Balance),
            _ => Err(TbdexError::Parse(format!("invalid resource kind {}", s))),
        }
    }
}

impl fmt::Display for ResourceKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ResourceKind::Offering => write!(f, "offering"),
            ResourceKind::Balance => write!(f, "balance"),
        }
    }
}

impl ResourceKind {
    pub fn typesafe_id(&self) -> Result<String> {
        let dynamic_type = DynamicType::new(&self.to_string())?;
        Ok(TypeSafeId::from_type_and_uuid(dynamic_type, Uuid::new_v4()).to_string())
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}
