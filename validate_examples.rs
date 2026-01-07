use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse TOML
    let toml_path = Path::new("examples/kreuzberg.toml");
    let toml_content = std::fs::read_to_string(toml_path)?;
    let _toml_config: toml::Value = toml::from_str(&toml_content)?;
    println!("✓ TOML file is valid");

    // Parse YAML
    let yaml_path = Path::new("examples/kreuzberg.yaml");
    let yaml_content = std::fs::read_to_string(yaml_path)?;
    let _yaml_config: serde_yaml_ng::Value = serde_yaml_ng::from_str(&yaml_content)?;
    println!("✓ YAML file is valid");

    // Parse JSON
    let json_path = Path::new("examples/kreuzberg.json");
    let json_content = std::fs::read_to_string(json_path)?;
    let _json_config: serde_json::Value = serde_json::from_str(&json_content)?;
    println!("✓ JSON file is valid");

    println!("\nAll configuration files parsed successfully!");
    Ok(())
}
