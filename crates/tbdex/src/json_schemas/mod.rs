pub mod generated;

use generated::DEFINITIONS_JSON_SCHEMA;
use jsonschema::{JSONSchema, SchemaResolver};
use serde::Serialize;
use serde_json::Error as SerdeJsonError;
use std::collections::HashMap;

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum JsonSchemaError {
    #[error("serde json error {0}")]
    SerdeJson(String),
    #[error("json schema failure {0:?}")]
    JsonSchema(Vec<String>),
    #[error("unsupported json schema version {0}")]
    UnsupportedVersion(String),
}

impl From<SerdeJsonError> for JsonSchemaError {
    fn from(err: SerdeJsonError) -> Self {
        JsonSchemaError::SerdeJson(err.to_string())
    }
}

type Result<T> = std::result::Result<T, JsonSchemaError>;

struct LocalSchemaResolver {
    schemas: HashMap<String, serde_json::Value>,
}

impl LocalSchemaResolver {
    fn new() -> Self {
        let mut schemas = HashMap::new();
        schemas.insert(
            "https://tbdex.dev/definitions.json".to_string(),
            serde_json::from_str(&DEFINITIONS_JSON_SCHEMA.replace("\\#", "#")).unwrap(),
        );
        LocalSchemaResolver { schemas }
    }
}

impl SchemaResolver for LocalSchemaResolver {
    fn resolve(
        &self,
        _root_schema: &josekit::Value,
        url: &reqwest::Url,
        _original_reference: &str,
    ) -> std::result::Result<std::sync::Arc<josekit::Value>, jsonschema::SchemaResolverError> {
        if let Some(schema) = self.schemas.get(url.as_str()) {
            Ok(std::sync::Arc::new(schema.clone()))
        } else {
            Err(jsonschema::SchemaResolverError::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("schema not found {}", url),
            )))
        }
    }
}

pub fn validate_from_str<T: Serialize>(schema_str: &str, value: &T) -> Result<()> {
    let schema = &serde_json::from_str::<serde_json::Value>(&schema_str.replace("\\#", "#"))?;

    validate(schema, value)?;

    Ok(())
}

pub fn validate<T: Serialize>(schema: &serde_json::Value, value: &T) -> Result<()> {
    if let Some(serde_json::Value::String(url)) = schema.get("$schema") {
        if url.contains("draft-04") || url.contains("draft-06") {
            return Err(JsonSchemaError::UnsupportedVersion(url.to_string()));
        }
    }

    let compiled = JSONSchema::options()
        .with_resolver(LocalSchemaResolver::new())
        .compile(schema)
        .map_err(|e| JsonSchemaError::JsonSchema(vec![e.to_string()]))?;

    let instance = serde_json::to_value(value)?;
    let result = compiled.validate(&instance);

    if let Err(errors) = result {
        let error_messages: Vec<String> = errors
            .map(|e| format!("{} at {}", e, e.instance_path))
            .collect();
        return Err(JsonSchemaError::JsonSchema(error_messages));
    }

    Ok(())
}
