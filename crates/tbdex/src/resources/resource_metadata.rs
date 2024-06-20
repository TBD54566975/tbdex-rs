use super::ResourceKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceMetadata {
    pub kind: ResourceKind,
    pub from: String,
    pub to: String,
    pub id: String,
    pub protocol: String,
    pub created_at: String,
    pub updated_at: Option<String>,
}
