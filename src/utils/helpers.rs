use std::collections::HashMap;

/// Check if a key contains sensitive information that should be masked
pub fn is_sensitive_key(key: &str) -> bool {
    let sensitive_patterns = [
        "TOKEN", "KEY", "SECRET", "PASSWORD", "AUTH", "CREDENTIAL", "API_KEY"
    ];
    
    let upper_key = key.to_uppercase();
    sensitive_patterns.iter().any(|pattern| upper_key.contains(pattern))
}

/// Mask sensitive values for display
pub fn mask_sensitive_value(value: &str) -> String {
    if value.len() <= 8 {
        "*".repeat(value.len())
    } else {
        format!("{}***{}", &value[..4], &value[value.len()-4..])
    }
}

/// Check if the configuration appears to be for Claude
pub fn is_claude_configuration(variables: &HashMap<String, String>) -> bool {
    let claude_indicators = [
        "ANTHROPIC_BASE_URL",
        "ANTHROPIC_MODEL", 
        "ANTHROPIC_AUTH_TOKEN",
        "ANTHROPIC_SMALL_FAST_MODEL"
    ];
    
    claude_indicators.iter().any(|key| variables.contains_key(*key))
}

/// Find configurations with similar names (simple string distance)
pub fn find_similar_configs(target: &str, available: &[String]) -> Vec<String> {
    let mut suggestions = Vec::new();
    let target_lower = target.to_lowercase();
    
    for config in available {
        let config_lower = config.to_lowercase();
        
        // Check for substring matches
        if config_lower.contains(&target_lower) || target_lower.contains(&config_lower) {
            suggestions.push(config.clone());
            continue;
        }
        
        // Check for similar length and character overlap
        if (config.len() as i32 - target.len() as i32).abs() <= 2 {
            let similarity = calculate_similarity(&target_lower, &config_lower);
            if similarity > 0.6 {
                suggestions.push(config.clone());
            }
        }
    }
    
    // Limit to top 3 suggestions
    suggestions.truncate(3);
    suggestions
}

/// Calculate simple similarity score between two strings
fn calculate_similarity(s1: &str, s2: &str) -> f64 {
    let chars1: std::collections::HashSet<char> = s1.chars().collect();
    let chars2: std::collections::HashSet<char> = s2.chars().collect();
    
    let intersection = chars1.intersection(&chars2).count();
    let union = chars1.union(&chars2).count();
    
    if union == 0 {
        0.0
    } else {
        intersection as f64 / union as f64
    }
}