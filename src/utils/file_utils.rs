use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde_json;

#[derive(Debug, Clone, PartialEq)]
pub enum FileFormat {
    Json,
    Env,
    Yaml,
}

#[derive(Debug)]
pub struct FormatValidationResult {
    pub is_valid: bool,
    pub format: Option<FileFormat>,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
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

/// Detect file format based on extension and content analysis
pub fn detect_file_format(path: &Path) -> Result<FileFormat, Box<dyn std::error::Error>> {
    // First try extension-based detection
    let extension = path.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    let format_from_extension = match extension.as_str() {
        "json" => Some(FileFormat::Json),
        "env" => Some(FileFormat::Env),
        "yaml" | "yml" => Some(FileFormat::Yaml),
        _ => None,
    };
    
    // If extension is clear, validate content matches
    if let Some(expected_format) = format_from_extension {
        let validation = validate_file_format(path, &expected_format)?;
        if validation.is_valid {
            return Ok(expected_format);
        }
    }
    
    // Try content-based detection
    let content = fs::read_to_string(path)?;
    let trimmed = content.trim();
    
    if trimmed.is_empty() {
        return Err("File is empty".into());
    }
    
    // JSON detection
    if (trimmed.starts_with('{') && trimmed.ends_with('}')) ||
       (trimmed.starts_with('[') && trimmed.ends_with(']')) {
        let validation = validate_file_format(path, &FileFormat::Json)?;
        if validation.is_valid {
            return Ok(FileFormat::Json);
        }
    }
    
    // ENV detection - look for KEY=VALUE patterns
    let has_env_pattern = trimmed.lines()
        .filter(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))
        .any(|line| line.contains('='));
    
    if has_env_pattern {
        let validation = validate_file_format(path, &FileFormat::Env)?;
        if validation.is_valid {
            return Ok(FileFormat::Env);
        }
    }
    
    // YAML detection - look for key: value patterns
    let has_yaml_pattern = trimmed.lines()
        .filter(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))
        .any(|line| line.contains(':') && !line.contains('='));
    
    if has_yaml_pattern {
        return Ok(FileFormat::Yaml);
    }
    
    Err("Unable to detect file format. Supported formats: JSON (.json), ENV (.env), YAML (.yaml/.yml)".into())
}

/// Validate that a file matches the expected format
pub fn validate_file_format(path: &Path, expected_format: &FileFormat) -> Result<FormatValidationResult, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let mut result = FormatValidationResult {
        is_valid: false,
        format: None,
        errors: Vec::new(),
        warnings: Vec::new(),
    };
    
    match expected_format {
        FileFormat::Json => {
            match serde_json::from_str::<serde_json::Value>(&content) {
                Ok(_) => {
                    result.is_valid = true;
                    result.format = Some(FileFormat::Json);
                }
                Err(e) => {
                    result.errors.push(format!("Invalid JSON format: {}", e));
                }
            }
        }
        
        FileFormat::Env => {
            let mut line_num = 0;
            let mut has_valid_entries = false;
            
            for line in content.lines() {
                line_num += 1;
                let line = line.trim();
                
                // Skip empty lines and comments
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                
                // Check for KEY=VALUE format
                if let Some(eq_pos) = line.find('=') {
                    let key = line[..eq_pos].trim();
                    let value = line[eq_pos + 1..].trim();
                    
                    if key.is_empty() {
                        result.errors.push(format!("Line {}: Empty variable name", line_num));
                    } else if !key.chars().all(|c| c.is_alphanumeric() || c == '_') {
                        result.warnings.push(format!("Line {}: Variable name '{}' contains non-standard characters", line_num, key));
                    }
                    
                    // Check for unquoted values with spaces
                    if value.contains(' ') && !((value.starts_with('"') && value.ends_with('"')) ||
                                               (value.starts_with('\'') && value.ends_with('\''))) {
                        result.warnings.push(format!("Line {}: Value contains spaces but is not quoted", line_num));
                    }
                    
                    has_valid_entries = true;
                } else {
                    result.errors.push(format!("Line {}: Invalid format, expected KEY=VALUE", line_num));
                }
            }
            
            if !has_valid_entries {
                result.errors.push("No valid environment variable entries found".to_string());
            } else if result.errors.is_empty() {
                result.is_valid = true;
                result.format = Some(FileFormat::Env);
            }
        }
        
        FileFormat::Yaml => {
            // Basic YAML validation - check for key: value patterns
            let mut has_valid_entries = false;
            let mut line_num = 0;
            
            for line in content.lines() {
                line_num += 1;
                let line = line.trim();
                
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                
                if line.contains(':') {
                    has_valid_entries = true;
                } else if !line.starts_with('-') && !line.starts_with(' ') {
                    result.warnings.push(format!("Line {}: May not be valid YAML format", line_num));
                }
            }
            
            if has_valid_entries {
                result.is_valid = true;
                result.format = Some(FileFormat::Yaml);
                result.warnings.push("YAML format detected but full validation requires YAML parser".to_string());
            } else {
                result.errors.push("No valid YAML entries found".to_string());
            }
        }
    }
    
    Ok(result)
}

/// Convert between different configuration formats
pub fn convert_format(
    input_path: &Path,
    output_path: &Path,
    source_format: &FileFormat,
    target_format: &FileFormat,
) -> Result<(), Box<dyn std::error::Error>> {
    if source_format == target_format {
        // Just copy the file
        fs::copy(input_path, output_path)?;
        return Ok(());
    }
    
    // Read and parse source format
    let variables = match source_format {
        FileFormat::Json => parse_json_file(input_path)?,
        FileFormat::Env => read_env_file(input_path.to_str().unwrap())?,
        FileFormat::Yaml => {
            return Err("YAML parsing not yet implemented for format conversion".into());
        }
    };
    
    // Write in target format
    match target_format {
        FileFormat::Json => write_json_file(output_path, &variables, true)?,
        FileFormat::Env => write_env_file(output_path, &variables)?,
        FileFormat::Yaml => write_yaml_file(output_path, &variables)?,
    }
    
    Ok(())
}

/// Parse JSON file into environment variables
fn parse_json_file(path: &Path) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let json_value: serde_json::Value = serde_json::from_str(&content)?;
    
    let mut variables = HashMap::new();
    
    match json_value {
        serde_json::Value::Object(obj) => {
            for (key, value) in obj {
                let string_value = match value {
                    serde_json::Value::String(s) => s,
                    serde_json::Value::Number(n) => n.to_string(),
                    serde_json::Value::Bool(b) => b.to_string(),
                    serde_json::Value::Null => String::new(),
                    _ => serde_json::to_string(&value)?.trim_matches('"').to_string(),
                };
                variables.insert(key, string_value);
            }
        }
        _ => return Err("JSON file must contain an object at root level".into()),
    }
    
    Ok(variables)
}

/// Write variables to JSON file
fn write_json_file(
    path: &Path,
    variables: &HashMap<String, String>,
    pretty: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let content = if pretty {
        serde_json::to_string_pretty(variables)?
    } else {
        serde_json::to_string(variables)?
    };
    
    fs::write(path, content)?;
    Ok(())
}

/// Write variables to ENV file
fn write_env_file(
    path: &Path,
    variables: &HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut content = String::new();
    content.push_str("# Environment variables\n");
    content.push_str(&format!("# Generated on {}\n\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
    
    let mut sorted_vars: Vec<_> = variables.iter().collect();
    sorted_vars.sort_by_key(|(k, _)| *k);
    
    for (key, value) in sorted_vars {
        // Quote values that contain spaces or special characters
        let quoted_value = if value.contains(' ') || value.contains('\t') || value.contains('\n') {
            format!("\"{}\"", value.replace('"', "\\\""))
        } else {
            value.clone()
        };
        
        content.push_str(&format!("{}={}\n", key, quoted_value));
    }
    
    fs::write(path, content)?;
    Ok(())
}

/// Write variables to YAML file (basic implementation)
fn write_yaml_file(
    path: &Path,
    variables: &HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut content = String::new();
    content.push_str("# Environment variables\n");
    content.push_str(&format!("# Generated on {}\n\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
    
    let mut sorted_vars: Vec<_> = variables.iter().collect();
    sorted_vars.sort_by_key(|(k, _)| *k);
    
    for (key, value) in sorted_vars {
        // Basic YAML formatting - quote values that need it
        let yaml_value = if value.contains(':') || value.contains('#') || value.contains('\n') ||
                           value.starts_with(' ') || value.ends_with(' ') {
            format!("\"{}\"", value.replace('"', "\\\""))
        } else {
            value.clone()
        };
        
        content.push_str(&format!("{}: {}\n", key, yaml_value));
    }
    
    fs::write(path, content)?;
    Ok(())
}