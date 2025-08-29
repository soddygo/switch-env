# EnvSwitch

A tool for managing and switching environment variable configurations.

## Project Structure

```
src/
├── main.rs      # Main entry point and command dispatch
├── cli.rs       # Command line interface definitions
├── config.rs    # Configuration management
├── env.rs       # Environment variable management
├── shell.rs     # Shell detection and command generation
└── error.rs     # Error types and handling
```

## Dependencies

- `clap` - Command line argument parsing
- `serde` - Serialization/deserialization
- `serde_json` - JSON support
- `thiserror` - Error handling
- `chrono` - Date/time handling
- `dirs` - Cross-platform directory paths

## Development Status

✅ Task 1: Project structure and core dependencies - COMPLETED
✅ Task 2: Error handling and basic type definitions - COMPLETED  
✅ Task 3: Shell detection and command generation - COMPLETED
✅ Task 4: Configuration data model and serialization - COMPLETED

## Usage (Planned)

```bash
# Create a configuration
envswitch set deepseek -e ANTHROPIC_BASE_URL=https://api.deepseek.com -e ANTHROPIC_MODEL=deepseek-chat

# Switch to a configuration
eval "$(envswitch use deepseek)"

# List configurations
envswitch list

# Show current status
envswitch status
```
