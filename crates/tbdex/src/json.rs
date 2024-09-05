use serde::{de::DeserializeOwned, Serialize};

use crate::errors::{Result, TbdexError};

pub trait FromJson: Sized + DeserializeOwned {
    fn from_json_string(json: &str) -> Result<Self> {
        serde_json::from_str(json).map_err(TbdexError::from)
    }
}

pub trait ToJson: Serialize {
    fn to_json_string(&self) -> Result<String> {
        serde_json::to_string(self).map_err(TbdexError::from)
    }
}
