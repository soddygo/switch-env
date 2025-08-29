use std::collections::HashMap;
use std::io::{self, Write};

/// Interactive mode to collect environment variables
pub fn interactive_env_input(verbose: bool) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut variables = HashMap::new();
    
    println!("Interactive mode: Enter environment variables (press Enter with empty name to finish)");
    println!("Format: KEY=VALUE or just KEY (you'll be prompted for the value)");
    
    loop {
        print!("Variable (or press Enter to finish): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input.is_empty() {
            break;
        }
        
        let (key, value) = if let Some(eq_pos) = input.find('=') {
            // KEY=VALUE format
            let key = input[..eq_pos].trim().to_string();
            let value = input[eq_pos + 1..].trim().to_string();
            (key, value)
        } else {
            // Just KEY, prompt for value
            let key = input.to_string();
            print!("Value for '{}': ", key);
            io::stdout().flush()?;
            
            let mut value = String::new();
            io::stdin().read_line(&mut value)?;
            let value = value.trim().to_string();
            (key, value)
        };
        
        if key.is_empty() {
            println!("Variable name cannot be empty. Try again.");
            continue;
        }
        
        // Validate variable name
        if !key.chars().next().unwrap_or('0').is_ascii_alphabetic() && !key.starts_with('_') {
            println!("Variable name '{}' must start with a letter or underscore. Try again.", key);
            continue;
        }
        
        if !key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            println!("Variable name '{}' can only contain letters, numbers, and underscores. Try again.", key);
            continue;
        }
        
        if variables.contains_key(&key) {
            println!("Variable '{}' already exists. Overwriting.", key);
        }
        
        variables.insert(key.clone(), value.clone());
        
        if verbose {
            let display_value = if value.len() > 30 {
                format!("{}...", &value[..27])
            } else {
                value
            };
            println!("Added: {} = {}", key, display_value);
        } else {
            println!("Added: {}", key);
        }
    }
    
    if variables.is_empty() {
        println!("No variables entered.");
    } else {
        println!("Collected {} variables.", variables.len());
    }
    
    Ok(variables)
}

/// Prompt user for a new variable (key=value)
pub fn prompt_for_variable() -> Result<Option<(String, String)>, Box<dyn std::error::Error>> {
    print!("Enter variable name: ");
    io::stdout().flush()?;
    
    let mut key = String::new();
    io::stdin().read_line(&mut key)?;
    let key = key.trim();
    
    if key.is_empty() {
        return Ok(None);
    }
    
    // Validate variable name
    if !key.chars().next().unwrap_or('0').is_ascii_alphabetic() && !key.starts_with('_') {
        return Err("Variable name must start with a letter or underscore".into());
    }
    
    if !key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err("Variable name can only contain letters, numbers, and underscores".into());
    }
    
    print!("Enter value for '{}': ", key);
    io::stdout().flush()?;
    
    let mut value = String::new();
    io::stdin().read_line(&mut value)?;
    let value = value.trim().to_string();
    
    Ok(Some((key.to_string(), value)))
}

/// Prompt user for a variable with a default value
pub fn prompt_for_variable_with_default(
    key: &str, 
    default_value: Option<&String>
) -> Result<String, Box<dyn std::error::Error>> {
    if let Some(default) = default_value {
        print!("Value for '{}' [{}]: ", key, default);
    } else {
        print!("Value for '{}': ", key);
    }
    io::stdout().flush()?;
    
    let mut value = String::new();
    io::stdin().read_line(&mut value)?;
    let value = value.trim();
    
    if value.is_empty() {
        if let Some(default) = default_value {
            Ok(default.clone())
        } else {
            Err("Value cannot be empty".into())
        }
    } else {
        Ok(value.to_string())
    }
}

/// Prompt user to select a variable from the list
pub fn prompt_for_variable_selection(
    variables: &HashMap<String, String>,
    action: &str
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    if variables.is_empty() {
        println!("No variables available to {}.", action);
        return Ok(None);
    }
    
    println!("Select a variable to {}:", action);
    let mut vars: Vec<_> = variables.keys().collect();
    vars.sort();
    
    for (i, key) in vars.iter().enumerate() {
        println!("  {}. {}", i + 1, key);
    }
    
    print!("Enter number (1-{}) or variable name: ", vars.len());
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();
    
    if input.is_empty() {
        return Ok(None);
    }
    
    // Try to parse as number first
    if let Ok(num) = input.parse::<usize>() {
        if num >= 1 && num <= vars.len() {
            return Ok(Some(vars[num - 1].clone()));
        }
    }
    
    // Try as variable name
    if variables.contains_key(input) {
        Ok(Some(input.to_string()))
    } else {
        Err(format!("Invalid selection: '{}'", input).into())
    }
}