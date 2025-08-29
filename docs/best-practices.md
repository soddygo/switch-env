# Best Practices Guide

This guide covers best practices for using EnvSwitch effectively and securely.

## Configuration Management

### Naming Conventions

Use clear, hierarchical naming conventions for your configurations:

```bash
# Good: Descriptive and hierarchical
envswitch set ai-claude-sonnet-coding
envswitch set ai-openai-gpt4-analysis
envswitch set db-postgres-dev
envswitch set api-rest-v1-staging

# Avoid: Vague or confusing names
envswitch set config1
envswitch set temp
envswitch set test123
```

### Organization Strategies

#### By Environment
```bash
envswitch set dev-database
envswitch set staging-database
envswitch set prod-database
```

#### By Service
```bash
envswitch set user-service-dev
envswitch set order-service-dev
envswitch set payment-service-dev
```

#### By Purpose
```bash
envswitch set ai-coding-assistant
envswitch set ai-content-generation
envswitch set ai-data-analysis
```

### Configuration Descriptions

Always add descriptions to your configurations:

```bash
envswitch set claude-coding \
  -e ANTHROPIC_API_KEY=sk-ant-your-key \
  -e ANTHROPIC_MODEL=claude-3-5-sonnet-20241022 \
  --description "Claude 3.5 Sonnet optimized for coding tasks with system prompts"
```

## Security Best Practices

### Sensitive Information Handling

**DO NOT** hardcode sensitive values directly:

```bash
# ❌ Bad: Hardcoded secrets
envswitch set prod-api \
  -e API_KEY=sk-1234567890abcdef \
  -e DATABASE_PASSWORD=supersecret123
```

**DO** reference environment variables or use secure storage:

```bash
# ✅ Good: Reference existing environment variables
envswitch set prod-api \
  -e API_KEY="$PROD_API_KEY" \
  -e DATABASE_PASSWORD="$PROD_DB_PASSWORD"

# ✅ Good: Use placeholders and update separately
envswitch set prod-api \
  -e API_KEY=REPLACE_WITH_ACTUAL_KEY \
  -e DATABASE_PASSWORD=REPLACE_WITH_ACTUAL_PASSWORD
```

### File Permissions

Ensure your configuration files have appropriate permissions:

```bash
# Check current permissions
ls -la ~/.config/envswitch/

# Set restrictive permissions if needed
chmod 600 ~/.config/envswitch/config.json
chmod 700 ~/.config/envswitch/
```

### Backup Security

When creating backups, be mindful of sensitive data:

```bash
# Create backups in secure locations
envswitch export -o ~/secure-backups/envswitch-$(date +%Y%m%d).json

# Set appropriate permissions on backup files
chmod 600 ~/secure-backups/envswitch-*.json
```

## Shell Integration

### Recommended Aliases

Create convenient aliases for common operations:

```bash
# ~/.zshrc or ~/.bashrc
alias envs='envswitch list'
alias envstatus='envswitch status'
alias envbackup='envswitch export -o ~/backups/envswitch-$(date +%Y%m%d).json'

# Quick switching aliases
alias use-dev='eval "$(envswitch use dev)"'
alias use-staging='eval "$(envswitch use staging)"'
alias use-prod='eval "$(envswitch use prod)"'
```

### Shell Functions

Create functions for complex operations:

```bash
# Switch and show status
switch_env() {
    if [ -z "$1" ]; then
        echo "Usage: switch_env <config_name>"
        envswitch list
        return 1
    fi
    
    eval "$(envswitch use $1)"
    if [ $? -eq 0 ]; then
        echo "✅ Switched to: $1"
        envswitch status
    else
        echo "❌ Failed to switch to: $1"
    fi
}

# Quick AI model switching
ai_model() {
    case "$1" in
        "claude"|"c")
            eval "$(envswitch use claude-sonnet)"
            ;;
        "gpt"|"g")
            eval "$(envswitch use gpt4-turbo)"
            ;;
        "local"|"l")
            eval "$(envswitch use ollama)"
            ;;
        *)
            echo "Available models: claude (c), gpt (g), local (l)"
            ;;
    esac
}
```

## Workflow Integration

### Development Workflow

Integrate EnvSwitch into your development workflow:

```bash
# Start of day setup
start_dev() {
    eval "$(envswitch use dev-database)"
    eval "$(envswitch use dev-api)"
    echo "🚀 Development environment ready"
    envswitch status
}

# Pre-deployment checks
pre_deploy() {
    eval "$(envswitch use staging)"
    echo "🧪 Running pre-deployment tests..."
    npm test
    if [ $? -eq 0 ]; then
        echo "✅ Tests passed, ready for production"
        eval "$(envswitch use prod)"
    else
        echo "❌ Tests failed, staying in staging"
    fi
}
```

### CI/CD Integration

Use EnvSwitch in your CI/CD pipelines:

```yaml
# .github/workflows/deploy.yml
- name: Setup Environment
  run: |
    eval "$(envswitch use ci-testing)"
    npm test

- name: Deploy to Staging
  run: |
    eval "$(envswitch use staging-deploy)"
    ./deploy.sh staging

- name: Deploy to Production
  if: github.ref == 'refs/heads/main'
  run: |
    eval "$(envswitch use prod-deploy)"
    ./deploy.sh production
```

## Configuration Validation

### Testing Configurations

Always test new configurations:

```bash
# Create and test a new configuration
envswitch set new-config -e TEST_VAR=test_value
eval "$(envswitch use new-config)"

# Verify the environment variable is set
if [ "$TEST_VAR" = "test_value" ]; then
    echo "✅ Configuration working correctly"
else
    echo "❌ Configuration failed"
fi
```

### Validation Scripts

Create validation scripts for critical configurations:

```bash
#!/bin/bash
# validate-ai-config.sh

validate_ai_config() {
    local config_name=$1
    
    eval "$(envswitch use $config_name)"
    
    # Check required variables
    if [ -z "$ANTHROPIC_API_KEY" ]; then
        echo "❌ Missing ANTHROPIC_API_KEY"
        return 1
    fi
    
    if [ -z "$ANTHROPIC_MODEL" ]; then
        echo "❌ Missing ANTHROPIC_MODEL"
        return 1
    fi
    
    # Test API connectivity (optional)
    curl -s -H "Authorization: Bearer $ANTHROPIC_API_KEY" \
         "$ANTHROPIC_BASE_URL/v1/models" > /dev/null
    
    if [ $? -eq 0 ]; then
        echo "✅ $config_name configuration is valid"
    else
        echo "⚠️  $config_name configuration may have issues"
    fi
}

# Usage: ./validate-ai-config.sh claude-sonnet
validate_ai_config "$1"
```

## Backup and Recovery

### Regular Backups

Set up automated backups:

```bash
# Add to crontab (crontab -e)
# Daily backup at 2 AM
0 2 * * * /usr/local/bin/envswitch export -o ~/backups/envswitch-$(date +\%Y\%m\%d).json

# Weekly cleanup (keep only last 4 weeks)
0 3 * * 0 find ~/backups -name "envswitch-*.json" -mtime +28 -delete
```

### Recovery Procedures

Document recovery procedures:

```bash
# Recovery script
#!/bin/bash
# recover-envswitch.sh

BACKUP_DIR="$HOME/backups"
LATEST_BACKUP=$(ls -t $BACKUP_DIR/envswitch-*.json | head -1)

if [ -f "$LATEST_BACKUP" ]; then
    echo "Restoring from: $LATEST_BACKUP"
    envswitch import "$LATEST_BACKUP" --merge
    echo "✅ Recovery complete"
else
    echo "❌ No backup found in $BACKUP_DIR"
    exit 1
fi
```

## Performance Optimization

### Configuration Size

Keep configurations focused and avoid unnecessary variables:

```bash
# ✅ Good: Focused configuration
envswitch set api-dev \
  -e API_URL=http://localhost:3000 \
  -e API_KEY=dev-key \
  -e DEBUG=true

# ❌ Avoid: Bloated configuration with unused variables
envswitch set api-dev \
  -e API_URL=http://localhost:3000 \
  -e API_KEY=dev-key \
  -e DEBUG=true \
  -e UNUSED_VAR1=value1 \
  -e UNUSED_VAR2=value2 \
  # ... many more unused variables
```

### Shell Performance

Use efficient shell integration:

```bash
# ✅ Good: Direct evaluation
eval "$(envswitch use config-name)"

# ❌ Avoid: Unnecessary subshells or pipes
envswitch use config-name | bash
```

## Team Collaboration

### Shared Configurations

Create shared configuration templates:

```bash
# team-configs.json template
{
  "configs": {
    "dev-template": {
      "variables": {
        "DATABASE_URL": "postgresql://localhost:5432/myapp_dev",
        "API_URL": "http://localhost:3000",
        "DEBUG": "true"
      },
      "description": "Development environment template"
    }
  }
}
```

### Documentation

Document your team's configurations:

```markdown
# Team Environment Configurations

## Available Configurations

- `dev-database`: Local development database
- `staging-api`: Staging API environment  
- `prod-readonly`: Production read-only access

## Usage

```bash
# Start development
eval "$(envswitch use dev-database)"

# Deploy to staging
eval "$(envswitch use staging-api)"
```

## Adding New Configurations

1. Create the configuration: `envswitch set new-config ...`
2. Test thoroughly
3. Add to team documentation
4. Share via export/import
```

### Version Control

Consider version controlling your configurations (without secrets):

```bash
# Export configurations without sensitive data
envswitch export -o team-configs-template.json

# Add to version control
git add team-configs-template.json
git commit -m "Add team configuration templates"
```

## Troubleshooting

### Common Issues

**Configuration not found:**
```bash
# List available configurations
envswitch list

# Check for typos in configuration name
envswitch show config-name
```

**Environment variables not set:**
```bash
# Verify the switch command worked
eval "$(envswitch use config-name)"
echo $?  # Should be 0 for success

# Check current environment
envswitch status
```

**Shell integration issues:**
```bash
# Verify shell type
echo $SHELL

# Test command generation
envswitch use config-name  # Don't use eval to see raw output
```

### Debugging

Enable verbose output for troubleshooting:

```bash
# Check configuration details
envswitch show config-name

# Verify file permissions
ls -la ~/.config/envswitch/

# Test with a simple configuration
envswitch set debug-test -e TEST_VAR=debug_value
eval "$(envswitch use debug-test)"
echo "TEST_VAR is: $TEST_VAR"
```

## Maintenance

### Regular Maintenance Tasks

1. **Review configurations monthly:**
   ```bash
   envswitch list
   # Remove unused configurations
   envswitch delete unused-config
   ```

2. **Update descriptions:**
   ```bash
   envswitch edit config-name  # Add or update descriptions
   ```

3. **Backup configurations:**
   ```bash
   envswitch export -o monthly-backup-$(date +%Y%m).json
   ```

4. **Validate critical configurations:**
   ```bash
   ./validate-configs.sh
   ```

### Cleanup

Remove old or unused configurations:

```bash
# List all configurations with details
envswitch list --verbose

# Remove unused configurations
envswitch delete old-config-1 old-config-2

# Clean up backup files
find ~/backups -name "envswitch-*.json" -mtime +90 -delete
```

Following these best practices will help you use EnvSwitch effectively, securely, and maintainably in your development workflow.