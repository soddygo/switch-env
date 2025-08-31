# EnvSwitch Best Practices

This document outlines recommended practices for using EnvSwitch effectively and securely.

## Table of Contents

- [Configuration Management](#configuration-management)
- [Security Best Practices](#security-best-practices)
- [Import/Export Guidelines](#importexport-guidelines)
- [Shell Integration](#shell-integration)
- [Backup and Recovery](#backup-and-recovery)
- [Performance Optimization](#performance-optimization)
- [Team Collaboration](#team-collaboration)
- [Troubleshooting](#troubleshooting)

## Configuration Management

### Naming Conventions

**Use descriptive, consistent names:**

```bash
# ✅ Good - Clear and descriptive
envswitch set myapp-prod-us-east
envswitch set myapp-staging-eu-west
envswitch set myapp-dev-local

# ❌ Avoid - Ambiguous names
envswitch set prod
envswitch set config1
envswitch set temp
```

**Follow a naming pattern:**

```bash
# Pattern: {project}-{environment}-{region/purpose}
envswitch set ecommerce-prod-us-east
envswitch set ecommerce-staging-eu-west
envswitch set ecommerce-dev-local

# Pattern: {service}-{environment}
envswitch set api-prod
envswitch set api-staging
envswitch set api-dev
```

### Configuration Organization

**Group related configurations:**

```bash
# AI/ML configurations
envswitch set ai-openai-gpt4
envswitch set ai-claude-sonnet
envswitch set ai-deepseek

# Database configurations
envswitch set db-prod-primary
envswitch set db-prod-replica
envswitch set db-staging
```

**Use descriptions effectively:**

```bash
# ✅ Good - Informative descriptions
envswitch set prod-api \
  -e API_URL=https://api.example.com \
  -d "Production API configuration - US East region"

# ✅ Good - Include important notes
envswitch set staging-db \
  -e DATABASE_URL=postgresql://staging:5432/app \
  -d "Staging database - shared with QA team, reset weekly"
```

### Variable Management

**Use consistent variable naming:**

```bash
# ✅ Good - Consistent naming
envswitch set myapp \
  -e DATABASE_URL=postgresql://... \
  -e REDIS_URL=redis://... \
  -e API_BASE_URL=https://...

# ❌ Avoid - Inconsistent naming
envswitch set myapp \
  -e DB_CONNECTION=postgresql://... \
  -e redis_host=redis://... \
  -e api-endpoint=https://...
```

**Group related variables:**

```bash
# Database variables together
envswitch set myapp \
  -e DATABASE_URL=postgresql://localhost:5432/myapp \
  -e DATABASE_POOL_SIZE=10 \
  -e DATABASE_TIMEOUT=30 \
  -e REDIS_URL=redis://localhost:6379 \
  -e REDIS_POOL_SIZE=5
```

## Security Best Practices

### Sensitive Data Handling

**Never store secrets in plain text exports:**

```bash
# ✅ Good - Export non-sensitive configs only
envswitch export -c "dev,staging" -o safe-configs.json

# ❌ Avoid - Exporting production secrets
envswitch export -o all-configs.json  # May include prod secrets
```

**Use environment-specific secret management:**

```bash
# ✅ Good - Reference external secret management
envswitch set prod \
  -e SECRET_KEY_PATH=/vault/secrets/app-key \
  -e DATABASE_URL_SECRET=vault:database-url \
  -d "Production - secrets managed by Vault"

# ❌ Avoid - Hardcoded secrets
envswitch set prod \
  -e SECRET_KEY=actual-secret-value \
  -e DATABASE_PASSWORD=plaintext-password
```

### Access Control

**Protect configuration files:**

```bash
# Set appropriate permissions
chmod 600 ~/.config/envswitch/config.json
chmod 700 ~/.config/envswitch/

# Regular permission audit
ls -la ~/.config/envswitch/
```

**Use separate configurations for different security levels:**

```bash
# Separate sensitive and non-sensitive configs
envswitch set app-config-public \
  -e API_URL=https://api.example.com \
  -e TIMEOUT=30 \
  -d "Public configuration - safe to share"

envswitch set app-config-secrets \
  -e API_KEY=secret-key \
  -e DATABASE_PASSWORD=secret-pass \
  -d "SENSITIVE: Contains secrets"
```

### Audit and Monitoring

**Regular configuration audits:**

```bash
# List all configurations with metadata
envswitch list --verbose

# Check for potentially sensitive configurations
envswitch list | grep -E "(prod|secret|key|token)"

# Review configuration descriptions
envswitch show myconfig
```

**Document sensitive configurations:**

```bash
# ✅ Good - Clear sensitivity marking
envswitch set prod-secrets \
  -e SECRET_KEY=... \
  -d "⚠️ SENSITIVE: Production secrets - restricted access"

# ✅ Good - Include access information
envswitch set db-prod \
  -e DATABASE_URL=... \
  -d "Production DB - Access: DevOps team only"
```

## Import/Export Guidelines

### Export Best Practices

**Always use metadata for important exports:**

```bash
# ✅ Good - Include metadata and pretty formatting
envswitch export -o backup.json --metadata --pretty

# ✅ Good - Specific exports with context
envswitch export -c "dev,staging" -o dev-configs.json --metadata
```

**Use appropriate formats:**

```bash
# JSON for full metadata and complex structures
envswitch export -o configs.json --format json --metadata

# ENV for simple variable sharing
envswitch export -o simple.env --format env

# YAML for human-readable configs
envswitch export -o readable.yaml --format yaml
```

### Import Best Practices

**Always preview imports first:**

```bash
# ✅ Good - Preview before importing
envswitch import configs.json --dry-run
envswitch import configs.json --backup --verbose
```

**Use appropriate conflict resolution:**

```bash
# For updates to existing configs
envswitch import configs.json --merge --backup

# For complete replacement
envswitch import configs.json --force --backup

# For new environments
envswitch import configs.json  # Default behavior
```

**Validate imports:**

```bash
# After importing, verify configurations
envswitch list --verbose
envswitch status

# Test critical configurations
envswitch use prod --dry-run
```

## Shell Integration

### Shell-Specific Setup

**Zsh configuration (~/.zshrc):**

```bash
# EnvSwitch aliases
alias es='envswitch'
alias esl='envswitch list'
alias ess='envswitch status'
alias esu='envswitch use'

# Quick switching functions
switch-to() {
  if [[ -z "$1" ]]; then
    echo "Usage: switch-to <config-name>"
    envswitch list
    return 1
  fi
  eval "$(envswitch use $1)"
}

# Auto-completion (if available)
# eval "$(envswitch completion zsh)"
```

**Fish configuration (~/.config/fish/config.fish):**

```fish
# EnvSwitch aliases
alias es='envswitch'
alias esl='envswitch list'
alias ess='envswitch status'

# Quick switching function
function switch-to
    if test (count $argv) -eq 0
        echo "Usage: switch-to <config-name>"
        envswitch list
        return 1
    end
    eval (envswitch use $argv[1])
end
```

**Bash configuration (~/.bashrc):**

```bash
# EnvSwitch aliases
alias es='envswitch'
alias esl='envswitch list'
alias ess='envswitch status'

# Quick switching function
switch-to() {
  if [[ -z "$1" ]]; then
    echo "Usage: switch-to <config-name>"
    envswitch list
    return 1
  fi
  eval "$(envswitch use $1)"
}
```

### Environment Validation

**Create validation functions:**

```bash
# Validate required environment variables
validate-env() {
  local required_vars=("DATABASE_URL" "API_KEY" "REDIS_URL")
  local missing_vars=()
  
  for var in "${required_vars[@]}"; do
    if [[ -z "${!var}" ]]; then
      missing_vars+=("$var")
    fi
  done
  
  if [[ ${#missing_vars[@]} -gt 0 ]]; then
    echo "❌ Missing required environment variables:"
    printf '  %s\n' "${missing_vars[@]}"
    return 1
  else
    echo "✅ All required environment variables are set"
    return 0
  fi
}

# Use after switching configurations
switch-and-validate() {
  eval "$(envswitch use $1)" && validate-env
}
```

## Backup and Recovery

### Automated Backups

**Daily backup script:**

```bash
#!/bin/bash
# ~/.local/bin/envswitch-backup.sh

BACKUP_DIR="$HOME/.config/envswitch/backups"
DATE=$(date +%Y%m%d)
TIMESTAMP=$(date +%Y%m%d-%H%M%S)

# Create backup directory
mkdir -p "$BACKUP_DIR"

# Create daily backup
envswitch export -o "$BACKUP_DIR/daily-backup-$DATE.json" --metadata --pretty

# Create timestamped backup for important changes
if [[ "$1" == "--important" ]]; then
  envswitch export -o "$BACKUP_DIR/important-backup-$TIMESTAMP.json" --metadata --pretty
  echo "Important backup created: important-backup-$TIMESTAMP.json"
fi

# Clean up old backups (keep last 30 days)
find "$BACKUP_DIR" -name "daily-backup-*.json" -mtime +30 -delete

echo "Daily backup completed: daily-backup-$DATE.json"
```

**Weekly backup with rotation:**

```bash
#!/bin/bash
# Weekly backup script

BACKUP_DIR="$HOME/.config/envswitch/backups/weekly"
WEEK=$(date +%Y-W%U)

mkdir -p "$BACKUP_DIR"

# Create weekly backup
envswitch export -o "$BACKUP_DIR/weekly-backup-$WEEK.json" --metadata --pretty

# Keep only last 12 weeks
find "$BACKUP_DIR" -name "weekly-backup-*.json" -mtime +84 -delete

echo "Weekly backup completed: weekly-backup-$WEEK.json"
```

### Recovery Procedures

**Standard recovery process:**

```bash
# 1. Assess the situation
envswitch list --verbose
envswitch status

# 2. Create emergency backup of current state
envswitch export -o "emergency-backup-$(date +%Y%m%d-%H%M%S).json" --metadata

# 3. Restore from backup
envswitch import backup-file.json --backup --verbose

# 4. Verify recovery
envswitch list --verbose
envswitch status
```

**Selective recovery:**

```bash
# Preview what would be restored
envswitch import backup.json --dry-run

# Restore only specific configurations
# (Extract specific configs from backup first)
envswitch import partial-backup.json --merge --backup
```

## Performance Optimization

### Large Configuration Management

**Optimize exports:**

```bash
# ✅ Good - Export only needed configurations
envswitch export -c "config1,config2" -o subset.json

# ✅ Good - Use compact format for large exports
envswitch export -o configs.json  # Without --pretty for speed

# ❌ Avoid - Exporting everything with pretty formatting
envswitch export -o all.json --pretty --metadata  # Slow for many configs
```

**Batch operations:**

```bash
# Process configurations in batches
configs=($(envswitch list))
batch_size=10

for ((i=0; i<${#configs[@]}; i+=batch_size)); do
  batch=("${configs[@]:i:batch_size}")
  echo "Processing batch: ${batch[*]}"
  # Process batch...
done
```

### Storage Optimization

**Regular cleanup:**

```bash
# Remove unused configurations
envswitch list | while read config; do
  echo "Last used: $config"
  # Add logic to check usage and remove old configs
done

# Clean up backup files
find ~/.config/envswitch/backups -name "*.json" -mtime +90 -delete
```

## Team Collaboration

### Shared Configurations

**Create team configuration templates:**

```bash
# Template for new team members
envswitch export -c "dev-template,staging-template" -o team-template.json --metadata

# Document the template
echo "Team Configuration Template" > team-template.md
echo "Import with: envswitch import team-template.json --merge" >> team-template.md
```

**Environment standardization:**

```bash
# Standard development environment
envswitch set dev-standard \
  -e NODE_ENV=development \
  -e DEBUG=true \
  -e LOG_LEVEL=debug \
  -e DATABASE_URL=postgresql://localhost:5432/app_dev \
  -d "Standard development environment for team"

# Export for team sharing
envswitch export -c "dev-standard" -o dev-standard.json --metadata
```

### Documentation Standards

**Configuration documentation:**

```bash
# ✅ Good - Comprehensive description
envswitch set api-prod \
  -e API_URL=https://api.example.com \
  -e TIMEOUT=30 \
  -d "Production API config - US East, load balanced, 99.9% SLA"

# ✅ Good - Include contact information
envswitch set db-prod \
  -e DATABASE_URL=postgresql://... \
  -d "Production DB - Contact: devops@company.com for access"
```

**Change management:**

```bash
# Before making changes, create backup
envswitch export -o "before-changes-$(date +%Y%m%d).json" --metadata

# Document changes
envswitch edit prod-config
# Add note in description about what changed and why
```

## Troubleshooting

### Common Issues and Solutions

**Configuration not found:**

```bash
# Check for typos
envswitch list | grep -i "partial-name"

# Use tab completion if available
envswitch use <TAB>

# Check similar names
envswitch list | sort
```

**Import/export failures:**

```bash
# Validate file format
file config-file.json
python -m json.tool config-file.json > /dev/null

# Check file permissions
ls -la config-file.json

# Use verbose mode for debugging
envswitch import config-file.json --dry-run --verbose
```

**Shell integration issues:**

```bash
# Check shell type
echo $SHELL

# Test command generation
envswitch use config --dry-run

# Check for shell-specific syntax
if [[ "$SHELL" == *"fish"* ]]; then
  eval (envswitch use config)
else
  eval "$(envswitch use config)"
fi
```

### Debugging Techniques

**Enable verbose output:**

```bash
# For all operations
envswitch --verbose list
envswitch --verbose use config
envswitch --verbose import file.json
```

**Check configuration integrity:**

```bash
# Verify configuration file
cat ~/.config/envswitch/config.json | python -m json.tool

# Check for corruption
envswitch list --verbose
```

**Performance debugging:**

```bash
# Time operations
time envswitch export -o large-export.json
time envswitch import large-export.json --dry-run

# Check file sizes
ls -lh ~/.config/envswitch/
du -h ~/.config/envswitch/backups/
```

### Recovery from Common Problems

**Corrupted configuration file:**

```bash
# Backup current state
cp ~/.config/envswitch/config.json ~/.config/envswitch/config.json.corrupted

# Restore from backup
envswitch import latest-backup.json --force

# Or start fresh if no backup
rm ~/.config/envswitch/config.json
envswitch list  # Will create new empty config
```

**Lost configurations:**

```bash
# Check for backup files
ls -la ~/.config/envswitch/backups/

# Restore from most recent backup
envswitch import ~/.config/envswitch/backups/latest-backup.json --merge

# Check system backups (Time Machine, etc.)
```

**Permission issues:**

```bash
# Fix permissions
chmod 700 ~/.config/envswitch/
chmod 600 ~/.config/envswitch/config.json

# Check ownership
ls -la ~/.config/envswitch/
chown -R $USER:$USER ~/.config/envswitch/
```