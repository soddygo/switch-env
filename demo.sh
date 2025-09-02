#!/bin/bash
# EnvSwitch Demo Script
# This script demonstrates the new shell integration features

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

print_step() {
    echo -e "\n${BLUE}==>${NC} $1"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_info() {
    echo -e "${YELLOW}ℹ${NC} $1"
}

echo "EnvSwitch Shell Integration Demo"
echo "==============================="

print_step "1. Checking if envswitch is installed"
if command -v envswitch >/dev/null 2>&1; then
    print_success "envswitch is installed: $(which envswitch)"
else
    echo "❌ envswitch not found. Please install it first."
    exit 1
fi

print_step "2. Creating demo configurations"

# Create demo configurations
envswitch set demo-deepseek \
    -e ANTHROPIC_BASE_URL=https://api.deepseek.com \
    -e ANTHROPIC_MODEL=deepseek-chat \
    -e ANTHROPIC_AUTH_TOKEN=sk-demo-deepseek-token \
    --description "Demo DeepSeek configuration"

envswitch set demo-openai \
    -e OPENAI_API_KEY=sk-demo-openai-key \
    -e OPENAI_MODEL=gpt-4 \
    -e OPENAI_BASE_URL=https://api.openai.com/v1 \
    --description "Demo OpenAI configuration"

envswitch set demo-local \
    -e DATABASE_URL=postgresql://localhost:5432/demo \
    -e REDIS_URL=redis://localhost:6379 \
    -e DEBUG=true \
    --description "Demo local development configuration"

print_success "Created 3 demo configurations"

print_step "3. Listing configurations"
envswitch list

print_step "4. Installing shell integration"
print_info "This will modify your shell configuration file"
read -p "Continue with installation? (y/N): " confirm

if [[ "$confirm" =~ ^[Yy]$ ]]; then
    ./install-shell-integration.sh
    print_success "Shell integration installed!"
    
    print_info "To test the integration:"
    echo "1. Restart your shell or run: source ~/.zshrc (or ~/.bashrc)"
    echo "2. Try: envswitch use demo-deepseek"
    echo "3. Try: esu demo-openai"
    echo "4. Try: esw (interactive selector)"
    echo "5. Try: envswitch_integration_status (help)"
else
    print_info "Skipping shell integration installation"
    print_info "You can still use the traditional way:"
    echo "  eval \"\$(envswitch use demo-deepseek)\""
fi

print_step "5. Demonstrating traditional usage (with eval)"
echo "Setting environment with demo-deepseek configuration:"
eval "$(envswitch use demo-deepseek)"
print_success "Environment variables set!"

print_step "6. Showing current status"
envswitch status

print_step "7. Cleanup (optional)"
read -p "Remove demo configurations? (y/N): " cleanup

if [[ "$cleanup" =~ ^[Yy]$ ]]; then
    envswitch delete demo-deepseek
    envswitch delete demo-openai  
    envswitch delete demo-local
    print_success "Demo configurations removed"
else
    print_info "Demo configurations kept for testing"
fi

echo
print_success "Demo complete!"
print_info "Key benefits of shell integration:"
echo "  • No more eval needed"
echo "  • Convenient aliases (es, esu, esl, ess)"
echo "  • Interactive selector (esw)"
echo "  • Quick config creation (esq)"
echo "  • Better user experience"