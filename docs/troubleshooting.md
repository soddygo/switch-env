# EnvSwitch Troubleshooting Guide

This guide helps you diagnose and resolve common issues with EnvSwitch.

## Table of Contents

- [Installation Issues](#installation-issues)
- [Configuration Problems](#configuration-problems)
- [Import/Export Issues](#importexport-issues)
- [Shell Integration Problems](#shell-integration-problems)
- [Performance Issues](#performance-issues)
- [File System Issues](#file-system-issues)
- [Error Messages](#error-messages)
- [Recovery Procedures](#recovery-procedures)

## Installation Issues

### Command Not Found

**Problem:** `envswitch: command not found`

**Solutions:**

1. **Check if EnvSwitch is installed:**
   ```bash
   which envswitch
   ls -la /usr/local/bin/envswitch
   ```

2. **Add to PATH if installed elsewhere:**
   ```bash
   # Add to your shell config (~/.zshrc, ~/.bashrc, etc.)
   export PATH="$PATH:/path/to/envswitch"
   ```

3. **Reinstall from source:**
   ```bash
   git clone https://github.com/soddygo/envswitch
   cd envswitch
   cargo build --release
   sudo cp target/release/envswitch /usr/local/bin/
   ```

### Permission Denied

**Problem:** `Permission denied` when running EnvSwitch

**Solutions:**

1. **Make executable:**
   ```bash
   chmod +x /usr/local/bin/envswitch
   ```

2. **Check ownership:**
   ```bash
   ls -la /usr/local/bin/envswitch
   sudo chown $USER:$USER /usr/local/bin/envswitch
   ```

## Configuration Problems

### Configuration Not Found

**Problem:** `Configuration 'myconfig' not found`

**Diagnosis:**
```bash
# List all configurations
envswitch list

# Check for similar names
envswitch list | grep -i "partial-name"
```

**Solutions:**

1. **Check spelling:**
   ```bash
   # EnvSwitch may suggest similar names
   envswitch use myconfg  # Will suggest "myconfig"
   ```

2. **Create the configuration:**
   ```bash
   envswitch set myconfig -e KEY=value
   ```

3. **Import from backup:**
   ```bash
   envswitch import backup.json --merge
   ```

### Configuration File Corruption

**Problem:** `JSON parsing error` or corrupted configuration file

**Diagnosis:**
```bash
# Check configuration file
cat ~/.config/envswitch/config.json | python -m json.tool

# Check file size and permissions
ls -la ~/.config/envswitch/config.json
```

**Solutions:**

1. **Restore from backup:**
   ```bash
   # Find backup files
   ls -la ~/.config/envswitch/backups/
   
   # Restore from most recent backup
   envswitch import ~/.config/envswitch/backups/latest.json --force
   ```

2. **Manual repair:**
   ```bash
   # Backup corrupted file
   cp ~/.config/envswitch/config.json ~/.config/envswitch/config.json.corrupted
   
   # Try to fix JSON manually or start fresh
   echo '{"configs": {}, "active_config": null}' > ~/.config/envswitch/config.json
   ```

3. **Start fresh:**
   ```bash
   # Remove corrupted file (will create new empty one)
   rm ~/.config/envswitch/config.json
   envswitch list  # Creates new empty configuration
   ```

### Invalid Configuration Names

**Problem:** `Invalid configuration name` error

**Diagnosis:**
```bash
# Check what characters are causing issues
envswitch set "my config" -e KEY=value  # Spaces not allowed
envswitch set "my-config!" -e KEY=value  # Special chars not allowed
```

**Solutions:**

1. **Use valid characters only:**
   ```bash
   # ✅ Valid names
   envswitch set my-config -e KEY=value
   envswitch set my_config -e KEY=value
   envswitch set myconfig123 -e KEY=value
   
   # ❌ Invalid names
   envswitch set "my config"    # No spaces
   envswitch set "my.config"    # No dots
   envswitch set "my@config"    # No special chars
   ```

2. **Rename existing configurations:**
   ```bash
   # Export, modify, and re-import
   envswitch export -c "invalid-name" -o temp.json
   # Edit temp.json to change the name
   envswitch delete "invalid-name" --force
   envswitch import temp.json
   ```

## Import/Export Issues

### File Format Errors

**Problem:** `Invalid JSON format` or `Format validation failed`

**Diagnosis:**
```bash
# Check file format
file myconfig.json
head -n 5 myconfig.json

# Validate JSON
python -m json.tool myconfig.json > /dev/null
```

**Solutions:**

1. **Fix JSON syntax:**
   ```bash
   # Use a JSON validator/formatter
   python -m json.tool myconfig.json > fixed.json
   envswitch import fixed.json
   ```

2. **Convert from other formats:**
   ```bash
   # If it's actually an ENV file
   envswitch import myconfig.env  # Auto-detects format
   ```

3. **Use dry-run to preview:**
   ```bash
   envswitch import myconfig.json --dry-run --verbose
   ```

### Import Conflicts

**Problem:** `Configuration conflicts found` during import

**Diagnosis:**
```bash
# Preview what would be imported
envswitch import configs.json --dry-run
```

**Solutions:**

1. **Use merge mode:**
   ```bash
   envswitch import configs.json --merge --backup
   ```

2. **Use force mode:**
   ```bash
   envswitch import configs.json --force --backup
   ```

3. **Resolve manually:**
   ```bash
   # Import with dry-run to see conflicts
   envswitch import configs.json --dry-run
   
   # Delete conflicting configs first
   envswitch delete conflicting-config --force
   
   # Then import
   envswitch import configs.json
   ```

### Export Failures

**Problem:** Export command fails or produces empty files

**Diagnosis:**
```bash
# Check if configurations exist
envswitch list

# Check output directory permissions
ls -la $(dirname output-file.json)

# Try with verbose output
envswitch export -o test.json --verbose
```

**Solutions:**

1. **Check permissions:**
   ```bash
   # Ensure output directory is writable
   mkdir -p $(dirname output-file.json)
   touch output-file.json && rm output-file.json
   ```

2. **Use different output location:**
   ```bash
   # Export to home directory
   envswitch export -o ~/configs.json
   ```

3. **Export specific configurations:**
   ```bash
   # If exporting all fails, try specific ones
   envswitch export -c "config1,config2" -o partial.json
   ```

## Shell Integration Problems

### Commands Not Working

**Problem:** `eval "$(envswitch use config)"` doesn't set variables

**Diagnosis:**
```bash
# Test command generation
envswitch use myconfig --dry-run

# Check shell type
echo $SHELL

# Test without eval
envswitch use myconfig
```

**Solutions:**

1. **Use correct shell syntax:**
   ```bash
   # For bash/zsh
   eval "$(envswitch use myconfig)"
   
   # For fish
   eval (envswitch use myconfig)
   ```

2. **Check shell detection:**
   ```bash
   # Force shell type if auto-detection fails
   SHELL=/bin/zsh envswitch use myconfig
   ```

3. **Manual variable setting:**
   ```bash
   # If eval doesn't work, set manually
   envswitch use myconfig --dry-run > temp_vars.sh
   source temp_vars.sh
   rm temp_vars.sh
   ```

### Shell Detection Issues

**Problem:** Wrong shell commands generated

**Diagnosis:**
```bash
# Check current shell
echo $SHELL
ps -p $$

# Test with different shells
envswitch use myconfig --dry-run
```

**Solutions:**

1. **Set SHELL environment variable:**
   ```bash
   export SHELL=/usr/local/bin/fish
   envswitch use myconfig
   ```

2. **Use shell-specific commands:**
   ```bash
   # Force specific shell format
   envswitch init --shell zsh
   envswitch init --shell fish
   envswitch init --shell bash
   ```

### Alias and Function Issues

**Problem:** Shell aliases or functions not working

**Diagnosis:**
```bash
# Check if aliases are defined
alias | grep envswitch

# Check function definitions
type switch-to
```

**Solutions:**

1. **Reload shell configuration:**
   ```bash
   source ~/.zshrc    # For zsh
   source ~/.bashrc   # For bash
   exec fish          # For fish
   ```

2. **Add aliases manually:**
   ```bash
   # Add to your shell config file
   alias es='envswitch'
   alias esl='envswitch list'
   alias esu='envswitch use'
   ```

## Performance Issues

### Slow Operations

**Problem:** EnvSwitch commands are slow

**Diagnosis:**
```bash
# Time operations
time envswitch list
time envswitch export -o test.json

# Check configuration file size
ls -lh ~/.config/envswitch/config.json

# Count configurations
envswitch list | wc -l
```

**Solutions:**

1. **Clean up old configurations:**
   ```bash
   # Remove unused configurations
   envswitch delete old-config --force
   
   # Export and re-import to compact file
   envswitch export -o compact.json
   mv ~/.config/envswitch/config.json ~/.config/envswitch/config.json.backup
   envswitch import compact.json --force
   ```

2. **Use specific operations:**
   ```bash
   # Export only needed configurations
   envswitch export -c "config1,config2" -o subset.json
   
   # Use non-pretty format for speed
   envswitch export -o configs.json  # Without --pretty
   ```

3. **Check disk space:**
   ```bash
   df -h ~/.config/
   ```

### Memory Issues

**Problem:** High memory usage or out of memory errors

**Diagnosis:**
```bash
# Check configuration file size
du -h ~/.config/envswitch/

# Monitor memory usage
top -p $(pgrep envswitch)
```

**Solutions:**

1. **Reduce configuration size:**
   ```bash
   # Split large configurations
   envswitch export -c "large-config" -o large.json
   # Edit large.json to split into smaller configs
   envswitch import smaller-configs.json
   ```

2. **Clean up backups:**
   ```bash
   # Remove old backup files
   find ~/.config/envswitch/backups -name "*.json" -mtime +30 -delete
   ```

## File System Issues

### Permission Denied

**Problem:** `Permission denied` accessing configuration files

**Diagnosis:**
```bash
# Check permissions
ls -la ~/.config/envswitch/
ls -la ~/.config/envswitch/config.json

# Check ownership
stat ~/.config/envswitch/config.json
```

**Solutions:**

1. **Fix permissions:**
   ```bash
   chmod 700 ~/.config/envswitch/
   chmod 600 ~/.config/envswitch/config.json
   ```

2. **Fix ownership:**
   ```bash
   chown -R $USER:$USER ~/.config/envswitch/
   ```

3. **Recreate directory:**
   ```bash
   # Backup first
   cp ~/.config/envswitch/config.json ~/envswitch-backup.json
   
   # Remove and recreate
   rm -rf ~/.config/envswitch/
   mkdir -p ~/.config/envswitch/
   
   # Restore
   envswitch import ~/envswitch-backup.json --force
   ```

### Disk Space Issues

**Problem:** `No space left on device` errors

**Diagnosis:**
```bash
# Check disk space
df -h ~/.config/
du -h ~/.config/envswitch/
```

**Solutions:**

1. **Clean up backup files:**
   ```bash
   # Remove old backups
   find ~/.config/envswitch/backups -name "*.json" -mtime +7 -delete
   
   # Compress old backups
   gzip ~/.config/envswitch/backups/*.json
   ```

2. **Move to different location:**
   ```bash
   # Move to location with more space
   mv ~/.config/envswitch ~/Documents/envswitch-backup
   ln -s ~/Documents/envswitch-backup ~/.config/envswitch
   ```

## Error Messages

### Common Error Messages and Solutions

#### "Configuration file is corrupted"

```bash
# Restore from backup
envswitch import ~/.config/envswitch/backups/latest.json --force

# Or start fresh
rm ~/.config/envswitch/config.json
envswitch list
```

#### "Invalid environment variable name"

```bash
# Check variable name format
envswitch set myconfig -e "VALID_NAME=value"  # ✅ Good
envswitch set myconfig -e "invalid-name=value"  # ❌ Bad (hyphen)
```

#### "Import file not found"

```bash
# Check file path
ls -la import-file.json

# Use absolute path
envswitch import /full/path/to/import-file.json
```

#### "Export directory not writable"

```bash
# Check directory permissions
ls -la $(dirname output-file.json)

# Create directory if needed
mkdir -p $(dirname output-file.json)
```

#### "Shell detection failed"

```bash
# Set SHELL environment variable
export SHELL=/usr/local/bin/zsh
envswitch use myconfig
```

## Recovery Procedures

### Complete System Recovery

If EnvSwitch is completely broken:

1. **Backup current state:**
   ```bash
   cp -r ~/.config/envswitch ~/.config/envswitch.backup
   ```

2. **Reinstall EnvSwitch:**
   ```bash
   # Remove old installation
   rm /usr/local/bin/envswitch
   
   # Reinstall
   cargo install --git https://github.com/soddygo/envswitch --force
   ```

3. **Restore configurations:**
   ```bash
   # Try to restore from backup
   envswitch import ~/.config/envswitch.backup/config.json --force
   
   # Or restore from external backup
   envswitch import ~/backups/envswitch-backup.json --force
   ```

### Partial Recovery

If only some configurations are lost:

1. **Check what's available:**
   ```bash
   envswitch list --verbose
   ```

2. **Restore from backup:**
   ```bash
   # Merge with existing configurations
   envswitch import backup.json --merge --backup
   ```

3. **Recreate missing configurations:**
   ```bash
   # Recreate from documentation or memory
   envswitch set missing-config -e KEY=value -d "Recreated configuration"
   ```

### Emergency Procedures

If you need to quickly restore a working environment:

1. **Create minimal working configuration:**
   ```bash
   envswitch set emergency \
     -e PATH=$PATH \
     -e HOME=$HOME \
     -e USER=$USER \
     -d "Emergency configuration"
   ```

2. **Use system environment as base:**
   ```bash
   # Export current environment to file
   env > current-env.txt
   
   # Convert to EnvSwitch format (manual process)
   # Then import
   ```

3. **Quick restore from known good state:**
   ```bash
   # If you have a known good backup
   rm ~/.config/envswitch/config.json
   envswitch import known-good-backup.json --force
   ```

## Getting Help

### Diagnostic Information

When reporting issues, include:

```bash
# System information
uname -a
echo $SHELL

# EnvSwitch version
envswitch --version

# Configuration status
envswitch list --verbose
ls -la ~/.config/envswitch/

# Error output with verbose mode
envswitch command --verbose 2>&1
```

### Log Files

EnvSwitch doesn't create log files by default, but you can capture output:

```bash
# Capture all output
envswitch command --verbose > envswitch.log 2>&1

# Monitor in real-time
envswitch command --verbose 2>&1 | tee envswitch.log
```

### Community Support

- Check the GitHub issues for similar problems
- Include diagnostic information when reporting bugs
- Use verbose mode output to help with troubleshooting
- Provide minimal reproduction steps