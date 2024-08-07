use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

fn write_json_schemas() -> Result<(), Box<dyn std::error::Error>> {
    let commit_hash = "7d2fdd03c9405b920b056ab7c7c776a858dc3591";
    let schemas = vec![
        // Tbdex Schemas
      ("DEFINITIONS_JSON_SCHEMA", format!("https://raw.githubusercontent.com/TBD54566975/tbdex/{}/hosted/json-schemas/definitions.json", commit_hash)),
      ("RESOURCE_JSON_SCHEMA", format!("https://raw.githubusercontent.com/TBD54566975/tbdex/{}/hosted/json-schemas/resource.schema.json", commit_hash)),
      ("BALANCE_DATA_JSON_SCHEMA", format!("https://raw.githubusercontent.com/TBD54566975/tbdex/{}/hosted/json-schemas/balance.schema.json", commit_hash)),
      ("OFFERING_DATA_JSON_SCHEMA", format!("https://raw.githubusercontent.com/TBD54566975/tbdex/{}/hosted/json-schemas/offering.schema.json", commit_hash)),
      ("MESSAGE_JSON_SCHEMA", format!("https://raw.githubusercontent.com/TBD54566975/tbdex/{}/hosted/json-schemas/message.schema.json", commit_hash)),
      ("RFQ_DATA_JSON_SCHEMA", format!("https://raw.githubusercontent.com/TBD54566975/tbdex/{}/hosted/json-schemas/rfq.schema.json", commit_hash)),
      ("RFQ_PRIVATE_DATA_JSON_SCHEMA", format!("https://raw.githubusercontent.com/TBD54566975/tbdex/{}/hosted/json-schemas/rfq-private.schema.json", commit_hash)),
      ("QUOTE_DATA_JSON_SCHEMA", format!("https://raw.githubusercontent.com/TBD54566975/tbdex/{}/hosted/json-schemas/quote.schema.json", commit_hash)),
      ("ORDER_DATA_JSON_SCHEMA", format!("https://raw.githubusercontent.com/TBD54566975/tbdex/{}/hosted/json-schemas/order.schema.json", commit_hash)),
      ("ORDER_INSTRUCTIONS_DATA_JSON_SCHEMA", format!("https://raw.githubusercontent.com/TBD54566975/tbdex/{}/hosted/json-schemas/orderinstructions.schema.json", commit_hash)),
      ("CANCEL_DATA_JSON_SCHEMA", format!("https://raw.githubusercontent.com/TBD54566975/tbdex/{}/hosted/json-schemas/cancel.schema.json", commit_hash)),
      ("ORDER_STATUS_DATA_JSON_SCHEMA", format!("https://raw.githubusercontent.com/TBD54566975/tbdex/{}/hosted/json-schemas/orderstatus.schema.json", commit_hash)),
      ("CLOSE_DATA_JSON_SCHEMA", format!("https://raw.githubusercontent.com/TBD54566975/tbdex/{}/hosted/json-schemas/close.schema.json", commit_hash)),

        // Json schema
      ("DRAFT_07_JSON_SCHEMA", "https://json-schema.org/draft-07/schema".to_string()),
    ];

    let dest_path = Path::new("src/json_schemas").join("generated.rs");

    // Check if the existing file contains the same commit hash
    if let Ok(file) = fs::File::open(&dest_path) {
        let reader = io::BufReader::new(file);
        let mut lines = reader.lines();
        let third_line = lines.nth(3); // Get the third line (index 2)

        if let Some(Ok(line)) = third_line {
            if line == format!("pub const GIT_COMMIT_HASH: &str = \"{}\";", commit_hash) {
                // The commit hash matches, no need to re-fetch the schemas
                println!("Schemas are up-to-date, skipping download.");
                return Ok(());
            }
        }
    }

    let mut file_content = String::new();

    // Add the commit hash to the top of the file
    file_content.push_str("// THIS FILE IS AUTO-GENERATED BY build.rs\n");
    file_content.push_str("#[warn(unused_imports)]\n");
    file_content.push_str("#[allow(dead_code)]\n");
    file_content.push_str(&format!(
        "pub const GIT_COMMIT_HASH: &str = \"{}\";\n",
        commit_hash
    ));

    for (name, url) in schemas {
        let response = reqwest::blocking::get(&url)?;
        let mut schema = response.text()?;
        schema = schema.replace('#', "\\#");
        file_content.push_str(&format!("pub const {}: &str = r#\"{}\"#;\n", name, schema));
    }

    fs::write(dest_path, file_content)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    write_json_schemas()?;

    Ok(())
}
