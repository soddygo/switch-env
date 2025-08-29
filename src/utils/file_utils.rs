use std::collections::HashMap;
use std::fs;
use serde_json;

#[derive(Debug, Clone)]
pub enum ImportFormat {
    Json,
    Env,
    Yaml,
}

/// Read environment variables from a file
/// Supports formats:
/// - KEY=VALUE (one per line)
/// - .env format
/// - JSON format
pub fn read_env_file(file_path: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;
    
    let mut variables = HashMap::new();
    
    // Try to parse as JSON first
    if file_path.ends_with(".json") {
        let json_vars: HashMap<String, serde_json::Value> = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse JSON file '{}': {}", file_path, e))?;
        
        for (key, value) in json_vars {
            let string_value = match value {
                serde_json::Value::String(s) => s,
                serde_json::Value::Number(n) => n.to_string(),
                serde_json::Value::Bool(b) => b.to_string(),
                _ => value.to_string(),
            };
            variables.insert(key, string_value);
        }
    } else {
        // Parse as .env format (KEY=VALUE lines)
        for (line_num, line) in content.lines().enumerate() {
            let line = line.trim();
            
            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            // Parse KEY=VALUE format
            if let Some(eq_pos) = line.find('=') {
                let key = line[..eq_pos].trim().to_string();
                let value = line[eq_pos + 1..].trim();
                
                // Remove quotes if present
                let value = if (value.starts_with('"') && value.ends_with('"')) ||
                              (value.starts_with('\'') && value.ends_with('\'')) {
                    &value[1..value.len()-1]
                } else {
                    value
                };
                
                if key.is_empty() {
                    return Err(format!("Empty variable name on line {} in file '{}'", line_num + 1, file_path).into());
                }
                
                variables.insert(key, value.to_string());
            } else {
                return Err(format!("Invalid format on line {} in file '{}': expected KEY=VALUE", line_num + 1, file_path).into());
            }
        }
    }
    
    Ok(variables)
}

/// Detect the format of an import file based on its extension and content
pub fn detect_import_format(path: &std::path::Path) -> Result<ImportFormat, Box<dyn std::error::Error>> {
    let extension = path.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    match extension.as_str() {
        "json" => Ok(ImportFormat::Json),
        "env" => Ok(ImportFormat::Env),
        "yaml" | "yml" => Ok(ImportFormat::Yaml),
        _ => {
            // Try to detect based on content
            let content = std::fs::read_to_string(path)?;
            let content_sample = content.lines().take(10).collect::<Vec<_>>().join("\n");
            
            if content_sample.contains('{') && content_sample.contains('}') {
                Ok(ImportFormat::Json)
            } else if content_sample.contains('=') {
                Ok(ImportFormat::Env)
            } else if content_sample.contains(':') && content_sample.contains('-') {
                Ok(ImportFormat::Yaml)
            } else {
                Err("Unable to detect file format. Please specify the format explicitly or use a standard file extension (.json, .env, .yaml)".into())
            }
        }
    }
}