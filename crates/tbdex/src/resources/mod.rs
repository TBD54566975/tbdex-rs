pub mod balance;
pub mod offering;

use std::{fmt, str::FromStr};

use crate::errors::{Result, TbdexError};
use serde::{Deserialize, Serialize};
use type_safe_id::{DynamicType, TypeSafeId};

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}
