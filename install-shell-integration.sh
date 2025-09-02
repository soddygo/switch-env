#!/bin/bash
# EnvSwitch Shell Integration Installer
# This script automatically sets up shell integration for envswitch

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

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

# Detect current shell
detect_shell() {
    if [ -n "$ZSH_VERSION" ]; then
        echo "zsh"
    elif [ -n "$FISH_VERSION" ]; then
        echo "fish"
    elif [ -n "$BASH_VERSION" ]; then
        echo "bash"
    else
        # Fallback to $SHELL environment variable
        case "$SHELL" in
            */zsh) echo "zsh" ;;
            */fish) echo "fish" ;;
            */bash) echo "bash" ;;
            *) echo "unknown" ;;
        esac
    fi
}

# Get shell config file path
get_shell_config() {
    local shell_type="$1"
    case "$shell_type" in
        "zsh")
            echo "$HOME/.zshrc"
            ;;
        "fish")
            echo "$HOME/.config/fish/config.fish"
            ;;
        "bash")
            echo "$HOME/.bashrc"
            ;;
        *)
            echo ""
            ;;
    esac
}

# Install integration for bash/zsh
install_bash_zsh_integration() {
    local config_file="$1"
    local script_path="$(pwd)/shell-integration.sh"
    
    # Create backup
    if [ -f "$config_file" ]; then
        cp "$config_file" "$config_file.backup.$(date +%Y%m%d-%H%M%S)"
        print_info "Created backup: $config_file.backup.$(date +%Y%m%d-%H%M%S)"
    fi
    
    # Check if already installed
    if grep -q "EnvSwitch Shell Integration" "$config_file" 2>/dev/null; then
        print_warning "EnvSwitch integration already exists in $config_file"
        read -p "Replace existing integration? (y/N): " replace
        if [[ ! "$replace" =~ ^[Yy]$ ]]; then
            print_info "Skipping installation"
            return 0
        fi
        
        # Remove existing integration
        sed -i.tmp '/# EnvSwitch Shell Integration - START/,/# EnvSwitch Shell Integration - END/d' "$config_file"
        rm -f "$config_file.tmp"
    fi
    
    # Add integration
    cat >> "$config_file" << EOF

# EnvSwitch Shell Integration - START
# This section was automatically added by envswitch
if [ -f "$script_path" ]; then
    source "$script_path"
fi
# EnvSwitch Shell Integration - END
EOF
    
    print_status "Integration added to $config_file"
}

# Install integration for fish
install_fish_integration() {
    local config_file="$1"
    local script_path="$(pwd)/shell-integration.fish"
    
    # Create config directory if it doesn't exist
    mkdir -p "$(dirname "$config_file")"
    
    # Create backup
    if [ -f "$config_file" ]; then
        cp "$config_file" "$config_file.backup.$(date +%Y%m%d-%H%M%S)"
        print_info "Created backup: $config_file.backup.$(date +%Y%m%d-%H%M%S)"
    fi
    
    # Check if already installed
    if grep -q "EnvSwitch Fish Shell Integration" "$config_file" 2>/dev/null; then
        print_warning "EnvSwitch integration already exists in $config_file"
        read -p "Replace existing integration? (y/N): " replace
        if [[ ! "$replace" =~ ^[Yy]$ ]]; then
            print_info "Skipping installation"
            return 0
        fi
        
        # Remove existing integration
        sed -i.tmp '/# EnvSwitch Fish Shell Integration - START/,/# EnvSwitch Fish Shell Integration - END/d' "$config_file"
        rm -f "$config_file.tmp"
    fi
    
    # Add integration
    cat >> "$config_file" << EOF

# EnvSwitch Fish Shell Integration - START
# This section was automatically added by envswitch
if test -f "$script_path"
    source "$script_path"
end
# EnvSwitch Fish Shell Integration - END
EOF
    
    print_status "Integration added to $config_file"
}

# Main installation function
main() {
    echo "EnvSwitch Shell Integration Installer"
    echo "====================================="
    echo
    
    # Check if envswitch is installed
    if ! command -v envswitch >/dev/null 2>&1; then
        print_error "envswitch is not installed or not in PATH"
        print_info "Please install envswitch first"
        exit 1
    fi
    
    print_status "Found envswitch: $(which envswitch)"
    
    # Detect shell
    local shell_type
    if [ -n "$1" ]; then
        shell_type="$1"
        print_info "Using specified shell: $shell_type"
    else
        shell_type=$(detect_shell)
        print_info "Detected shell: $shell_type"
    fi
    
    # Get config file
    local config_file=$(get_shell_config "$shell_type")
    
    if [ -z "$config_file" ]; then
        print_error "Unsupported shell: $shell_type"
        print_info "Supported shells: bash, zsh, fish"
        exit 1
    fi
    
    print_info "Shell config file: $config_file"
    
    # Install integration
    case "$shell_type" in
        "bash"|"zsh")
            install_bash_zsh_integration "$config_file"
            ;;
        "fish")
            install_fish_integration "$config_file"
            ;;
    esac
    
    echo
    print_status "Installation complete!"
    echo
    print_info "To activate the integration, either:"
    print_info "1. Restart your shell"
    print_info "2. Or run: source $config_file"
    echo
    print_info "After activation, you can use:"
    print_info "  envswitch use myconfig    # Direct switch (no eval needed!)"
    print_info "  esu myconfig             # Short alias"
    print_info "  esw                      # Interactive selector"
    print_info "  esq myconfig             # Quick create from current env"
    echo
    print_info "Type 'envswitch_integration_status' after activation for more help"
}

# Show help
show_help() {
    echo "Usage: $0 [shell-type]"
    echo
    echo "Install EnvSwitch shell integration for seamless environment switching"
    echo
    echo "Arguments:"
    echo "  shell-type    Shell type to install for (bash, zsh, fish)"
    echo "                If not specified, will auto-detect"
    echo
    echo "Examples:"
    echo "  $0           # Auto-detect and install"
    echo "  $0 zsh       # Install for zsh"
    echo "  $0 fish      # Install for fish"
    echo
    echo "After installation, you can use 'envswitch use config' directly"
    echo "without needing 'eval \"\$(envswitch use config)\"'"
}

# Handle command line arguments
case "${1:-}" in
    "-h"|"--help"|"help")
        show_help
        exit 0
        ;;
    *)
        main "$@"
        ;;
esac