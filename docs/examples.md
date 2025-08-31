# EnvSwitch Usage Examples

This document provides comprehensive examples of using EnvSwitch for various scenarios.

## Table of Contents

- [Basic Configuration Management](#basic-configuration-management)
- [Import/Export Operations](#importexport-operations)
- [Interactive Configuration Editing](#interactive-configuration-editing)
- [Advanced Workflows](#advanced-workflows)
- [AI Model Configuration Examples](#ai-model-configuration-examples)
- [Development Environment Management](#development-environment-management)
- [Backup and Recovery](#backup-and-recovery)
- [Troubleshooting Examples](#troubleshooting-examples)

## Basic Configuration Management

### Creating Configurations

```bash
# Create a simple configuration
envswitch set myapp \
  -e DATABASE_URL=postgresql://localhost:5432/myapp \
  -e REDIS_URL=redis://localhost:6379 \
  -e DEBUG=true

# Create with description
envswitch set production \
  -e DATABASE_URL=postgresql://prod-server:5432/myapp \
  -e REDIS_URL=redis://prod-redis:6379 \
  -e DEBUG=false \
  -d "Production environment configuration"

# Create from environment file
echo "API_KEY=secret123" > .env
echo "API_URL=https://api.example.com" >> .env
envswitch set api-config -f .env

# Interactive creation
envswitch set interactive-config --interactive
```

### Using Configurations

```bash
# Switch to a configuration
eval "$(envswitch use myapp)"

# Check what would be set (dry run)
envswitch use myapp --dry-run

# Switch with verbose output
envswitch use myapp --verbose

# For fish shell
eval (envswitch use myapp)
```

### Listing and Viewing Configurations

```bash
# List all configurations
envswitch list

# List with detailed information
envswitch list --verbose

# List in table format
envswitch list --table

# Show only active configuration
envswitch list --active

# Show specific configuration details
envswitch show myapp
```

### Deleting Configurations

```bash
# Delete with confirmation prompt
envswitch delete myapp

# Force delete without confirmation
envswitch delete myapp --force

# Delete with verbose output
envswitch delete myapp --verbose
```

## Import/Export Operations

### Basic Export

```bash
# Export all configurations to JSON
envswitch export -o all-configs.json

# Export with pretty formatting
envswitch export -o all-configs.json --pretty

# Export with metadata (timestamps, descriptions)
envswitch export -o all-configs.json --metadata --pretty

# Export specific configurations
envswitch export -c prod,staging,dev -o environments.json
```

### Format-Specific Exports

```bash
# Export as environment file
envswitch export -o configs.env --format env

# Export as YAML
envswitch export -o configs.yaml --format yaml

# Export specific config as ENV format
envswitch export -c production -o prod.env --format env
```

### Basic Import

```bash
# Import from JSON file
envswitch import configs.json

# Import with backup (recommended)
envswitch import configs.json --backup

# Preview import without making changes
envswitch import configs.json --dry-run
```

### Conflict Resolution

```bash
# Force overwrite existing configurations
envswitch import configs.json --force

# Merge with existing configurations
envswitch import configs.json --merge

# Import with verbose output to see what's happening
envswitch import configs.json --verbose --backup
```

### Cross-Format Import

```bash
# Import from ENV file (auto-detected)
envswitch import production.env

# Import from YAML file
envswitch import configs.yaml

# Import with format validation
envswitch import configs.json --verbose
```

## Interactive Configuration Editing

### Basic Editing

```bash
# Edit existing configuration
envswitch edit myapp

# Edit non-existent configuration (will offer to create)
envswitch edit newconfig
```

### Interactive Editor Commands

When you run `envswitch edit <config>`, you'll see a menu like this:

```
📝 Editing configuration: myapp
   Description: My application configuration

📋 Current variables:
   1. DATABASE_URL = postgresql://localhost:5432/myapp
   2. REDIS_URL = redis://localhost:6379
   3. DEBUG = true

Actions:
   [a]dd     - Add a new variable
   [e]dit    - Edit an existing variable
   [d]elete  - Delete a variable
   [desc]    - Edit description
   [s]ave    - Save changes and exit
   [q]uit    - Quit without saving

> 
```

### Example Editing Session

```bash
# Start editing
envswitch edit myapp

# In the interactive editor:
# 1. Type 'a' to add a new variable
# 2. Enter variable name: LOG_LEVEL
# 3. Enter variable value: info
# 4. Type 'e' to edit existing variable
# 5. Enter variable name to edit: DEBUG
# 6. Enter new value: false
# 7. Type 's' to save changes
```

## Advanced Workflows

### Configuration Migration

```bash
# Export from old system
envswitch export -o migration-backup.json --metadata --pretty

# Import to new system with backup
envswitch import migration-backup.json --backup --verbose

# Verify migration
envswitch list --verbose
```

### Environment Synchronization

```bash
# Export production config
envswitch export -c production -o prod-config.json --metadata

# Import to staging with merge
envswitch import prod-config.json --merge

# Rename imported config
envswitch edit production  # Change name to staging-from-prod
```

### Batch Operations

```bash
# Export multiple specific configurations
envswitch export -c "config1,config2,config3" -o batch.json

# Create multiple configs from template
for env in dev staging prod; do
  envswitch set "myapp-$env" \
    -e DATABASE_URL="postgresql://$env-db:5432/myapp" \
    -e ENVIRONMENT="$env" \
    -d "MyApp $env environment"
done
```

## AI Model Configuration Examples

### OpenAI Configurations

```bash
# GPT-4 Configuration
envswitch set openai-gpt4 \
  -e OPENAI_API_KEY=sk-your-openai-key \
  -e OPENAI_MODEL=gpt-4 \
  -e OPENAI_BASE_URL=https://api.openai.com/v1 \
  -d "OpenAI GPT-4 configuration"

# GPT-3.5 Configuration
envswitch set openai-gpt35 \
  -e OPENAI_API_KEY=sk-your-openai-key \
  -e OPENAI_MODEL=gpt-3.5-turbo \
  -e OPENAI_BASE_URL=https://api.openai.com/v1 \
  -d "OpenAI GPT-3.5 Turbo configuration"
```

### Anthropic Claude Configurations

```bash
# Claude 3 Sonnet
envswitch set claude-sonnet \
  -e ANTHROPIC_API_KEY=sk-ant-your-key \
  -e ANTHROPIC_MODEL=claude-3-sonnet-20240229 \
  -e ANTHROPIC_BASE_URL=https://api.anthropic.com \
  -d "Anthropic Claude 3 Sonnet"

# Claude 3 Haiku
envswitch set claude-haiku \
  -e ANTHROPIC_API_KEY=sk-ant-your-key \
  -e ANTHROPIC_MODEL=claude-3-haiku-20240307 \
  -e ANTHROPIC_BASE_URL=https://api.anthropic.com \
  -d "Anthropic Claude 3 Haiku"
```

### Alternative AI Providers

```bash
# DeepSeek Configuration
envswitch set deepseek \
  -e ANTHROPIC_BASE_URL=https://api.deepseek.com \
  -e ANTHROPIC_MODEL=deepseek-chat \
  -e ANTHROPIC_AUTH_TOKEN=sk-your-deepseek-token \
  -d "DeepSeek AI configuration"

# Kimi Configuration
envswitch set kimi \
  -e ANTHROPIC_BASE_URL=https://api.moonshot.cn \
  -e ANTHROPIC_MODEL=moonshot-v1-8k \
  -e ANTHROPIC_AUTH_TOKEN=sk-your-kimi-token \
  -d "Kimi AI configuration"

# Local AI Model
envswitch set local-llm \
  -e ANTHROPIC_BASE_URL=http://localhost:8080 \
  -e ANTHROPIC_MODEL=llama2 \
  -e ANTHROPIC_AUTH_TOKEN=local-token \
  -d "Local LLM configuration"
```

### AI Configuration Management

```bash
# Export all AI configurations
envswitch export -c "openai-gpt4,claude-sonnet,deepseek,kimi" -o ai-configs.json --metadata

# Quick switching aliases (add to your shell config)
alias ai-openai='eval "$(envswitch use openai-gpt4)"'
alias ai-claude='eval "$(envswitch use claude-sonnet)"'
alias ai-deepseek='eval "$(envswitch use deepseek)"'
alias ai-kimi='eval "$(envswitch use kimi)"'
```

## Development Environment Management

### Multi-Environment Setup

```bash
# Development Environment
envswitch set dev \
  -e DATABASE_URL=postgresql://localhost:5432/myapp_dev \
  -e REDIS_URL=redis://localhost:6379/0 \
  -e API_URL=http://localhost:3000 \
  -e DEBUG=true \
  -e LOG_LEVEL=debug \
  -e NODE_ENV=development \
  -d "Development environment"

# Staging Environment
envswitch set staging \
  -e DATABASE_URL=postgresql://staging-db:5432/myapp \
  -e REDIS_URL=redis://staging-redis:6379/0 \
  -e API_URL=https://staging-api.example.com \
  -e DEBUG=false \
  -e LOG_LEVEL=info \
  -e NODE_ENV=staging \
  -d "Staging environment"

# Production Environment
envswitch set prod \
  -e DATABASE_URL=postgresql://prod-db:5432/myapp \
  -e REDIS_URL=redis://prod-redis:6379/0 \
  -e API_URL=https://api.example.com \
  -e DEBUG=false \
  -e LOG_LEVEL=warn \
  -e NODE_ENV=production \
  -d "Production environment"
```

### Testing Configurations

```bash
# Unit Testing
envswitch set test-unit \
  -e DATABASE_URL=postgresql://localhost:5432/myapp_test \
  -e REDIS_URL=redis://localhost:6379/1 \
  -e NODE_ENV=test \
  -e DEBUG=false \
  -d "Unit testing environment"

# Integration Testing
envswitch set test-integration \
  -e DATABASE_URL=postgresql://test-db:5432/myapp_test \
  -e REDIS_URL=redis://test-redis:6379/0 \
  -e API_URL=http://test-api:3000 \
  -e NODE_ENV=test \
  -e DEBUG=true \
  -d "Integration testing environment"
```

## Backup and Recovery

### Regular Backups

```bash
# Daily backup script
#!/bin/bash
DATE=$(date +%Y%m%d)
envswitch export -o "backups/envswitch-backup-$DATE.json" --metadata --pretty

# Keep only last 7 days of backups
find backups/ -name "envswitch-backup-*.json" -mtime +7 -delete
```

### Disaster Recovery

```bash
# Full system backup
envswitch export -o "disaster-recovery-$(date +%Y%m%d-%H%M%S).json" --metadata --pretty

# Recovery from backup
envswitch import disaster-recovery-20241201-143022.json --backup --verbose

# Verify recovery
envswitch list --verbose
envswitch status
```

### Selective Recovery

```bash
# Export only critical configurations
envswitch export -c "prod,staging" -o critical-configs.json --metadata

# Import only specific configurations
envswitch import full-backup.json --dry-run  # Preview first
envswitch import full-backup.json --merge    # Then import
```

## Troubleshooting Examples

### Debugging Import Issues

```bash
# Check file format
file configs.json
head -n 5 configs.json

# Validate JSON
python -m json.tool configs.json > /dev/null

# Import with verbose output
envswitch import configs.json --dry-run --verbose

# Import with backup for safety
envswitch import configs.json --backup --verbose
```

### Fixing Corrupted Configurations

```bash
# Export current configs as backup
envswitch export -o emergency-backup.json

# Check configuration file
cat ~/.config/envswitch/config.json | python -m json.tool

# If corrupted, restore from backup
envswitch import emergency-backup.json --force
```

### Performance Issues

```bash
# For large configurations, export specific ones
envswitch export -c "config1,config2" -o subset.json

# Use non-pretty format for speed
envswitch export -o configs.json  # Without --pretty

# Check configuration sizes
envswitch list --verbose | grep -E "(Variables|Created)"
```

### Shell Integration Issues

```bash
# Check shell type
echo $SHELL

# Test command generation
envswitch use myconfig --dry-run

# For fish shell, use different syntax
if test "$SHELL" = "/usr/local/bin/fish"
    eval (envswitch use myconfig)
else
    eval "$(envswitch use myconfig)"
end
```

### Permission Problems

```bash
# Check permissions
ls -la ~/.config/envswitch/

# Fix permissions
chmod 755 ~/.config/envswitch/
chmod 644 ~/.config/envswitch/config.json

# Check disk space
df -h ~/.config/

# Check if directory is writable
touch ~/.config/envswitch/test && rm ~/.config/envswitch/test
```

## Best Practices

### Configuration Naming

```bash
# Use descriptive names
envswitch set myapp-prod-us-east    # Good
envswitch set prod                  # Less clear

# Use consistent naming patterns
envswitch set myapp-dev
envswitch set myapp-staging
envswitch set myapp-prod

# Include version or date for temporary configs
envswitch set myapp-dev-v2
envswitch set myapp-hotfix-20241201
```

### Regular Maintenance

```bash
# Weekly backup
envswitch export -o "weekly-backup-$(date +%Y%W).json" --metadata --pretty

# Clean up old configurations
envswitch list | grep -E "(old|temp|test)" | while read config; do
  echo "Consider deleting: $config"
done

# Verify all configurations work
envswitch list | while read config; do
  echo "Testing $config..."
  envswitch use "$config" --dry-run
done
```

### Security Considerations

```bash
# Don't export sensitive configs to shared locations
envswitch export -c "dev,staging" -o safe-configs.json  # Exclude prod

# Use descriptions to document sensitivity
envswitch set prod-secrets \
  -e SECRET_KEY=... \
  -d "SENSITIVE: Production secrets - handle with care"

# Regular audit of configurations
envswitch list --verbose | grep -i secret
```
## Adv
anced Integration Examples

### CI/CD Pipeline Integration

```bash
# GitLab CI example
stages:
  - test
  - deploy

test:
  script:
    - envswitch import ci-configs.json --force
    - eval "$(envswitch use test-env)"
    - npm test

deploy:
  script:
    - envswitch import production-configs.json --force
    - eval "$(envswitch use prod-env)"
    - ./deploy.sh
```

### Docker Integration

```bash
# Dockerfile example
FROM node:16
COPY envswitch /usr/local/bin/
COPY configs.json /app/
WORKDIR /app
RUN envswitch import configs.json --force
CMD eval "$(envswitch use production)" && npm start
```

### Kubernetes Integration

```bash
# Create ConfigMap from EnvSwitch export
envswitch export -c k8s-config --format env -o k8s.env
kubectl create configmap app-config --from-env-file=k8s.env

# Use in deployment
apiVersion: apps/v1
kind: Deployment
spec:
  template:
    spec:
      containers:
      - name: app
        envFrom:
        - configMapRef:
            name: app-config
```

### Terraform Integration

```bash
# Export for Terraform variables
envswitch export -c terraform-vars --format env -o terraform.env

# Use in terraform.tfvars
# (Convert ENV format to Terraform format as needed)
```

## Automation Scripts

### Automated Configuration Sync

```bash
#!/bin/bash
# sync-configs.sh - Sync configurations across environments

SOURCE_ENV="staging"
TARGET_ENV="dev"
BACKUP_DIR="$HOME/envswitch-backups"

# Create backup
mkdir -p "$BACKUP_DIR"
envswitch export -o "$BACKUP_DIR/backup-$(date +%Y%m%d-%H%M%S).json" --metadata

# Export source configuration
envswitch export -c "$SOURCE_ENV" -o "/tmp/sync-config.json"

# Import to target (with backup)
envswitch import "/tmp/sync-config.json" --backup --merge

# Clean up
rm "/tmp/sync-config.json"

echo "Configuration sync completed: $SOURCE_ENV -> $TARGET_ENV"
```

### Configuration Validation Script

```bash
#!/bin/bash
# validate-configs.sh - Validate all configurations

echo "Validating EnvSwitch configurations..."

# Get all configuration names
CONFIGS=$(envswitch list | grep -v "No configurations" | awk '{print $1}')

for config in $CONFIGS; do
    echo -n "Validating $config... "
    
    # Test configuration switching
    if envswitch use "$config" --dry-run > /dev/null 2>&1; then
        echo "✅ OK"
    else
        echo "❌ FAILED"
        envswitch use "$config" --dry-run
    fi
done

echo "Validation completed."
```

### Bulk Configuration Management

```bash
#!/bin/bash
# bulk-update.sh - Update multiple configurations

CONFIGS=("dev" "staging" "prod")
NEW_VAR="API_VERSION=v2"

for config in "${CONFIGS[@]}"; do
    echo "Updating $config..."
    envswitch set "$config" -e "$NEW_VAR"
done

echo "Bulk update completed."
```

## Error Recovery Examples

### Configuration Recovery Workflow

```bash
#!/bin/bash
# recover-configs.sh - Comprehensive recovery workflow

BACKUP_DIR="$HOME/.config/envswitch/backups"
CONFIG_FILE="$HOME/.config/envswitch/config.json"

echo "Starting EnvSwitch recovery..."

# Step 1: Check if config file exists and is valid
if [ -f "$CONFIG_FILE" ]; then
    if python -m json.tool "$CONFIG_FILE" > /dev/null 2>&1; then
        echo "✅ Configuration file is valid"
        exit 0
    else
        echo "❌ Configuration file is corrupted"
    fi
else
    echo "❌ Configuration file is missing"
fi

# Step 2: Look for backups
if [ -d "$BACKUP_DIR" ]; then
    LATEST_BACKUP=$(ls -t "$BACKUP_DIR"/*.json 2>/dev/null | head -n1)
    if [ -n "$LATEST_BACKUP" ]; then
        echo "Found backup: $LATEST_BACKUP"
        
        # Backup corrupted file
        if [ -f "$CONFIG_FILE" ]; then
            mv "$CONFIG_FILE" "$CONFIG_FILE.corrupted.$(date +%Y%m%d-%H%M%S)"
        fi
        
        # Restore from backup
        envswitch import "$LATEST_BACKUP" --force
        echo "✅ Restored from backup"
        exit 0
    fi
fi

# Step 3: Start fresh
echo "No valid backups found. Starting with fresh configuration."
rm -f "$CONFIG_FILE"
envswitch list  # This creates a new empty config file
echo "✅ Fresh configuration created"
```

### Emergency Configuration Creation

```bash
#!/bin/bash
# emergency-config.sh - Create emergency working configuration

echo "Creating emergency configuration..."

envswitch set emergency \
  -e PATH="$PATH" \
  -e HOME="$HOME" \
  -e USER="$USER" \
  -e SHELL="$SHELL" \
  -d "Emergency configuration with basic environment"

echo "✅ Emergency configuration created"
echo "To use: eval \"\$(envswitch use emergency)\""
```

## Performance Optimization Examples

### Large Configuration Handling

```bash
# For configurations with many variables (>100)
# Split into logical groups

# Database configuration
envswitch set myapp-db \
  -e DATABASE_URL=... \
  -e DB_POOL_SIZE=10 \
  -e DB_TIMEOUT=30 \
  -d "Database configuration"

# API configuration
envswitch set myapp-api \
  -e API_BASE_URL=... \
  -e API_KEY=... \
  -e API_TIMEOUT=5000 \
  -d "API configuration"

# Use multiple configurations
eval "$(envswitch use myapp-db)"
eval "$(envswitch use myapp-api)"
```

### Optimized Export/Import

```bash
# For faster operations on large configurations
# Use specific configuration exports
envswitch export -c "critical-config" -o critical.json

# Skip validation for trusted files
envswitch import trusted-config.json --skip-validation

# Use non-pretty format for speed
envswitch export -o fast-export.json  # Without --pretty flag
```

## Integration Testing Examples

### End-to-End Testing

```bash
#!/bin/bash
# e2e-test.sh - End-to-end configuration testing

set -e

echo "Starting end-to-end configuration test..."

# Test configuration creation
envswitch set test-config -e TEST_VAR=test_value -d "Test configuration"

# Test export
envswitch export -c test-config -o test-export.json

# Test deletion
envswitch delete test-config --force

# Test import
envswitch import test-export.json

# Test switching
eval "$(envswitch use test-config)"

# Verify variable is set
if [ "$TEST_VAR" = "test_value" ]; then
    echo "✅ End-to-end test passed"
else
    echo "❌ End-to-end test failed"
    exit 1
fi

# Cleanup
envswitch delete test-config --force
rm test-export.json

echo "✅ Test completed successfully"
```

### Cross-Format Testing

```bash
#!/bin/bash
# format-test.sh - Test all supported formats

FORMATS=("json" "env" "yaml")
TEST_CONFIG="format-test"

# Create test configuration
envswitch set "$TEST_CONFIG" \
  -e FORMAT_TEST=true \
  -e TEST_VALUE=123 \
  -d "Format testing configuration"

for format in "${FORMATS[@]}"; do
    echo "Testing $format format..."
    
    # Export in format
    envswitch export -c "$TEST_CONFIG" --format "$format" -o "test.$format"
    
    # Delete original
    envswitch delete "$TEST_CONFIG" --force
    
    # Import back
    envswitch import "test.$format"
    
    # Verify
    if envswitch show "$TEST_CONFIG" | grep -q "FORMAT_TEST"; then
        echo "✅ $format format test passed"
    else
        echo "❌ $format format test failed"
        exit 1
    fi
    
    # Cleanup
    rm "test.$format"
done

# Final cleanup
envswitch delete "$TEST_CONFIG" --force
echo "✅ All format tests passed"
```

This comprehensive examples document covers all the major use cases and provides practical, copy-paste ready examples for users to get started quickly with EnvSwitch's advanced features.