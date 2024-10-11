pub mod balance;
pub mod offering;

use std::{fmt, str::FromStr};
use std::any::type_name;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use chrono::Utc;
use lazy_static::lazy_static;
use crate::errors::{Result, TbdexError};
use serde::{Deserialize, Serialize};
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

lazy_static! {
    static ref COUNTER: AtomicU64 = AtomicU64::new(0);
    static ref LAST_TIMESTAMP: Mutex<i64> = Mutex::new(0);
}

impl ResourceKind {
    pub fn typesafe_id(&self) -> Result<String> {
        let class_name = type_name::<Self>();
        let timestamp = Utc::now().timestamp_nanos_opt().unwrap();

        let mut last_timestamp = LAST_TIMESTAMP.lock().unwrap();

        let counter_value = if timestamp > *last_timestamp {
            // New timestamp, reset counter
            *last_timestamp = timestamp;
            COUNTER.store(0, Ordering::SeqCst);
            0
        } else {
            // Same or earlier timestamp, increment counter
            COUNTER.fetch_add(1, Ordering::SeqCst)
        };

        let uuid_v4 = Uuid::new_v4();

        // Create an ID in the format: classname_timestamp_counter_uuid
        let id = format!("{}_{}_{}_{}", class_name, timestamp, counter_value, uuid_v4);

        Ok(id)
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
