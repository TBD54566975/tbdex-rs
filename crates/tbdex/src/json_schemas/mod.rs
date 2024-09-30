pub mod generated;

use crate::{
    errors::{Result, TbdexError},
    http_client::get_json,
    json_schemas::generated::DRAFT_07_JSON_SCHEMA,
};
use futures::executor::block_on;
use generated::DEFINITIONS_JSON_SCHEMA;
use jsonschema::{JSONSchema, SchemaResolver, SchemaResolverError};
use serde::Serialize;
use std::{collections::HashMap, sync::Arc};

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
            "https://json-schema.org/draft-07/schema".to_string(),
            serde_json::from_str(&DRAFT_07_JSON_SCHEMA.replace("\\#", "#")).unwrap(),
        );
        LocalSchemaResolver { schemas }
    }

    fn normalize_url(url: &url::Url) -> String {
        let mut normalized_url = url.clone();
        normalized_url.set_fragment(None);
        let mut url_str = normalized_url.to_string();
        if url_str.starts_with("http://") {
            url_str = url_str.replacen("http://", "https://", 1);
        }
        url_str
    }
}

impl SchemaResolver for LocalSchemaResolver {
    fn resolve(
        &self,
        _root_schema: &serde_json::Value,
        url: &url::Url,
        _original_reference: &str,
    ) -> std::result::Result<std::sync::Arc<serde_json::Value>, SchemaResolverError> {
        if let Some(schema) = self.schemas.get(&LocalSchemaResolver::normalize_url(url)) {
            Ok(std::sync::Arc::new(schema.clone()))
        } else {
            match block_on(get_json::<serde_json::Value>(url.as_str(), None)) {
                Ok(schema) => Ok(Arc::new(schema)),
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
            return Err(TbdexError::JsonSchema(format!(
                "unsupported version {}",
                url
            )));
        }
    }

    let compiled = JSONSchema::options()
        .with_resolver(LocalSchemaResolver::new())
        .compile(schema)
        .map_err(|e| TbdexError::JsonSchema(e.to_string()))?;

    let instance = serde_json::to_value(value)?;
    let result = compiled.validate(&instance);

    if let Err(errors) = result {
        let error_messages = errors
            .map(|e| format!("{} at {}", e, e.instance_path))
            .collect::<Vec<String>>()
            .join(", ");
        return Err(TbdexError::JsonSchema(error_messages));
    }

    Ok(())
}

#[cfg(test)]
mod json_schemas_test {
    use super::*;
    use serde_json::json;
    use url::Url;

    #[test]
    fn test_validate_json_schema() {
        let data = json!({
            "name": "John Doe",
            "age": 30,
            "email": "john.doe@example.com"
        });

        let schema = json!({
            "$schema": "https://json-schema.org/draft-07/schema#",
            "type": "object",
            "properties": {
                "name": { "type": "string" },
                "age": { "type": "integer", "minimum": 0 },
                "email": { "type": "string", "format": "email" }
            },
            "required": ["name", "age", "email"]
        });

        assert!(crate::json_schemas::validate(&schema, &data).is_ok());
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
        let expected_schema: serde_json::Value =
            serde_json::from_str(&DEFINITIONS_JSON_SCHEMA.replace("\\#", "#")).unwrap();
        assert_eq!(resolved_schema.as_ref(), &expected_schema);
    }

    #[test]
    fn test_local_draft_07_schema_resolver_local() {
        // Create a local schema resolver
        let resolver = LocalSchemaResolver::new();

        // Create a URL that matches one of the schemas in the resolver
        let url = Url::parse("http://json-schema.org/draft-07/schema").unwrap();

        // Resolve the schema
        let resolved_schema = resolver.resolve(&json!({}), &url, "").unwrap();

        // Ensure the resolved schema is correct
        let expected_schema: serde_json::Value =
            serde_json::from_str(&DRAFT_07_JSON_SCHEMA.replace("\\#", "#")).unwrap();
        assert_eq!(resolved_schema.as_ref(), &expected_schema);
    }

    #[test]
    fn test_local_draft_07_schema_resolver_local_with_fragment() {
        // Create a local schema resolver
        let resolver = LocalSchemaResolver::new();

        // Create a URL with the fragment (#)
        let url = Url::parse("http://json-schema.org/draft-07/schema#").unwrap();

        // Resolve the schema
        let resolved_schema = resolver.resolve(&json!({}), &url, "").unwrap();

        // Ensure the resolved schema is correct
        let expected_schema: serde_json::Value =
            serde_json::from_str(&DRAFT_07_JSON_SCHEMA.replace("\\#", "#")).unwrap();
        assert_eq!(resolved_schema.as_ref(), &expected_schema);
    }
}
