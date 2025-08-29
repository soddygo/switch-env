# Configuration Examples

This document provides practical examples of how to use EnvSwitch for various scenarios.

## AI Model Configurations

### Claude AI Models

```bash
# Claude 3.5 Sonnet
envswitch set claude-sonnet \
  -e ANTHROPIC_API_KEY=sk-ant-your-key \
  -e ANTHROPIC_MODEL=claude-3-5-sonnet-20241022 \
  -e ANTHROPIC_BASE_URL=https://api.anthropic.com

# Claude 3 Haiku (faster, cheaper)
envswitch set claude-haiku \
  -e ANTHROPIC_API_KEY=sk-ant-your-key \
  -e ANTHROPIC_MODEL=claude-3-haiku-20240307 \
  -e ANTHROPIC_BASE_URL=https://api.anthropic.com
```

### OpenAI Models

```bash
# GPT-4 Turbo
envswitch set gpt4-turbo \
  -e OPENAI_API_KEY=sk-your-openai-key \
  -e OPENAI_MODEL=gpt-4-turbo-preview \
  -e OPENAI_BASE_URL=https://api.openai.com/v1

# GPT-3.5 Turbo (faster, cheaper)
envswitch set gpt35-turbo \
  -e OPENAI_API_KEY=sk-your-openai-key \
  -e OPENAI_MODEL=gpt-3.5-turbo \
  -e OPENAI_BASE_URL=https://api.openai.com/v1
```

### Alternative AI Providers

```bash
# DeepSeek V3
envswitch set deepseek \
  -e ANTHROPIC_BASE_URL=https://api.deepseek.com \
  -e ANTHROPIC_MODEL=deepseek-chat \
  -e ANTHROPIC_AUTH_TOKEN=sk-your-deepseek-token \
  -e ANTHROPIC_SMALL_FAST_MODEL=deepseek-chat

# Moonshot AI (Kimi)
envswitch set kimi \
  -e ANTHROPIC_BASE_URL=https://api.moonshot.cn \
  -e ANTHROPIC_MODEL=moonshot-v1-8k \
  -e ANTHROPIC_AUTH_TOKEN=sk-your-kimi-token \
  -e ANTHROPIC_SMALL_FAST_MODEL=moonshot-v1-8k

# Ollama (Local)
envswitch set ollama \
  -e ANTHROPIC_BASE_URL=http://localhost:11434/v1 \
  -e ANTHROPIC_MODEL=llama3.2 \
  -e ANTHROPIC_AUTH_TOKEN=ollama
```

## Development Environment Configurations

### Database Configurations

```bash
# Local Development
envswitch set dev-db \
  -e DATABASE_URL=postgresql://localhost:5432/myapp_dev \
  -e REDIS_URL=redis://localhost:6379/0 \
  -e MONGODB_URL=mongodb://localhost:27017/myapp_dev

# Staging Environment
envswitch set staging-db \
  -e DATABASE_URL=postgresql://staging-db.example.com:5432/myapp_staging \
  -e REDIS_URL=redis://staging-redis.example.com:6379/0 \
  -e MONGODB_URL=mongodb://staging-mongo.example.com:27017/myapp_staging

# Production Environment
envswitch set prod-db \
  -e DATABASE_URL=postgresql://prod-db.example.com:5432/myapp \
  -e REDIS_URL=redis://prod-redis.example.com:6379/0 \
  -e MONGODB_URL=mongodb://prod-mongo.example.com:27017/myapp
```

### API Configurations

```bash
# Development API
envswitch set dev-api \
  -e API_BASE_URL=http://localhost:3000 \
  -e API_KEY=dev-key-123 \
  -e DEBUG=true \
  -e LOG_LEVEL=debug \
  -e CORS_ORIGIN=http://localhost:3001

# Production API
envswitch set prod-api \
  -e API_BASE_URL=https://api.myapp.com \
  -e API_KEY=prod-key-xyz \
  -e DEBUG=false \
  -e LOG_LEVEL=info \
  -e CORS_ORIGIN=https://myapp.com
```

### Cloud Provider Configurations

```bash
# AWS Development
envswitch set aws-dev \
  -e AWS_PROFILE=dev \
  -e AWS_REGION=us-west-2 \
  -e AWS_ACCESS_KEY_ID=your-dev-access-key \
  -e AWS_SECRET_ACCESS_KEY=your-dev-secret-key \
  -e S3_BUCKET=myapp-dev-bucket

# AWS Production
envswitch set aws-prod \
  -e AWS_PROFILE=production \
  -e AWS_REGION=us-east-1 \
  -e AWS_ACCESS_KEY_ID=your-prod-access-key \
  -e AWS_SECRET_ACCESS_KEY=your-prod-secret-key \
  -e S3_BUCKET=myapp-prod-bucket

# Google Cloud
envswitch set gcp \
  -e GOOGLE_APPLICATION_CREDENTIALS=/path/to/service-account.json \
  -e GOOGLE_CLOUD_PROJECT=my-project-id \
  -e GCS_BUCKET=my-gcs-bucket
```

## Testing Configurations

### Test Environment Variables

```bash
# Unit Testing
envswitch set test \
  -e NODE_ENV=test \
  -e DATABASE_URL=postgresql://localhost:5432/myapp_test \
  -e REDIS_URL=redis://localhost:6379/1 \
  -e LOG_LEVEL=error \
  -e DISABLE_LOGGING=true

# Integration Testing
envswitch set integration \
  -e NODE_ENV=integration \
  -e DATABASE_URL=postgresql://localhost:5432/myapp_integration \
  -e API_BASE_URL=http://localhost:3000 \
  -e TIMEOUT=30000
```

## Workflow Examples

### Daily Development Workflow

```bash
# Morning: Start with development environment
eval "$(envswitch use dev-db)"
eval "$(envswitch use dev-api)"

# Testing: Switch to test environment
eval "$(envswitch use test)"
npm test

# Deployment: Switch to production for deployment
eval "$(envswitch use prod-api)"
eval "$(envswitch use aws-prod)"
```

### AI Model Comparison Workflow

```bash
# Test with different AI models
eval "$(envswitch use claude-sonnet)"
python my_ai_script.py

eval "$(envswitch use gpt4-turbo)"
python my_ai_script.py

eval "$(envswitch use deepseek)"
python my_ai_script.py
```

## Advanced Configuration Patterns

### Configuration with Descriptions

```bash
envswitch set claude-coding \
  -e ANTHROPIC_API_KEY=sk-ant-your-key \
  -e ANTHROPIC_MODEL=claude-3-5-sonnet-20241022 \
  -e ANTHROPIC_BASE_URL=https://api.anthropic.com \
  -e SYSTEM_PROMPT="You are a helpful coding assistant" \
  --description "Claude configuration optimized for coding tasks"
```

### Environment-Specific Configurations

```bash
# Development with debug flags
envswitch set dev-debug \
  -e NODE_ENV=development \
  -e DEBUG=* \
  -e VERBOSE=true \
  -e LOG_LEVEL=debug \
  -e ENABLE_PROFILING=true

# Production with optimizations
envswitch set prod-optimized \
  -e NODE_ENV=production \
  -e DEBUG=false \
  -e LOG_LEVEL=warn \
  -e ENABLE_COMPRESSION=true \
  -e CACHE_TTL=3600
```

### Multi-Service Configurations

```bash
# Microservices development
envswitch set microservices-dev \
  -e USER_SERVICE_URL=http://localhost:3001 \
  -e ORDER_SERVICE_URL=http://localhost:3002 \
  -e PAYMENT_SERVICE_URL=http://localhost:3003 \
  -e NOTIFICATION_SERVICE_URL=http://localhost:3004 \
  -e MESSAGE_QUEUE_URL=amqp://localhost:5672

# Microservices production
envswitch set microservices-prod \
  -e USER_SERVICE_URL=https://user-service.myapp.com \
  -e ORDER_SERVICE_URL=https://order-service.myapp.com \
  -e PAYMENT_SERVICE_URL=https://payment-service.myapp.com \
  -e NOTIFICATION_SERVICE_URL=https://notification-service.myapp.com \
  -e MESSAGE_QUEUE_URL=amqps://prod-rabbitmq.myapp.com:5671
```

## Import/Export Examples

### Backup All Configurations

```bash
# Create a timestamped backup
envswitch export -o "backup-$(date +%Y%m%d-%H%M%S).json"

# Export specific configurations
envswitch export -c "claude-sonnet,gpt4-turbo,deepseek" -o ai-models.json

# Export with metadata
envswitch export -o full-backup.json --metadata --pretty
```

### Share Team Configurations

```bash
# Export team development configurations
envswitch export -c "dev-db,staging-db,dev-api,staging-api" -o team-dev-configs.json

# Import team configurations
envswitch import team-dev-configs.json --merge

# Import with conflict resolution
envswitch import new-configs.json --force
```

## Shell Integration Examples

### Advanced Aliases

```bash
# Zsh configuration (~/.zshrc)
alias ai-claude='eval "$(envswitch use claude-sonnet)"'
alias ai-gpt='eval "$(envswitch use gpt4-turbo)"'
alias ai-local='eval "$(envswitch use ollama)"'
alias dev-mode='eval "$(envswitch use dev-db)" && eval "$(envswitch use dev-api)"'
alias prod-mode='eval "$(envswitch use prod-db)" && eval "$(envswitch use prod-api)"'

# Function to quickly switch and show status
switch_and_status() {
    eval "$(envswitch use $1)"
    envswitch status
}
alias switch='switch_and_status'
```

### Fish Functions

```fish
# Fish configuration (~/.config/fish/config.fish)
function ai-claude
    eval (envswitch use claude-sonnet)
end

function ai-gpt
    eval (envswitch use gpt4-turbo)
end

function switch-and-status
    eval (envswitch use $argv[1])
    envswitch status
end
```

## Best Practices

### Naming Conventions

```bash
# Use descriptive, hierarchical names
envswitch set ai-claude-sonnet-coding
envswitch set ai-openai-gpt4-analysis
envswitch set db-postgres-dev
envswitch set db-postgres-prod
envswitch set api-rest-v1-staging
```

### Security Considerations

```bash
# Use environment-specific tokens
envswitch set prod-secure \
  -e API_KEY="$PROD_API_KEY" \
  -e DATABASE_PASSWORD="$PROD_DB_PASSWORD"

# Avoid hardcoding sensitive values
# Instead, reference them from secure storage
```

### Configuration Validation

```bash
# Test configurations after creation
envswitch set new-config -e TEST_VAR=value
eval "$(envswitch use new-config)"
echo $TEST_VAR  # Should output: value
```

This comprehensive set of examples should help users understand how to effectively use EnvSwitch in various scenarios.