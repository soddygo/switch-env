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