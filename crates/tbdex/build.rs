use reqwest;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

fn write_json_schemas() -> Result<(), Box<dyn std::error::Error>> {
    let commit_hash = "96a1a7164e8e0a608befa31b6cf0c9a4e5cc0f07";
    let schemas = vec![
      ("RESOURCE_JSON_SCHEMA", format!("https://raw.githubusercontent.com/TBD54566975/tbdex/{}/hosted/json-schemas/resource.schema.json", commit_hash)),
      ("BALANCE_DATA_JSON_SCHEMA", format!("https://raw.githubusercontent.com/TBD54566975/tbdex/{}/hosted/json-schemas/balance.schema.json", commit_hash)),
      ("OFFERING_DATA_JSON_SCHEMA", format!("https://raw.githubusercontent.com/TBD54566975/tbdex/{}/hosted/json-schemas/offering.schema.json", commit_hash)),
      ("MESSAGE_JSON_SCHEMA", format!("https://raw.githubusercontent.com/TBD54566975/tbdex/{}/hosted/json-schemas/message.schema.json", commit_hash)),
      ("RFQ_DATA_JSON_SCHEMA", format!("https://raw.githubusercontent.com/TBD54566975/tbdex/{}/hosted/json-schemas/rfq.schema.json", commit_hash)),
      ("RFQ_PRIVATE_DATA_JSON_SCHEMA", format!("https://raw.githubusercontent.com/TBD54566975/tbdex/{}/hosted/json-schemas/rfq-private.schema.json", commit_hash)),
      ("QUOTE_DATA_JSON_SCHEMA", format!("https://raw.githubusercontent.com/TBD54566975/tbdex/{}/hosted/json-schemas/quote.schema.json", commit_hash)),
      ("ORDER_DATA_JSON_SCHEMA", format!("https://raw.githubusercontent.com/TBD54566975/tbdex/{}/hosted/json-schemas/order.schema.json", commit_hash)),
      ("ORDER_DATA_STATUS_JSON_SCHEMA", format!("https://raw.githubusercontent.com/TBD54566975/tbdex/{}/hosted/json-schemas/orderstatus.schema.json", commit_hash)),
      ("CLOSE_DATA_JSON_SCHEMA", format!("https://raw.githubusercontent.com/TBD54566975/tbdex/{}/hosted/json-schemas/close.schema.json", commit_hash)),
  ];

    let dest_path = Path::new("src").join("json_schemas.rs");

    // Check if the existing file contains the same commit hash
    if let Ok(file) = fs::File::open(&dest_path) {
        let reader = io::BufReader::new(file);
        if let Some(Ok(first_line)) = reader.lines().next() {
            if first_line == format!("pub const GIT_COMMIT_HASH: &str = \"{}\";", commit_hash) {
                // The commit hash matches, no need to re-fetch the schemas
                println!("Schemas are up-to-date, skipping download.");
                return Ok(());
            }
        }
    }

    let mut file_content = String::new();

    // Add the commit hash to the top of the file
    file_content.push_str(&format!(
        "pub const GIT_COMMIT_HASH: &str = \"{}\";\n",
        commit_hash
    ));

    for (name, url) in schemas {
        let response = reqwest::blocking::get(&url)?;
        let mut schema = response.text()?;
        schema = schema.replace("#", "\\#");
        file_content.push_str(&format!("pub const {}: &str = r#\"{}\"#;\n", name, schema));
    }

    fs::write(dest_path, file_content)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    write_json_schemas()?;

    Ok(())
}
