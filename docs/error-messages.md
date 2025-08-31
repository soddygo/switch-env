# EnvSwitch Error Messages Reference

This document provides a comprehensive reference for all error messages in EnvSwitch, their causes, and solutions.

## Table of Contents

- [Configuration Errors](#configuration-errors)
- [Import/Export Errors](#importexport-errors)
- [File System Errors](#file-system-errors)
- [Validation Errors](#validation-errors)
- [Shell Integration Errors](#shell-integration-errors)
- [Interactive Editor Errors](#interactive-editor-errors)
- [System Errors](#system-errors)

## Configuration Errors

### `Configuration 'name' not found`

**Cause:** The specified configuration doesn't exist.

**Solutions:**
```bash
# List available configurations
envswitch list

# Check for similar names (EnvSwitch may suggest alternatives)
envswitch use myconfg  # May suggest "myconfig"

# Create the configuration
envswitch set name -e KEY=value
```

### `Configuration 'name' already exists`

**Cause:** Trying to create a configuration that already exists without using update mode.

**Solutions:**
```bash
# Update existing configuration
envswitch set name -e NEW_KEY=value  # Merges with existing

# Replace entire configuration
envswitch set name -e KEY=value --replace

# Delete and recreate
envswitch delete name --force
envswitch set name -e KEY=value
```

### `Cannot delete active configuration 'name'`

**Cause:** Trying to delete the currently active configuration.

**Solutions:**
```bash
# Switch to different configuration first
envswitch use other-config

# Or clear active configuration
envswitch clear

# Then delete
envswitch delete name
```

### `Configuration name contains invalid characters`

**Cause:** Configuration name contains spaces, special characters, or other invalid characters.

**Valid characters:** Letters (a-z, A-Z), numbers (0-9), hyphens (-), underscores (_)

**Solutions:**
```bash
# ✅ Valid names
envswitch set my-config -e KEY=value
envswitch set my_config -e KEY=value
envswitch set config123 -e KEY=value

# ❌ Invalid names - fix these
envswitch set "my config"     # Remove spaces: my-config
envswitch set "my.config"     # Replace dots: my-config
envswitch set "my@config"     # Remove special chars: myconfig
```

### `Configuration description too long`

**Cause:** Configuration description exceeds maximum length (usually 500 characters).

**Solutions:**
```bash
# Shorten the description
envswitch set config -e KEY=value -d "Shorter description"

# Or omit description
envswitch set config -e KEY=value
```

## Import/Export Errors

### `Import file 'filename' not found`

**Cause:** The specified import file doesn't exist or path is incorrect.

**Solutions:**
```bash
# Check file exists
ls -la filename.json

# Use absolute path
envswitch import /full/path/to/filename.json

# Check current directory
pwd
ls -la *.json
```

### `Invalid JSON format in import file`

**Cause:** The import file contains malformed JSON.

**Solutions:**
```bash
# Validate JSON syntax
python -m json.tool filename.json

# Fix JSON formatting
python -m json.tool filename.json > fixed.json
envswitch import fixed.json

# Check file encoding
file filename.json
```

### `Unsupported file format 'format'`

**Cause:** Trying to export/import in an unsupported format.

**Supported formats:** json, env, yaml

**Solutions:**
```bash
# Use supported format
envswitch export --format json -o configs.json
envswitch export --format env -o configs.env
envswitch export --format yaml -o configs.yaml

# Let EnvSwitch auto-detect from extension
envswitch export -o configs.json  # Auto-detects JSON
envswitch import configs.env       # Auto-detects ENV
```

### `Export directory not writable`

**Cause:** Don't have write permissions to the output directory.

**Solutions:**
```bash
# Check directory permissions
ls -la $(dirname output-file.json)

# Create directory if needed
mkdir -p $(dirname output-file.json)

# Fix permissions
chmod 755 $(dirname output-file.json)

# Export to writable location
envswitch export -o ~/configs.json
```

### `Import validation failed: invalid configuration structure`

**Cause:** Import file doesn't match expected EnvSwitch configuration format.

**Solutions:**
```bash
# Check file structure with dry-run
envswitch import filename.json --dry-run --verbose

# Skip validation if file is trusted
envswitch import filename.json --skip-validation

# Convert from other format
# If it's an ENV file:
envswitch import filename.env  # Auto-detects format
```

### `Configuration conflicts detected during import`

**Cause:** Import file contains configurations that already exist.

**Solutions:**
```bash
# Preview conflicts
envswitch import filename.json --dry-run

# Merge with existing configurations
envswitch import filename.json --merge --backup

# Overwrite existing configurations
envswitch import filename.json --force --backup

# Resolve manually
envswitch delete conflicting-config --force
envswitch import filename.json
```

### `Backup creation failed`

**Cause:** Cannot create backup file before import.

**Solutions:**
```bash
# Check backup directory permissions
ls -la ~/.config/envswitch/backups/

# Create backup directory
mkdir -p ~/.config/envswitch/backups/

# Fix permissions
chmod 755 ~/.config/envswitch/backups/

# Import without backup
envswitch import filename.json  # Without --backup flag
```

## File System Errors

### `Permission denied accessing configuration file`

**Cause:** Insufficient permissions to read/write configuration files.

**Solutions:**
```bash
# Fix file permissions
chmod 600 ~/.config/envswitch/config.json

# Fix directory permissions
chmod 700 ~/.config/envswitch/

# Fix ownership
chown -R $USER:$USER ~/.config/envswitch/
```

### `Configuration directory not found`

**Cause:** EnvSwitch configuration directory doesn't exist.

**Solutions:**
```bash
# Create configuration directory
mkdir -p ~/.config/envswitch/

# Set proper permissions
chmod 700 ~/.config/envswitch/

# Initialize with empty configuration
envswitch list  # Creates initial config file
```

### `Disk space insufficient`

**Cause:** Not enough disk space for operation.

**Solutions:**
```bash
# Check disk space
df -h ~/.config/

# Clean up old backups
find ~/.config/envswitch/backups -name "*.json" -mtime +30 -delete

# Move to location with more space
mv ~/.config/envswitch ~/Documents/envswitch-backup
ln -s ~/Documents/envswitch-backup ~/.config/envswitch
```

### `Configuration file is corrupted`

**Cause:** Configuration file contains invalid data or is corrupted.

**Solutions:**
```bash
# Restore from backup
ls ~/.config/envswitch/backups/
envswitch import ~/.config/envswitch/backups/latest.json --force

# Start with fresh configuration
mv ~/.config/envswitch/config.json ~/.config/envswitch/config.json.corrupted
envswitch list  # Creates new empty config

# Manual repair (advanced)
python -m json.tool ~/.config/envswitch/config.json
```

## Validation Errors

### `Invalid environment variable name 'name'`

**Cause:** Environment variable name contains invalid characters.

**Valid format:** Letters, numbers, underscores only. Must start with letter or underscore.

**Solutions:**
```bash
# ✅ Valid variable names
envswitch set config -e VALID_NAME=value
envswitch set config -e _PRIVATE_VAR=value
envswitch set config -e VAR123=value

# ❌ Invalid names - fix these
envswitch set config -e "invalid-name=value"  # Use: INVALID_NAME
envswitch set config -e "123invalid=value"    # Use: VAR_123_INVALID
envswitch set config -e "var.name=value"      # Use: VAR_NAME
```

### `Environment variable value too long`

**Cause:** Environment variable value exceeds maximum length.

**Solutions:**
```bash
# Shorten the value
envswitch set config -e LONG_VAR="shorter value"

# Use file reference instead
echo "very long value" > /tmp/longvalue.txt
envswitch set config -e LONG_VAR="$(cat /tmp/longvalue.txt)"

# Split into multiple variables
envswitch set config -e PART1="first part" -e PART2="second part"
```

### `Too many environment variables in configuration`

**Cause:** Configuration exceeds maximum number of variables (usually 1000).

**Solutions:**
```bash
# Split into multiple configurations
envswitch set config-part1 -e VAR1=value1 -e VAR2=value2
envswitch set config-part2 -e VAR3=value3 -e VAR4=value4

# Remove unused variables
envswitch edit config  # Use interactive editor to remove variables
```

### `Duplicate environment variable 'name'`

**Cause:** Same variable name specified multiple times.

**Solutions:**
```bash
# Remove duplicate from command
envswitch set config -e VAR=value1  # Don't repeat -e VAR=value2

# Use replace mode to overwrite
envswitch set config -e VAR=new_value --replace
```

## Shell Integration Errors

### `Shell detection failed`

**Cause:** Cannot determine current shell type.

**Solutions:**
```bash
# Check current shell
echo $SHELL

# Set SHELL environment variable
export SHELL=/bin/zsh  # or /bin/bash, /usr/local/bin/fish

# Force shell type
envswitch use config --shell zsh
```

### `Unsupported shell 'shell_name'`

**Cause:** Shell type is not supported by EnvSwitch.

**Supported shells:** bash, zsh, fish, sh

**Solutions:**
```bash
# Use supported shell
export SHELL=/bin/bash
envswitch use config

# Generate commands manually
envswitch use config --dry-run > temp_vars.sh
source temp_vars.sh  # For bash/zsh
```

### `Shell command generation failed`

**Cause:** Error generating shell-specific commands.

**Solutions:**
```bash
# Use dry-run to see generated commands
envswitch use config --dry-run

# Try different shell
SHELL=/bin/bash envswitch use config

# Manual variable setting
envswitch show config  # Copy variables manually
export VAR1=value1
export VAR2=value2
```

## Interactive Editor Errors

### `Interactive editor initialization failed`

**Cause:** Cannot start interactive editor.

**Solutions:**
```bash
# Check terminal capabilities
echo $TERM

# Use non-interactive mode
envswitch set config -e KEY=value

# Try different terminal
# Run in different terminal emulator
```

### `Input validation failed in editor`

**Cause:** Invalid input provided in interactive editor.

**Solutions:**
```bash
# Follow the prompts carefully
# Variable names: Use only letters, numbers, underscores
# Values: Avoid special characters that might cause issues

# Exit and use command line
# Press 'q' to quit editor
envswitch set config -e VALID_NAME=value
```

### `Editor session corrupted`

**Cause:** Interactive editor session became corrupted.

**Solutions:**
```bash
# Exit editor (press 'q')
# Try again
envswitch edit config

# Use command line instead
envswitch set config -e KEY=value

# Check terminal settings
stty sane
```

## System Errors

### `Command execution timeout`

**Cause:** Operation took too long to complete.

**Solutions:**
```bash
# Use specific operations instead of bulk operations
envswitch export -c "config1,config2" -o subset.json

# Check system resources
top
df -h

# Restart and try again
```

### `Memory allocation failed`

**Cause:** Insufficient memory for operation.

**Solutions:**
```bash
# Close other applications
# Check memory usage
free -h

# Use smaller operations
envswitch export -c "single-config" -o small.json

# Restart system if needed
```

### `Network connection failed`

**Cause:** Network-related operation failed (if applicable).

**Note:** EnvSwitch is primarily local, but some operations might require network access.

**Solutions:**
```bash
# Check network connectivity
ping google.com

# Use offline mode if available
# Retry operation

# Check firewall settings
```

## Error Code Reference

EnvSwitch uses standard exit codes:

- `0`: Success
- `1`: General error
- `2`: Misuse of shell command
- `64`: Command line usage error
- `65`: Data format error
- `66`: Cannot open input
- `67`: Addressee unknown
- `68`: Host name unknown
- `69`: Service unavailable
- `70`: Internal software error
- `71`: System error
- `72`: Critical OS file missing
- `73`: Can't create output file
- `74`: Input/output error
- `75`: Temporary failure
- `76`: Remote error in protocol
- `77`: Permission denied
- `78`: Configuration error

## Getting Detailed Error Information

### Verbose Mode

Always use verbose mode when troubleshooting:

```bash
envswitch command --verbose
```

### Error Context

Most errors include context information:
- What operation was being performed
- What file or configuration was involved
- Suggestions for resolution

### Debug Information

For complex issues, gather debug information:

```bash
# System information
uname -a
echo $SHELL

# EnvSwitch version
envswitch --version

# Configuration status
envswitch list --verbose

# File permissions
ls -la ~/.config/envswitch/

# Disk space
df -h ~/.config/
```

## Reporting Bugs

When reporting errors, include:

1. **Error message** (exact text)
2. **Command used** (full command line)
3. **System information** (OS, shell, EnvSwitch version)
4. **Steps to reproduce**
5. **Expected vs actual behavior**
6. **Verbose output** (if applicable)

Example bug report:
```
Error: Configuration 'test' not found

Command: envswitch use test --verbose

System: macOS 12.6, zsh 5.8.1, EnvSwitch 0.1.0

Steps:
1. envswitch set test -e KEY=value
2. envswitch list (shows 'test' configuration)
3. envswitch use test (fails with error)

Expected: Should switch to 'test' configuration
Actual: Error message about configuration not found

Verbose output:
[verbose output here]
```