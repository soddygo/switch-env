# EnvSwitch

A fast, reliable command-line tool for managing and switching environment variable configurations. Built with Rust for performance and safety.

## Features

- 🚀 **Fast Configuration Switching**: Instantly switch between different environment variable sets
- 🐚 **Multi-Shell Support**: Works with zsh, fish, bash, and other shells
- 💾 **Persistent Storage**: Configurations are saved locally and persist across sessions
- 🔒 **Safe Operations**: Built-in validation prevents invalid configurations
- 📦 **Import/Export**: Share configurations between machines or backup your settings
- 🎯 **Claude AI Integration**: Optimized for switching between different AI model configurations

## Installation

### From Source

```bash
git clone https://github.com/soddygo/envswitch
cd envswitch
cargo build --release
cp target/release/envswitch /usr/local/bin/
```

### Using Cargo

```bash
cargo install --git https://github.com/soddygo/envswitch
```

## Quick Start

### 1. Create your first configuration

```bash
# Create a configuration for DeepSeek AI
envswitch set deepseek \
  -e ANTHROPIC_BASE_URL=https://api.deepseek.com \
  -e ANTHROPIC_MODEL=deepseek-chat \
  -e ANTHROPIC_AUTH_TOKEN=sk-your-deepseek-token

# Create a configuration for Kimi AI
envswitch set kimi \
  -e ANTHROPIC_BASE_URL=https://api.moonshot.cn \
  -e ANTHROPIC_MODEL=moonshot-v1-8k \
  -e ANTHROPIC_AUTH_TOKEN=sk-your-kimi-token
```

### 2. Switch between configurations

```bash
# Switch to DeepSeek configuration
eval "$(envswitch use deepseek)"

# Switch to Kimi configuration
eval "$(envswitch use kimi)"
```

### 3. View your configurations

```bash
# List all configurations
envswitch list

# Show current active configuration and environment variables
envswitch status
```

## Shell Integration

For the best experience, add these aliases to your shell configuration:

### Zsh (~/.zshrc)

```bash
alias switch-deepseek='eval "$(envswitch use deepseek)"'
alias switch-kimi='eval "$(envswitch use kimi)"'
alias envs='envswitch list'
alias envstatus='envswitch status'
```

### Fish (~/.config/fish/config.fish)

```fish
alias switch-deepseek='eval (envswitch use deepseek)'
alias switch-kimi='eval (envswitch use kimi)'
alias envs='envswitch list'
alias envstatus='envswitch status'
```

### Bash (~/.bashrc)

```bash
alias switch-deepseek='eval "$(envswitch use deepseek)"'
alias switch-kimi='eval "$(envswitch use kimi)"'
alias envs='envswitch list'
alias envstatus='envswitch status'
```

## Commands

### Configuration Management

```bash
# Create or update a configuration
envswitch set <alias> -e KEY1=value1 -e KEY2=value2

# List all configurations
envswitch list

# Show detailed information about a configuration
envswitch show <alias>

# Delete a configuration
envswitch delete <alias>

# Edit a configuration interactively
envswitch edit <alias>
```

### Environment Switching

```bash
# Switch to a configuration (generates shell commands)
envswitch use <alias>

# Show current environment status
envswitch status

# Clear environment variables
envswitch clear
```

### Import/Export

```bash
# Export all configurations to JSON (default format)
envswitch export -o configs.json

# Export with metadata and pretty formatting
envswitch export -o configs.json --metadata --pretty

# Export specific configurations
envswitch export -c deepseek,kimi -o my-ai-configs.json

# Export in different formats
envswitch export -o configs.env --format env
envswitch export -o configs.yaml --format yaml

# Import configurations from a file
envswitch import configs.json

# Import with backup (creates backup before importing)
envswitch import configs.json --backup

# Import with conflict resolution
envswitch import configs.json --force    # Overwrite existing
envswitch import configs.json --merge    # Merge with existing

# Preview import without making changes
envswitch import configs.json --dry-run

# Import from different formats (auto-detected)
envswitch import configs.env
envswitch import configs.yaml
```

## Configuration Examples

### AI Model Configurations

```bash
# OpenAI GPT-4
envswitch set openai \
  -e OPENAI_API_KEY=sk-your-openai-key \
  -e OPENAI_MODEL=gpt-4 \
  -e OPENAI_BASE_URL=https://api.openai.com/v1

# Anthropic Claude
envswitch set claude \
  -e ANTHROPIC_API_KEY=sk-ant-your-key \
  -e ANTHROPIC_MODEL=claude-3-sonnet-20240229 \
  -e ANTHROPIC_BASE_URL=https://api.anthropic.com

# Local development
envswitch set local \
  -e DATABASE_URL=postgresql://localhost:5432/myapp_dev \
  -e REDIS_URL=redis://localhost:6379 \
  -e DEBUG=true \
  -e LOG_LEVEL=debug

# Production
envswitch set prod \
  -e DATABASE_URL=postgresql://prod-db:5432/myapp \
  -e REDIS_URL=redis://prod-redis:6379 \
  -e DEBUG=false \
  -e LOG_LEVEL=info
```

## Advanced Usage

### Conditional Configurations

```bash
# Create configurations with descriptions
envswitch set staging \
  -e API_URL=https://staging-api.example.com \
  -e DEBUG=true \
  --description "Staging environment configuration"
```

### Backup and Restore

```bash
# Create a backup of all configurations
envswitch export -o backup-$(date +%Y%m%d).json --metadata --pretty

# Restore from backup with merge
envswitch import backup-20241201.json --merge --backup

# Force restore (overwrites existing configurations)
envswitch import backup-20241201.json --force

# Preview what would be restored
envswitch import backup-20241201.json --dry-run
```

### Interactive Configuration Editing

```bash
# Edit a configuration interactively
envswitch edit myconfig

# The interactive editor allows you to:
# - Add new environment variables
# - Edit existing variables
# - Delete variables
# - Update configuration description
# - Save or cancel changes
```

### Advanced Export/Import Options

```bash
# Export with verbose output
envswitch export -o configs.json --verbose

# Export only specific configurations with metadata
envswitch export -c prod,staging -o prod-configs.json --metadata --pretty

# Import with validation and backup
envswitch import configs.json --backup --verbose

# Cross-format conversion (export JSON, import as ENV)
envswitch export -o temp.json --format json
envswitch import temp.json  # Auto-detects JSON format
```

## Configuration File Location

Configurations are stored in:
- **macOS/Linux**: `~/.config/envswitch/config.json`
- **Windows**: `%APPDATA%\envswitch\config.json`

## Troubleshooting

### Common Issues

**Configuration not found**
```bash
envswitch list  # Check available configurations
# If you see suggestions, check for typos in the configuration name
```

**Shell commands not working**
```bash
# Make sure to use eval
eval "$(envswitch use myconfig)"

# Check your shell type
echo $SHELL

# For fish shell, use different syntax
eval (envswitch use myconfig)
```

**Permission errors**
```bash
# Check configuration directory permissions
ls -la ~/.config/envswitch/

# Fix permissions if needed
chmod 755 ~/.config/envswitch/
chmod 644 ~/.config/envswitch/config.json
```

**Import/Export issues**
```bash
# Check file format and content
envswitch import myfile.json --dry-run

# Use verbose mode for detailed error information
envswitch import myfile.json --verbose

# For corrupted files, check the format
file myfile.json  # Should show JSON data
```

**Interactive editing problems**
```bash
# If edit command doesn't work, check configuration exists
envswitch list

# Create new configuration if needed
envswitch edit newconfig  # Will offer to create new one
```

**Large configuration performance**
```bash
# For large configurations, use specific exports
envswitch export -c config1,config2 -o subset.json

# Use non-pretty format for faster processing
envswitch export -o configs.json  # Without --pretty flag
```

### Getting Help

```bash
# Show help for all commands
envswitch --help

# Show help for a specific command
envswitch set --help
```

## Development

### Building from Source

```bash
git clone https://github.com/soddygo/envswitch
cd envswitch
cargo build --release
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test suites
cargo test --test integration_tests
cargo test --test shell_compatibility_tests
cargo test --test error_scenario_tests
cargo test --test missing_features_tests
cargo test --test command_workflow_tests

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_export_import_workflow
```

### Project Structure

```
src/
├── main.rs              # Main entry point
├── cli.rs               # Command line interface
├── config.rs            # Configuration management
├── env.rs               # Environment variable handling
├── shell.rs             # Shell detection and command generation
├── error.rs             # Error types and handling
├── types.rs             # Common type definitions
├── commands/            # Command implementations
├── handlers/            # Utility handlers
└── utils/               # Utility functions

tests/
├── integration_tests.rs        # End-to-end workflow tests
├── shell_compatibility_tests.rs # Shell-specific tests
└── error_scenario_tests.rs     # Error handling tests
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for your changes
5. Ensure all tests pass (`cargo test`)
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) for performance and safety
- Uses [clap](https://github.com/clap-rs/clap) for command-line parsing
- Inspired by the need for easy AI model configuration switching
