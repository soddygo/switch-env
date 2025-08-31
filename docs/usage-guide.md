# EnvSwitch Usage Guide

This comprehensive guide covers all aspects of using EnvSwitch effectively, from basic operations to advanced workflows.

## Table of Contents

- [Getting Started](#getting-started)
- [Basic Operations](#basic-operations)
- [Advanced Features](#advanced-features)
- [Import/Export Operations](#importexport-operations)
- [Interactive Configuration Management](#interactive-configuration-management)
- [Shell Integration](#shell-integration)
- [Best Practices](#best-practices)
- [Troubleshooting](#troubleshooting)

## Getting Started

### First Time Setup

1. **Install EnvSwitch** (see README for installation instructions)

2. **Verify installation:**
   ```bash
   envswitch --version
   envswitch --help
   ```

3. **Create your first configuration:**
   ```bash
   envswitch set my-first-config \
     -e EXAMPLE_VAR=hello_world \
     -e DEBUG=true \
     -d "My first EnvSwitch configuration"
   ```

4. **List configurations:**
   ```bash
   envswitch list
   ```

5. **Use the configuration:**
   ```bash
   eval "$(envswitch use my-first-config)"
   echo $EXAMPLE_VAR  # Should output: hello_world
   ```

### Understanding EnvSwitch Concepts

- **Configuration**: A named set of environment variables
- **Alias**: The name you give to a configuration
- **Active Configuration**: The currently loaded configuration
- **Export/Import**: Save/load configurations to/from files
- **Interactive Editor**: Built-in editor for managing configurations

## Basic Operations

### Creating Configurations

#### Simple Configuration
```bash
envswitch set myconfig -e VAR1=value1 -e VAR2=value2
```

#### With Description
```bash
envswitch set myconfig \
  -e VAR1=value1 \
  -e VAR2=value2 \
  -d "Description of this configuration"
```

#### From File
```bash
# Create a file with KEY=VALUE pairs
echo "VAR1=value1" > vars.env
echo "VAR2=value2" >> vars.env

envswitch set myconfig --file vars.env
```

#### Interactive Mode
```bash
envswitch set myconfig --interactive
# Follow the prompts to add variables one by one
```

### Viewing Configurations

#### List All Configurations
```bash
envswitch list                    # Simple list
envswitch list --verbose          # Detailed information
envswitch list --table            # Table format
envswitch list --active           # Show only active configuration
```

#### Show Specific Configuration
```bash
envswitch show myconfig           # Show configuration details
```

#### Check Current Status
```bash
envswitch status                  # Show current environment status
envswitch status --claude         # Show only Claude-specific variables
envswitch status --mismatched     # Show only mismatched variables
```

### Using Configurations

#### Switch to Configuration
```bash
eval "$(envswitch use myconfig)"
```

#### Preview Switch (Dry Run)
```bash
envswitch use myconfig --dry-run
```

#### For Fish Shell
```bash
eval (envswitch use myconfig)
```

### Updating Configurations

#### Add/Update Variables
```bash
envswitch set myconfig -e NEW_VAR=new_value
```

#### Replace All Variables
```bash
envswitch set myconfig -e VAR1=value1 -e VAR2=value2 --replace
```

#### Update Description
```bash
envswitch set myconfig -d "Updated description"
```

### Deleting Configurations

#### Interactive Deletion (with confirmation)
```bash
envswitch delete myconfig
```

#### Force Deletion (no confirmation)
```bash
envswitch delete myconfig --force
```

#### Verbose Deletion
```bash
envswitch delete myconfig --verbose
```

## Advanced Features

### Configuration Management

#### Copying Configurations
```bash
# Export and re-import with new name
envswitch export -c source-config -o temp.json
# Edit temp.json to change the configuration name
envswitch import temp.json
```

#### Merging Configurations
```bash
# Create base configuration
envswitch set base -e COMMON_VAR=value

# Create specific configuration that extends base
envswitch set specific -e SPECIFIC_VAR=value

# Use both (variables from both configurations)
eval "$(envswitch use base)"
eval "$(envswitch use specific)"
```

#### Configuration Templates
```bash
# Create template with placeholder values
envswitch set template \
  -e API_URL=REPLACE_WITH_ACTUAL_URL \
  -e API_KEY=REPLACE_WITH_ACTUAL_KEY \
  -d "Template configuration - replace placeholders"

# Export template
envswitch export -c template -o template.json

# Others can import and customize
envswitch import template.json
envswitch edit template  # Replace placeholders
```

### Environment Variable Management

#### Variable Naming Conventions
- Use UPPER_CASE for environment variables
- Use underscores to separate words
- Start with letter or underscore
- Avoid special characters

```bash
# ✅ Good variable names
envswitch set config -e API_BASE_URL=https://api.example.com
envswitch set config -e _PRIVATE_VAR=secret
envswitch set config -e DEBUG_MODE=true

# ❌ Bad variable names (will cause errors)
envswitch set config -e "api-url=value"      # Hyphens not allowed
envswitch set config -e "123var=value"       # Can't start with number
envswitch set config -e "var.name=value"     # Dots not allowed
```

#### Handling Special Values
```bash
# URLs with special characters
envswitch set config -e API_URL="https://api.example.com/v1?key=value&other=param"

# Multi-line values
envswitch set config -e MULTI_LINE="line1
line2
line3"

# Empty values
envswitch set config -e EMPTY_VAR=""

# Values with spaces
envswitch set config -e MESSAGE="Hello World"
```

## Import/Export Operations

### Export Operations

#### Basic Export
```bash
envswitch export -o my-configs.json
```

#### Export Specific Configurations
```bash
envswitch export -c "config1,config2,config3" -o selected-configs.json
```

#### Export with Metadata
```bash
envswitch export --metadata --pretty -o detailed-configs.json
```

#### Export in Different Formats
```bash
envswitch export --format json -o configs.json    # JSON format (default)
envswitch export --format env -o configs.env      # Environment file format
envswitch export --format yaml -o configs.yaml    # YAML format
```

#### Export with Verbose Output
```bash
envswitch export --verbose -o configs.json
```

### Import Operations

#### Basic Import
```bash
envswitch import configs.json
```

#### Import with Backup
```bash
envswitch import configs.json --backup
```

#### Import with Conflict Resolution
```bash
envswitch import configs.json --merge     # Merge with existing
envswitch import configs.json --force     # Overwrite existing
```

#### Preview Import (Dry Run)
```bash
envswitch import configs.json --dry-run
```

#### Import with Validation Control
```bash
envswitch import configs.json --skip-validation  # Skip validation for speed
```

#### Import Different Formats
```bash
envswitch import configs.json    # JSON format (auto-detected)
envswitch import configs.env     # ENV format (auto-detected)
envswitch import configs.yaml    # YAML format (auto-detected)
```

### Format Detection

EnvSwitch automatically detects file formats based on:
1. File extension (.json, .env, .yaml)
2. File content analysis
3. Explicit format specification

#### Manual Format Specification
```bash
envswitch export --format env -o configs.txt     # Force ENV format
envswitch import configs.txt                     # Auto-detects content
```

## Interactive Configuration Management

### Using the Interactive Editor

#### Start Interactive Editor
```bash
envswitch edit myconfig
```

#### Editor Commands
- `a` - Add a new variable
- `e` - Edit an existing variable
- `d` - Delete a variable
- `desc` - Update configuration description
- `s` - Save changes and exit
- `q` - Quit without saving

#### Interactive Editor Workflow
```bash
# Start editor
envswitch edit myconfig

# Example session:
# > Current configuration: myconfig
# > Variables: VAR1=value1, VAR2=value2
# > 
# > Commands: [a]dd, [e]dit, [d]elete, [desc]ription, [s]ave, [q]uit
# > Choice: a
# > Variable name: NEW_VAR
# > Variable value: new_value
# > Added NEW_VAR=new_value
# > 
# > Commands: [a]dd, [e]dit, [d]elete, [desc]ription, [s]ave, [q]uit
# > Choice: s
# > Configuration saved successfully!
```

### Creating New Configurations Interactively

```bash
# Edit non-existent configuration (will offer to create)
envswitch edit new-config

# > Configuration 'new-config' not found. Create it? (y/n): y
# > Created new configuration 'new-config'
# > 
# > Commands: [a]dd, [e]dit, [d]elete, [desc]ription, [s]ave, [q]uit
# > Choice: a
# > Variable name: FIRST_VAR
# > Variable value: first_value
# > ...
```

## Shell Integration

### Shell Detection

EnvSwitch automatically detects your shell:
- Bash
- Zsh
- Fish
- Sh (POSIX shell)

#### Manual Shell Specification
```bash
# Force specific shell
SHELL=/bin/zsh envswitch use myconfig
```

### Shell-Specific Usage

#### Bash/Zsh
```bash
# Basic usage
eval "$(envswitch use myconfig)"

# With error handling
if output=$(envswitch use myconfig 2>&1); then
    eval "$output"
else
    echo "Failed to switch configuration: $output"
fi
```

#### Fish Shell
```bash
# Basic usage
eval (envswitch use myconfig)

# With error handling
if set output (envswitch use myconfig 2>&1)
    eval $output
else
    echo "Failed to switch configuration: $output"
end
```

### Shell Aliases and Functions

#### Recommended Aliases
Add to your shell configuration file:

```bash
# For Bash/Zsh (~/.bashrc, ~/.zshrc)
alias es='envswitch'
alias esl='envswitch list'
alias ess='envswitch status'
alias esu='envswitch use'
alias ese='envswitch edit'

# Quick switching aliases
alias switch-dev='eval "$(envswitch use dev)"'
alias switch-prod='eval "$(envswitch use prod)"'
alias switch-staging='eval "$(envswitch use staging)"'
```

```fish
# For Fish (~/.config/fish/config.fish)
alias es='envswitch'
alias esl='envswitch list'
alias ess='envswitch status'
alias esu='envswitch use'
alias ese='envswitch edit'

# Quick switching functions
function switch-dev
    eval (envswitch use dev)
end

function switch-prod
    eval (envswitch use prod)
end
```

#### Advanced Shell Functions

```bash
# Bash/Zsh function for safe switching
switch_env() {
    if [ -z "$1" ]; then
        echo "Usage: switch_env <config_name>"
        envswitch list
        return 1
    fi
    
    if envswitch show "$1" > /dev/null 2>&1; then
        eval "$(envswitch use "$1")"
        echo "Switched to configuration: $1"
    else
        echo "Configuration '$1' not found"
        envswitch list
        return 1
    fi
}
```

### Shell Integration Setup

#### Automatic Setup
```bash
envswitch setup --install
```

#### Manual Setup
```bash
# Generate setup script
envswitch setup --generate -o envswitch-setup.sh

# Review and source the script
cat envswitch-setup.sh
source envswitch-setup.sh
```

## Best Practices

### Configuration Organization

#### Naming Conventions
```bash
# Use descriptive, hierarchical names
envswitch set myapp-dev-database
envswitch set myapp-prod-api
envswitch set myapp-staging-frontend

# Include environment indicators
envswitch set dev-myservice
envswitch set prod-myservice
envswitch set test-myservice

# Use consistent patterns
envswitch set project-env-component
```

#### Configuration Descriptions
```bash
# Always include descriptions
envswitch set myconfig \
  -e VAR=value \
  -d "Development configuration for MyApp API service"

# Include important notes
envswitch set prod-secrets \
  -e SECRET_KEY=... \
  -d "PRODUCTION SECRETS - Handle with extreme care. Rotate monthly."
```

### Security Best Practices

#### Sensitive Data Handling
```bash
# Don't export sensitive configurations to shared locations
envswitch export -c "dev,staging" -o shareable-configs.json  # Exclude prod

# Use separate configurations for secrets
envswitch set app-config -e DEBUG=true -e LOG_LEVEL=info
envswitch set app-secrets -e API_KEY=secret -e DB_PASSWORD=secret

# Load both when needed
eval "$(envswitch use app-config)"
eval "$(envswitch use app-secrets)"
```

#### Access Control
```bash
# Set proper file permissions
chmod 700 ~/.config/envswitch/
chmod 600 ~/.config/envswitch/config.json

# Limit backup access
chmod 600 ~/.config/envswitch/backups/*.json
```

### Backup Strategies

#### Regular Backups
```bash
# Daily backup script
#!/bin/bash
DATE=$(date +%Y%m%d)
BACKUP_DIR="$HOME/envswitch-backups"
mkdir -p "$BACKUP_DIR"

envswitch export -o "$BACKUP_DIR/envswitch-$DATE.json" --metadata --pretty

# Keep only last 30 days
find "$BACKUP_DIR" -name "envswitch-*.json" -mtime +30 -delete
```

#### Pre-Change Backups
```bash
# Before major changes
envswitch export -o "backup-before-$(date +%Y%m%d-%H%M%S).json" --metadata

# Before imports
envswitch import new-configs.json --backup  # Automatic backup
```

### Performance Optimization

#### Large Configuration Management
```bash
# Split large configurations into logical groups
envswitch set myapp-database -e DB_URL=... -e DB_POOL=...
envswitch set myapp-cache -e REDIS_URL=... -e CACHE_TTL=...
envswitch set myapp-api -e API_KEY=... -e API_TIMEOUT=...

# Use all together
eval "$(envswitch use myapp-database)"
eval "$(envswitch use myapp-cache)"
eval "$(envswitch use myapp-api)"
```

#### Efficient Operations
```bash
# Export only needed configurations
envswitch export -c "config1,config2" -o subset.json

# Use non-pretty format for speed
envswitch export -o configs.json  # Without --pretty

# Skip validation for trusted files
envswitch import trusted-config.json --skip-validation
```

## Troubleshooting

### Common Issues

#### Configuration Not Found
```bash
# Check available configurations
envswitch list

# Look for similar names
envswitch list | grep -i "partial-name"

# Check for typos (EnvSwitch may suggest corrections)
envswitch use myconfg  # May suggest "myconfig"
```

#### Shell Commands Not Working
```bash
# Make sure to use eval
eval "$(envswitch use myconfig)"

# Check shell type
echo $SHELL

# For fish shell, use different syntax
eval (envswitch use myconfig)

# Test command generation
envswitch use myconfig --dry-run
```

#### Import/Export Problems
```bash
# Check file format
file configs.json
python -m json.tool configs.json > /dev/null

# Use dry-run to preview
envswitch import configs.json --dry-run --verbose

# Check file permissions
ls -la configs.json
```

#### Permission Errors
```bash
# Fix configuration directory permissions
chmod 700 ~/.config/envswitch/
chmod 600 ~/.config/envswitch/config.json

# Check ownership
ls -la ~/.config/envswitch/
chown -R $USER:$USER ~/.config/envswitch/
```

### Recovery Procedures

#### Configuration File Corruption
```bash
# Check if file is valid JSON
python -m json.tool ~/.config/envswitch/config.json

# Restore from backup
ls ~/.config/envswitch/backups/
envswitch import ~/.config/envswitch/backups/latest.json --force

# Start fresh if no backups
mv ~/.config/envswitch/config.json ~/.config/envswitch/config.json.corrupted
envswitch list  # Creates new empty config
```

#### Complete System Recovery
```bash
# Backup current state
cp -r ~/.config/envswitch ~/.config/envswitch.backup

# Reinstall EnvSwitch
cargo install --git https://github.com/soddygo/envswitch --force

# Restore configurations
envswitch import ~/.config/envswitch.backup/config.json --force
```

### Getting Help

#### Verbose Output
```bash
# Use verbose mode for detailed information
envswitch command --verbose
```

#### Command Help
```bash
# General help
envswitch --help

# Command-specific help
envswitch set --help
envswitch import --help
envswitch export --help
```

#### Diagnostic Information
```bash
# System information
uname -a
echo $SHELL
envswitch --version

# Configuration status
envswitch list --verbose
ls -la ~/.config/envswitch/
```

This usage guide provides comprehensive coverage of all EnvSwitch features with practical examples and best practices for effective configuration management.