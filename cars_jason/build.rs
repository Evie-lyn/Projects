use std::env;
use std::fs;
use std::path::Path;
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json_path = Path::new("test.json");
    let out_dir_str = env::var("OUT_DIR")?; 
    let out_dir = Path::new(&out_dir_str).join("generated.rs");

    println!("cargo:rerun-if-changed={}", json_path.display());

    let json_content = fs::read_to_string(json_path)?;
    let parsed: Value = serde_json::from_str(&json_content)?;

    let types_array = parsed["types"].as_array().ok_or("Expected 'types' array in JSON")?;

    let mut output = String::new();

    for type_def in types_array {
        let name = type_def["name"].as_str().ok_or("Expected 'name' to be a string")?;
        let fields_obj = type_def["fields"].as_object().ok_or("Expected 'fields' to be an object")?;

        output.push_str("#[derive(Debug)]\n");
        output.push_str(&format!("pub struct {} {{\n", name));

        for (field_name, field_type_value) in fields_obj.iter() {
            let field_type = field_type_value.as_str().ok_or(format!("Expected type for '{}' to be a string", field_name))?;
            output.push_str(&format!("    pub {}: {},\n", field_name, field_type));
        }

        output.push_str("}\n\n");
    }

    fs::write(&out_dir, output)?;

    Ok(())
}