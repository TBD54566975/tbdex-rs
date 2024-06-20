pub mod resource_metadata;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum ResourceKind {
    Offering,
    Balance,
}
