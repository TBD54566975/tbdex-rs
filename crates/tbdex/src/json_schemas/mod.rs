pub mod generated;

use generated::DEFINITIONS_JSON_SCHEMA;
use jsonschema::{JSONSchema, SchemaResolver, SchemaResolverError};
use reqwest::blocking::get;
use serde::Serialize;
use serde_json::Error as SerdeJsonError;
use std::{collections::HashMap, sync::Arc};
use crate::json_schemas::generated::DRAFT_07_JSON_SCHEMA;

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
        schemas.insert(
            "http://json-schema.org/draft-07/schema#".to_string(),
            serde_json::from_str(&DRAFT_07_JSON_SCHEMA.replace("\\#", "#")).unwrap(),
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
    ) -> std::result::Result<std::sync::Arc<josekit::Value>, SchemaResolverError> {
        if let Some(schema) = self.schemas.get(url.as_str()) {
            Ok(std::sync::Arc::new(schema.clone()))
        } else {
            match get(url.as_str()) {
                Ok(response) => {
                    let schema: serde_json::Value = response.json().map_err(|err| {
                        SchemaResolverError::new(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            format!("failed to parse schema: {}", err),
                        ))
                    })?;
                    Ok(Arc::new(schema))
                }
                Err(err) => Err(SchemaResolverError::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("schema not found {}: {}", url, err),
                ))),
            }
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

#[cfg(test)]
mod json_schemas_test {
    use reqwest::Url;
    use super::*;
    use serde_json::json;

    #[test]
    fn test_validate_json_schema() {
        // Sample JSON data
        let sample_json_data = json!({
            "name": "John Doe",
            "age": 30,
            "email": "john.doe@example.com"
        });

        // Sample JSON schema
        let sample_json_schema = r#"
        {
            "$schema": "https://json-schema.org/draft-07/schema#",
            "type": "object",
            "properties": {
                "name": {
                    "type": "string"
                },
                "age": {
                    "type": "integer",
                    "minimum": 0
                },
                "email": {
                    "type": "string",
                    "format": "email"
                }
            },
            "required": ["name", "age", "email"]
        }
        "#;

        // Validate the sample JSON data against the schema
        let schema: serde_json::Value = serde_json::from_str(sample_json_schema).unwrap();
        let result = crate::json_schemas::validate(&schema, &sample_json_data);

        // Check that the validation was successful
        assert!(result.is_ok());
    }

    #[test]
    fn test_local_schema_resolver_local() {
        // Create a local schema resolver
        let resolver = LocalSchemaResolver::new();

        // Create a URL that matches one of the schemas in the resolver
        let url = Url::parse("https://tbdex.dev/definitions.json").unwrap();

        // Resolve the schema
        let resolved_schema = resolver.resolve(&json!({}), &url, "").unwrap();

        // Ensure the resolved schema is correct
        let expected_schema: serde_json::Value = serde_json::from_str(&DEFINITIONS_JSON_SCHEMA.replace("\\#", "#")).unwrap();
        assert_eq!(
            resolved_schema.as_ref(),
            &expected_schema
        );
    }

    #[test]
    fn test_local_draft_07_schema_resolver_local() {
        // Create a local schema resolver
        let resolver = LocalSchemaResolver::new();

        // Create a URL that matches one of the schemas in the resolver
        let url = Url::parse("http://json-schema.org/draft-07/schema#").unwrap();

        // Resolve the schema
        let resolved_schema = resolver.resolve(&json!({}), &url, "").unwrap();

        // Ensure the resolved schema is correct
        let expected_schema: serde_json::Value = serde_json::from_str(&DRAFT_07_JSON_SCHEMA.replace("\\#", "#")).unwrap();
        assert_eq!(
            resolved_schema.as_ref(),
            &expected_schema
        );
    }
}