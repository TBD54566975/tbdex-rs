use super::{MessageKind, MessageMetadata, Result};
use crate::json_schemas::generated::{CANCEL_DATA_JSON_SCHEMA, MESSAGE_JSON_SCHEMA};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use web5::dids::bearer_did::BearerDid;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Cancel {
    pub metadata: MessageMetadata,
    pub data: CancelData,
    pub signature: String,
}

impl Cancel {
    pub fn new(
        bearer_did: &BearerDid,
        to: &str,
        from: &str,
        exchange_id: &str,
        data: &CancelData,
        protocol: &str,
        external_id: Option<String>,
    ) -> Result<Self> {
        let metadata = MessageMetadata {
            from: from.to_string(),
            to: to.to_string(),
            kind: MessageKind::Cancel,
            id: MessageKind::Cancel.typesafe_id()?,
            exchange_id: exchange_id.to_string(),
            external_id,
            protocol: protocol.to_string(),
            created_at: Utc::now().to_rfc3339(),
        };

        let cancel = Self {
            metadata: metadata.clone(),
            data: data.clone(),
            signature: crate::signature::sign(
                bearer_did,
                &serde_json::to_value(metadata)?,
                &serde_json::to_value(data)?,
            )?,
        };

        Ok(cancel)
    }

    pub fn from_json_string(json: &str) -> Result<Self> {
        let cancel = serde_json::from_str::<Self>(json)?;
        Ok(cancel)
    }

    pub fn verify(&self) -> Result<()> {
        // verify resource json schema
        crate::json_schemas::validate_from_str(MESSAGE_JSON_SCHEMA, self)?;

        // verify data json schema
        crate::json_schemas::validate_from_str(CANCEL_DATA_JSON_SCHEMA, &self.data)?;

        // verify signature
        crate::signature::verify(
            &self.metadata.from,
            &serde_json::to_value(self.metadata.clone())?,
            &serde_json::to_value(self.data.clone())?,
            &self.signature,
        )?;

        Ok(())
    }

    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self)?)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct CancelData {
    pub reason: String,
}

#[cfg(test)]
mod tbdex_test_vectors_protocol {
    use super::*;
    use std::fs;

    #[derive(Debug, serde::Deserialize)]
    pub struct TestVector {
        pub input: String,
        pub output: Cancel,
    }

    #[test]
    fn parse_cancel() {
        let path = "../../tbdex/hosted/test-vectors/protocol/vectors/parse-cancel.json";
        let test_vector_json: String = fs::read_to_string(path).unwrap();

        let test_vector: TestVector = serde_json::from_str(&test_vector_json).unwrap();
        let parsed_cancel: Cancel = Cancel::from_json_string(&test_vector.input).unwrap();

        assert!(parsed_cancel.verify().is_ok(), "Verification failed");
        assert_eq!(test_vector.output, parsed_cancel);
    }
}
