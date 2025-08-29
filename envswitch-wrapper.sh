#!/bin/bash
# envswitch wrapper script
# This script provides convenient shortcuts for common envswitch operations

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Helper function to print colored output
print_status() {
    echo -e "${GREEN}✓${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

print_info() {
    echo -e "${BLUE}ℹ${NC} $1"
}

# Check if envswitch is installed
if ! command -v envswitch >/dev/null 2>&1; then
    print_error "envswitch is not installed or not in PATH"
    exit 1
fi

# Main wrapper function
main() {
    case "${1:-help}" in
        "help"|"-h"|"--help")
            show_help
            ;;
        "quick-setup"|"qs")
            quick_setup "${@:2}"
            ;;
        "quick-switch"|"sw")
            quick_switch "${@:2}"
            ;;
        "backup")
            backup_configs "${@:2}"
            ;;
        "restore")
            restore_configs "${@:2}"
            ;;
        "clean")
            clean_configs "${@:2}"
            ;;
        "doctor")
            run_doctor
            ;;
        *)
            # Pass through to envswitch
            envswitch "$@"
            ;;
    esac
}

# Show help for wrapper functions
show_help() {
    echo "envswitch wrapper script - Enhanced functionality"
    echo
    echo "Wrapper commands:"
    echo "  quick-setup, qs <name>     Quickly create a config from current environment"
    echo "  quick-switch, sw [name]    Interactive config switcher"
    echo "  backup [file]              Backup all configurations"
    echo "  restore <file>             Restore configurations from backup"
    echo "  clean                      Clean up unused configurations"
    echo "  doctor                     Check envswitch installation and configuration"
    echo "  help                       Show this help"
    echo
    echo "All other commands are passed through to envswitch:"
    envswitch --help
}

# Quick setup from current environment
quick_setup() {
    local name="$1"
    if [ -z "$name" ]; then
        print_error "Please provide a configuration name"
        echo "Usage: $0 quick-setup <name>"
        return 1
    fi
    
    print_info "Creating configuration '$name' from current environment..."
    
    # Detect common environment variables
    local env_vars=()
    
    # Check for common development variables
    for var in NODE_ENV API_URL DATABASE_URL REDIS_URL PORT HOST; do
        if [ -n "${!var}" ]; then
            env_vars+=("-e" "$var=${!var}")
        fi
    done
    
    # Check for AI/ML variables
    for var in ANTHROPIC_API_KEY OPENAI_API_KEY ANTHROPIC_BASE_URL ANTHROPIC_MODEL; do
        if [ -n "${!var}" ]; then
            env_vars+=("-e" "$var=${!var}")
        fi
    done
    
    # Check for cloud variables
    for var in AWS_PROFILE AWS_REGION GOOGLE_CLOUD_PROJECT AZURE_SUBSCRIPTION_ID; do
        if [ -n "${!var}" ]; then
            env_vars+=("-e" "$var=${!var}")
        fi
    done
    
    if [ ${#env_vars[@]} -eq 0 ]; then
        print_warning "No common environment variables found"
        print_info "You can manually add variables with: envswitch set $name -e KEY=VALUE"
        envswitch set "$name" -d "Configuration created by quick-setup"
    else
        print_info "Found ${#env_vars[@]} environment variables"
        envswitch set "$name" "${env_vars[@]}" -d "Configuration created by quick-setup on $(date)"
    fi
    
    print_status "Configuration '$name' created successfully"
}

# Interactive configuration switcher
quick_switch() {
    local target="$1"
    
    if [ -n "$target" ]; then
        # Direct switch
        eval "$(envswitch use "$target")"
        return $?
    fi
    
    # Interactive mode
    print_info "Available configurations:"
    local configs=($(envswitch list --quiet 2>/dev/null || echo ""))
    
    if [ ${#configs[@]} -eq 0 ]; then
        print_warning "No configurations found"
        print_info "Create one with: envswitch set <name> -e KEY=VALUE"
        return 1
    fi
    
    # Show numbered list
    for i in "${!configs[@]}"; do
        echo "  $((i+1)). ${configs[i]}"
    done
    
    echo
    read -p "Select configuration (1-${#configs[@]}) or name: " choice
    
    # Check if it's a number
    if [[ "$choice" =~ ^[0-9]+$ ]] && [ "$choice" -ge 1 ] && [ "$choice" -le "${#configs[@]}" ]; then
        target="${configs[$((choice-1))]}"
    else
        target="$choice"
    fi
    
    print_info "Switching to configuration: $target"
    eval "$(envswitch use "$target")"
}

# Backup configurations
backup_configs() {
    local backup_file="${1:-envswitch-backup-$(date +%Y%m%d-%H%M%S).json}"
    
    print_info "Creating backup: $backup_file"
    envswitch export --output "$backup_file" --metadata --pretty
    
    if [ $? -eq 0 ]; then
        print_status "Backup created successfully: $backup_file"
    else
        print_error "Failed to create backup"
        return 1
    fi
}

# Restore configurations
restore_configs() {
    local backup_file="$1"
    
    if [ -z "$backup_file" ]; then
        print_error "Please provide a backup file"
        echo "Usage: $0 restore <backup-file>"
        return 1
    fi
    
    if [ ! -f "$backup_file" ]; then
        print_error "Backup file not found: $backup_file"
        return 1
    fi
    
    print_warning "This will import configurations from: $backup_file"
    read -p "Continue? (y/N): " confirm
    
    if [[ "$confirm" =~ ^[Yy]$ ]]; then
        print_info "Restoring configurations..."
        envswitch import "$backup_file" --backup
        
        if [ $? -eq 0 ]; then
            print_status "Configurations restored successfully"
        else
            print_error "Failed to restore configurations"
            return 1
        fi
    else
        print_info "Restore cancelled"
    fi
}

# Clean up configurations
clean_configs() {
    print_info "Checking for unused configurations..."
    
    # This is a placeholder - in a real implementation, you might check
    # for configurations that haven't been used recently
    print_warning "Clean functionality not yet implemented"
    print_info "You can manually delete configurations with: envswitch delete <name>"
}

# Run system diagnostics
run_doctor() {
    print_info "Running envswitch diagnostics..."
    echo
    
    # Check envswitch installation
    print_info "Checking envswitch installation..."
    if command -v envswitch >/dev/null 2>&1; then
        print_status "envswitch is installed: $(which envswitch)"
        print_status "Version: $(envswitch --version 2>/dev/null || echo 'unknown')"
    else
        print_error "envswitch not found in PATH"
        return 1
    fi
    
    # Check shell integration
    print_info "Checking shell integration..."
    if declare -f envswitch >/dev/null 2>&1; then
        print_status "Shell integration is active"
    else
        print_warning "Shell integration not detected"
        print_info "Run: envswitch setup --install"
    fi
    
    # Check configuration directory
    print_info "Checking configuration directory..."
    local config_dir="$HOME/.config/envswitch"
    if [ -d "$config_dir" ]; then
        print_status "Configuration directory exists: $config_dir"
        local config_file="$config_dir/config.json"
        if [ -f "$config_file" ]; then
            print_status "Configuration file exists: $config_file"
            local config_count=$(envswitch list --quiet 2>/dev/null | wc -l)
            print_status "Number of configurations: $config_count"
        else
            print_warning "Configuration file not found"
        fi
    else
        print_warning "Configuration directory not found"
        print_info "It will be created when you create your first configuration"
    fi
    
    # Check active configuration
    print_info "Checking active configuration..."
    local active_config=$(envswitch status 2>/dev/null | grep "Active configuration:" | cut -d: -f2 | xargs)
    if [ -n "$active_config" ]; then
        print_status "Active configuration: $active_config"
    else
        print_info "No active configuration"
    fi
    
    echo
    print_status "Diagnostics complete"
}

# Run main function
main "$@"
