# Design Document

## Overview

This design document outlines the implementation approach for the missing core features in EnvSwitch: export, import, delete, and edit functionality. The design leverages the existing architecture and extends the current command handlers to provide full functionality.

## Architecture

The implementation follows the existing modular architecture:

```
src/
├── commands/
│   ├── import_export.rs     # Export/Import command handlers (to be implemented)
│   ├── config_commands.rs   # Edit/Delete command handlers (to be extended)
│   └── router.rs           # Command routing (already configured)
├── config.rs               # Configuration management (has advanced methods)
├── handlers/               # UI and interaction handlers
└── utils/                  # Utility functions
```

## Components and Interfaces

### 1. Export Command Handler

**Location:** `src/commands/import_export.rs::handle_export_command`

**Interface:**
```rust
pub fn handle_export_command(
    config_manager: &FileConfigManager,
    output: Option<String>,
    configs: Vec<String>,
    format: String,
    metadata: bool,
    pretty: bool,
    verbose: bool,
) -> Result<(), Box<dyn Error>>
```

**Responsibilities:**
- Validate export parameters and resolve output file path
- Filter configurations if specific configs are requested
- Use `FileConfigManager::export_to_file_with_options()` for actual export
- Handle different export formats (JSON, ENV, YAML)
- Provide user feedback and error handling

### 2. Import Command Handler

**Location:** `src/commands/import_export.rs::handle_import_command`

**Interface:**
```rust
pub fn handle_import_command(
    config_manager: &FileConfigManager,
    file: String,
    force: bool,
    merge: bool,
    dry_run: bool,
    skip_validation: bool,
    backup: bool,
    verbose: bool,
) -> Result<(), Box<dyn Error>>
```

**Responsibilities:**
- Validate import file existence and format
- Create backup if requested using `FileConfigManager::backup_config()`
- Use `FileConfigManager::import_from_file_with_options()` for actual import
- Handle conflict resolution (force, merge, or prompt user)
- Display import results and statistics

### 3. Delete Command Handler

**Location:** `src/commands/config_commands.rs::handle_delete_command`

**Interface:**
```rust
pub fn handle_delete_command(
    config_manager: &FileConfigManager,
    alias: String,
    force: bool,
    verbose: bool,
) -> Result<(), Box<dyn Error>>
```

**Responsibilities:**
- Validate configuration exists
- Handle confirmation prompt unless force flag is used
- Check if deleting active configuration and handle appropriately
- Use `ConfigManager::delete_config()` for actual deletion
- Provide user feedback

### 4. Edit Command Handler

**Location:** `src/commands/config_commands.rs::handle_edit_command`

**Interface:**
```rust
pub fn handle_edit_command(
    config_manager: &FileConfigManager,
    alias: String,
    verbose: bool,
) -> Result<(), Box<dyn Error>>
```

**Responsibilities:**
- Load existing configuration or offer to create new one
- Present interactive editing interface
- Validate changes before applying
- Use `ConfigManager::update_config()` to save changes
- Handle cancellation gracefully

## Data Models

### Export Options Structure
```rust
pub struct ExportOptions {
    pub format: ExportFormat,
    pub include_metadata: bool,
    pub pretty_print: bool,
    pub configs: Option<Vec<String>>,
}
```

### Import Options Structure
```rust
pub struct ImportOptions {
    pub format: ImportFormat,
    pub force_overwrite: bool,
    pub merge_existing: bool,
    pub skip_validation: bool,
    pub dry_run: bool,
}
```

### Import Result Structure
```rust
pub struct ImportResult {
    pub imported: Vec<String>,
    pub conflicts: Vec<String>,
    pub errors: Vec<String>,
}
```

## Error Handling

### Export Error Scenarios
- **File Permission Errors**: Provide clear message about directory permissions
- **Invalid Configuration Names**: List available configurations
- **Disk Space Issues**: Inform user about storage requirements

### Import Error Scenarios
- **File Not Found**: Suggest checking file path and permissions
- **Invalid Format**: Show supported formats and validation errors
- **Configuration Conflicts**: Present resolution options (force, merge, skip)
- **Validation Failures**: Highlight specific validation issues

### Delete Error Scenarios
- **Configuration Not Found**: Suggest similar configuration names
- **Active Configuration Deletion**: Warn user and explain consequences
- **File System Errors**: Provide recovery suggestions

### Edit Error Scenarios
- **Configuration Not Found**: Offer to create new configuration
- **Validation Failures**: Show specific validation errors
- **Save Failures**: Suggest backup and retry options

## Testing Strategy

### Unit Tests
- Test each command handler with various parameter combinations
- Mock file system operations for error scenario testing
- Validate error message content and format
- Test configuration validation logic

### Integration Tests
- End-to-end export/import workflows
- Configuration lifecycle (create, edit, delete)
- Cross-format compatibility (JSON ↔ ENV)
- Backup and restore functionality

### Error Scenario Tests
- File permission issues
- Corrupted configuration files
- Network/disk space limitations
- User cancellation scenarios

## Implementation Approach

### Phase 1: Export/Import Implementation
1. Implement `handle_export_command` using existing `FileConfigManager` methods
2. Add format detection and validation logic
3. Implement `handle_import_command` with conflict resolution
4. Add comprehensive error handling and user feedback

### Phase 2: Delete/Edit Implementation
1. Implement `handle_delete_command` with confirmation prompts
2. Create interactive editing interface for `handle_edit_command`
3. Add validation and error recovery mechanisms
4. Integrate with existing configuration management

### Phase 3: Enhanced User Experience
1. Add progress indicators for long operations
2. Implement smart suggestions for typos and errors
3. Add verbose mode output for debugging
4. Create comprehensive help messages

## User Interface Design

### Export Command Output
```
✅ Exported 3 configurations to configs.json
📊 Total: 15 environment variables
💾 File size: 2.1 KB
🔧 Configurations: deepseek, kimi, openai
```

### Import Command Output
```
📥 Importing configurations from configs.json...
✅ Successfully imported 2 configurations
⚠️  1 conflict found: 'deepseek' already exists
📝 Use --force to overwrite or --merge to combine
```

### Delete Command Interaction
```
⚠️  Delete configuration 'deepseek'? This cannot be undone.
   Variables: 3 (ANTHROPIC_BASE_URL, ANTHROPIC_MODEL, ANTHROPIC_AUTH_TOKEN)
   Created: 2024-01-15 10:30:00 UTC
   
Continue? [y/N]: 
```

### Edit Command Interface
```
📝 Editing configuration: deepseek
   Description: DeepSeek AI configuration
   
Current variables:
1. ANTHROPIC_BASE_URL = https://api.deepseek.com
2. ANTHROPIC_MODEL = deepseek-chat
3. ANTHROPIC_AUTH_TOKEN = sk-***

Actions: [a]dd, [e]dit, [d]elete, [s]ave, [q]uit
> 
```