use std::io::{self, Write};
use std::time::{Duration, Instant};
use std::thread;
use std::collections::HashMap;

/// Progress indicator for long-running operations
pub struct ProgressIndicator {
    message: String,
    start_time: Instant,
    pub is_running: bool,
}

impl ProgressIndicator {
    /// Create a new progress indicator with a message
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            start_time: Instant::now(),
            is_running: false,
        }
    }
    
    /// Start the progress indicator
    pub fn start(&mut self) {
        self.is_running = true;
        self.start_time = Instant::now();
        print!("{} ", self.message);
        io::stdout().flush().unwrap();
    }
    
    /// Update progress with a dot
    pub fn tick(&self) {
        if self.is_running {
            print!(".");
            io::stdout().flush().unwrap();
        }
    }
    
    /// Finish the progress indicator with success
    pub fn finish_success(&mut self, result_message: &str) {
        if self.is_running {
            let elapsed = self.start_time.elapsed();
            println!(" ✅ {} ({:.1}s)", result_message, elapsed.as_secs_f64());
            self.is_running = false;
        }
    }
    
    /// Finish the progress indicator with error
    pub fn finish_error(&mut self, error_message: &str) {
        if self.is_running {
            let elapsed = self.start_time.elapsed();
            println!(" ❌ {} ({:.1}s)", error_message, elapsed.as_secs_f64());
            self.is_running = false;
        }
    }
    
    /// Finish the progress indicator with warning
    pub fn finish_warning(&mut self, warning_message: &str) {
        if self.is_running {
            let elapsed = self.start_time.elapsed();
            println!(" ⚠️  {} ({:.1}s)", warning_message, elapsed.as_secs_f64());
            self.is_running = false;
        }
    }
}

/// Simulate progress for operations that don't have real progress tracking
pub fn simulate_progress<F, R>(message: &str, operation: F) -> R
where
    F: FnOnce() -> R,
{
    let mut progress = ProgressIndicator::new(message);
    progress.start();
    
    // Start a background thread to show progress
    let progress_handle = thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(100));
            print!(".");
            io::stdout().flush().unwrap();
        }
    });
    
    // Execute the operation
    let result = operation();
    
    // Wait for progress thread to finish (or timeout)
    let _ = progress_handle.join();
    
    result
}

/// Display user-friendly error messages with suggestions
pub fn display_error_with_suggestions(error: &dyn std::error::Error, verbose: bool) {
    println!("❌ Error: {}", error);
    
    if verbose {
        // Show error chain
        let mut source = error.source();
        let mut level = 1;
        while let Some(err) = source {
            println!("   {}: {}", level, err);
            source = err.source();
            level += 1;
        }
    }
    
    // Provide context-specific suggestions
    let error_str = error.to_string().to_lowercase();
    
    if error_str.contains("permission denied") {
        println!();
        println!("💡 Suggestions:");
        println!("   • Check file permissions: ls -la ~/.config/envswitch/");
        println!("   • Ensure you have write access to the configuration directory");
        println!("   • Try running with appropriate permissions");
    } else if error_str.contains("no such file or directory") {
        println!();
        println!("💡 Suggestions:");
        println!("   • Check if the file path is correct");
        println!("   • Ensure the directory exists");
        println!("   • Use absolute paths if relative paths aren't working");
    } else if error_str.contains("not found") {
        println!();
        println!("💡 Suggestions:");
        println!("   • Use 'envswitch list' to see available configurations");
        println!("   • Check spelling of configuration names");
        println!("   • Create the configuration first with 'envswitch set'");
    } else if error_str.contains("json") || error_str.contains("parse") {
        println!();
        println!("💡 Suggestions:");
        println!("   • Check file format - ensure it's valid JSON/ENV/YAML");
        println!("   • Use 'envswitch import --dry-run' to preview import");
        println!("   • Validate file content with external tools");
    } else if error_str.contains("disk") || error_str.contains("space") {
        println!();
        println!("💡 Suggestions:");
        println!("   • Check available disk space: df -h");
        println!("   • Clean up old backup files");
        println!("   • Use a different output location");
    }
}

/// Display success messages with next steps
pub fn display_success_with_next_steps(message: &str, next_steps: &[&str]) {
    println!("✅ {}", message);
    
    if !next_steps.is_empty() {
        println!();
        println!("🚀 Next steps:");
        for step in next_steps {
            println!("   {}", step);
        }
    }
}

/// Display warning messages
pub fn display_warning(message: &str, details: Option<&[&str]>) {
    println!("⚠️  {}", message);
    
    if let Some(details) = details {
        for detail in details {
            println!("   • {}", detail);
        }
    }
}

/// Display informational messages with icons
pub fn display_info(message: &str, icon: &str) {
    println!("{} {}", icon, message);
}

/// Prompt user for confirmation with custom message
pub fn prompt_confirmation(message: &str, default_yes: bool) -> Result<bool, Box<dyn std::error::Error>> {
    let prompt = if default_yes {
        format!("{} [Y/n]: ", message)
    } else {
        format!("{} [y/N]: ", message)
    };
    
    print!("{}", prompt);
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();
    
    let result = match input.as_str() {
        "y" | "yes" => true,
        "n" | "no" => false,
        "" => default_yes, // Use default for empty input
        _ => {
            println!("Please enter 'y' for yes or 'n' for no.");
            return prompt_confirmation(message, default_yes);
        }
    };
    
    Ok(result)
}

/// Display operation summary with statistics
pub fn display_operation_summary(
    operation: &str,
    success_count: usize,
    warning_count: usize,
    error_count: usize,
    duration: Duration,
    details: Option<&[&str]>,
) {
    println!();
    println!("📊 {} Summary:", operation);
    println!("   ✅ Successful: {}", success_count);
    
    if warning_count > 0 {
        println!("   ⚠️  Warnings: {}", warning_count);
    }
    
    if error_count > 0 {
        println!("   ❌ Errors: {}", error_count);
    }
    
    println!("   ⏱️  Duration: {:.2}s", duration.as_secs_f64());
    
    if let Some(details) = details {
        println!();
        println!("📋 Details:");
        for detail in details {
            println!("   • {}", detail);
        }
    }
}

/// Display file operation results
pub fn display_file_operation_result(
    operation: &str,
    file_path: &str,
    file_size: Option<u64>,
    success: bool,
) {
    let icon = if success { "✅" } else { "❌" };
    let status = if success { "completed" } else { "failed" };
    
    print!("{} {} {} for: {}", icon, operation, status, file_path);
    
    if let Some(size) = file_size {
        let size_str = format_file_size(size);
        print!(" ({})", size_str);
    }
    
    println!();
}

/// Format file size in human-readable format
pub fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

/// Display verbose operation details
pub fn display_verbose_info(message: &str, details: &[(&str, &str)]) {
    println!("🔍 {}", message);
    for (key, value) in details {
        println!("   {}: {}", key, value);
    }
}

/// Display conflict resolution options
pub fn display_conflict_resolution_options(conflicts: &[String]) -> Result<String, Box<dyn std::error::Error>> {
    println!("⚠️  {} conflicts found:", conflicts.len());
    for (i, conflict) in conflicts.iter().enumerate() {
        println!("   {}. {}", i + 1, conflict);
    }
    
    println!();
    println!("Resolution options:");
    println!("   [f]orce    - Overwrite existing configurations");
    println!("   [m]erge    - Merge with existing configurations");
    println!("   [s]kip     - Skip conflicting configurations");
    println!("   [c]ancel   - Cancel the operation");
    println!();
    
    loop {
        print!("Choose resolution [f/m/s/c]: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();
        
        match input.as_str() {
            "f" | "force" => return Ok("force".to_string()),
            "m" | "merge" => return Ok("merge".to_string()),
            "s" | "skip" => return Ok("skip".to_string()),
            "c" | "cancel" => return Ok("cancel".to_string()),
            _ => {
                println!("Invalid option. Please choose f, m, s, or c.");
                continue;
            }
        }
    }
}

/// Prompt for text input with optional default value
pub fn prompt_for_input(prompt: &str, default: Option<&str>) -> Result<String, Box<dyn std::error::Error>> {
    let display_prompt = if let Some(default_val) = default {
        format!("{} [{}]: ", prompt, default_val)
    } else {
        format!("{}: ", prompt)
    };
    
    print!("{}", display_prompt);
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();
    
    if input.is_empty() {
        if let Some(default_val) = default {
            Ok(default_val.to_string())
        } else {
            Ok(String::new())
        }
    } else {
        Ok(input.to_string())
    }
}

/// Prompt for non-empty text input with validation
pub fn prompt_for_required_input(prompt: &str, validator: Option<fn(&str) -> Result<(), String>>) -> Result<String, Box<dyn std::error::Error>> {
    loop {
        let input = prompt_for_input(prompt, None)?;
        
        if input.is_empty() {
            println!("❌ Input cannot be empty. Please try again.");
            continue;
        }
        
        if let Some(validate) = validator {
            match validate(&input) {
                Ok(()) => return Ok(input),
                Err(error) => {
                    println!("❌ {}", error);
                    continue;
                }
            }
        } else {
            return Ok(input);
        }
    }
}

/// Display interactive menu and get user selection
pub fn display_interactive_menu(title: &str, options: &[(&str, &str)]) -> Result<String, Box<dyn std::error::Error>> {
    println!("{}", title);
    println!();
    
    for (key, description) in options {
        println!("   [{}] - {}", key, description);
    }
    
    println!();
    
    let valid_keys: Vec<&str> = options.iter().map(|(key, _)| *key).collect();
    
    loop {
        print!("> ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();
        
        if valid_keys.contains(&input.as_str()) {
            return Ok(input);
        } else {
            println!("❌ Invalid option. Please choose from: {}", valid_keys.join(", "));
        }
    }
}

/// Create a reusable confirmation dialog
pub struct ConfirmationDialog {
    message: String,
    details: Vec<String>,
    default_yes: bool,
}

impl ConfirmationDialog {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            details: Vec::new(),
            default_yes: false,
        }
    }
    
    pub fn with_details(mut self, details: Vec<String>) -> Self {
        self.details = details;
        self
    }
    
    pub fn with_default_yes(mut self) -> Self {
        self.default_yes = true;
        self
    }
    
    pub fn show(self) -> Result<bool, Box<dyn std::error::Error>> {
        println!("⚠️  {}", self.message);
        
        for detail in &self.details {
            println!("   {}", detail);
        }
        
        if !self.details.is_empty() {
            println!();
        }
        
        prompt_confirmation("Continue?", self.default_yes)
    }
}

/// Interactive variable editor for configuration editing
pub struct VariableEditor {
    variables: std::collections::HashMap<String, String>,
    changed: bool,
}

impl VariableEditor {
    pub fn new(variables: std::collections::HashMap<String, String>) -> Self {
        Self {
            variables,
            changed: false,
        }
    }
    
    pub fn edit_interactively(mut self) -> Result<(std::collections::HashMap<String, String>, bool), Box<dyn std::error::Error>> {
        loop {
            self.display_current_variables();
            
            let action = display_interactive_menu(
                "📝 Variable Editor",
                &[
                    ("a", "Add a new variable"),
                    ("e", "Edit an existing variable"),
                    ("d", "Delete a variable"),
                    ("s", "Save changes and exit"),
                    ("q", "Quit without saving"),
                ]
            )?;
            
            match action.as_str() {
                "a" => {
                    if let Some((key, value)) = self.prompt_for_new_variable()? {
                        self.variables.insert(key.clone(), value);
                        self.changed = true;
                        println!("✅ Added variable '{}'", key);
                    }
                }
                "e" => {
                    if self.variables.is_empty() {
                        println!("❌ No variables to edit. Use 'add' to create variables first.");
                        continue;
                    }
                    self.edit_existing_variable()?;
                }
                "d" => {
                    if self.variables.is_empty() {
                        println!("❌ No variables to delete.");
                        continue;
                    }
                    self.delete_variable()?;
                }
                "s" => {
                    return Ok((self.variables, self.changed));
                }
                "q" => {
                    if self.changed {
                        let confirm = prompt_confirmation("You have unsaved changes. Quit anyway?", false)?;
                        if !confirm {
                            continue;
                        }
                    }
                    return Ok((self.variables, false));
                }
                _ => unreachable!(),
            }
            
            println!();
        }
    }
    
    fn display_current_variables(&self) {
        if self.variables.is_empty() {
            println!("📋 Current variables: (none)");
        } else {
            println!("📋 Current variables:");
            let mut sorted_vars: Vec<_> = self.variables.iter().collect();
            sorted_vars.sort_by_key(|(k, _)| *k);
            
            for (i, (key, value)) in sorted_vars.iter().enumerate() {
                // Basic masking for sensitive-looking keys
                let display_value = if key.to_uppercase().contains("TOKEN") || 
                                      key.to_uppercase().contains("KEY") || 
                                      key.to_uppercase().contains("SECRET") {
                    format!("{}***", &value.chars().take(4).collect::<String>())
                } else {
                    value.to_string()
                };
                println!("   {}. {} = {}", i + 1, key, display_value);
            }
        }
        println!();
    }
    
    fn prompt_for_new_variable(&self) -> Result<Option<(String, String)>, Box<dyn std::error::Error>> {
        let key = prompt_for_required_input("Enter variable name", Some(|name| {
            if name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                Ok(())
            } else {
                Err("Variable name must contain only letters, numbers, and underscores".to_string())
            }
        }))?;
        
        if self.variables.contains_key(&key) {
            println!("⚠️  Variable '{}' already exists. Use 'edit' to modify it.", key);
            return Ok(None);
        }
        
        let value = prompt_for_input("Enter variable value", None)?;
        Ok(Some((key, value)))
    }
    
    fn edit_existing_variable(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let key = prompt_for_input("Enter variable name to edit", None)?;
        
        if let Some(current_value) = self.variables.get(&key) {
            let display_value = if key.to_uppercase().contains("TOKEN") || 
                                  key.to_uppercase().contains("KEY") || 
                                  key.to_uppercase().contains("SECRET") {
                format!("{}***", &current_value.chars().take(4).collect::<String>())
            } else {
                current_value.clone()
            };
            
            println!("Current value: {}", display_value);
            let new_value = prompt_for_input("Enter new value (or press Enter to keep current)", None)?;
            
            if !new_value.is_empty() {
                self.variables.insert(key.clone(), new_value);
                self.changed = true;
                println!("✅ Updated variable '{}'", key);
            } else {
                println!("⏭️  Variable '{}' unchanged", key);
            }
        } else {
            println!("❌ Variable '{}' not found.", key);
        }
        
        Ok(())
    }
    
    fn delete_variable(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let key = prompt_for_input("Enter variable name to delete", None)?;
        
        if self.variables.remove(&key).is_some() {
            self.changed = true;
            println!("✅ Deleted variable '{}'", key);
        } else {
            println!("❌ Variable '{}' not found.", key);
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(0), "0 B");
        assert_eq!(format_file_size(512), "512 B");
        assert_eq!(format_file_size(1024), "1.0 KB");
        assert_eq!(format_file_size(1536), "1.5 KB");
        assert_eq!(format_file_size(1048576), "1.0 MB");
        assert_eq!(format_file_size(1073741824), "1.0 GB");
    }

    #[test]
    fn test_progress_indicator() {
        let mut progress = ProgressIndicator::new("Testing");
        assert!(!progress.is_running);
        
        progress.start();
        assert!(progress.is_running);
        
        progress.finish_success("Done");
        assert!(!progress.is_running);
    }
}