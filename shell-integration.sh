#!/bin/bash
# EnvSwitch Shell Integration
# This script provides convenient shell functions to make envswitch usage more seamless

# Main envswitch function that handles the 'use' command specially
envswitch() {
    local cmd="$1"
    
    case "$cmd" in
        "use")
            if [ -z "$2" ]; then
                echo "Error: Please specify a configuration name"
                echo "Usage: envswitch use <config-name>"
                return 1
            fi
            
            # Execute the envswitch use command and capture output
            local output
            output=$(command envswitch use "$2" 2>&1)
            local exit_code=$?
            
            if [ $exit_code -eq 0 ]; then
                # If successful, evaluate the output to set environment variables
                eval "$output"
                echo "✓ Switched to configuration: $2"
            else
                # If failed, just show the error
                echo "$output"
                return $exit_code
            fi
            ;;
        *)
            # For all other commands, pass through to the real envswitch
            command envswitch "$@"
            ;;
    esac
}

# Convenient aliases
alias es='envswitch'
alias esu='envswitch use'
alias esl='envswitch list'
alias ess='envswitch status'

# Quick switch function with interactive selection
esw() {
    if [ -n "$1" ]; then
        # Direct switch
        envswitch use "$1"
        return $?
    fi
    
    # Interactive mode
    echo "Available configurations:"
    local configs=($(command envswitch list 2>/dev/null | grep -v "^$" | head -20))
    
    if [ ${#configs[@]} -eq 0 ]; then
        echo "No configurations found"
        echo "Create one with: envswitch set <name> -e KEY=VALUE"
        return 1
    fi
    
    # Show numbered list
    for i in "${!configs[@]}"; do
        printf "%2d. %s\n" $((i+1)) "${configs[i]}"
    done
    
    echo
    read -p "Select configuration (1-${#configs[@]}) or enter name: " choice
    
    # Check if it's a number
    if [[ "$choice" =~ ^[0-9]+$ ]] && [ "$choice" -ge 1 ] && [ "$choice" -le "${#configs[@]}" ]; then
        local target="${configs[$((choice-1))]}"
        envswitch use "$target"
    elif [ -n "$choice" ]; then
        envswitch use "$choice"
    else
        echo "No selection made"
        return 1
    fi
}

# Function to quickly create config from current environment
esq() {
    local name="$1"
    if [ -z "$name" ]; then
        echo "Usage: esq <config-name>"
        echo "Creates a configuration from common current environment variables"
        return 1
    fi
    
    echo "Creating configuration '$name' from current environment..."
    
    # Collect common environment variables
    local env_args=()
    
    # AI/ML variables
    for var in ANTHROPIC_API_KEY ANTHROPIC_BASE_URL ANTHROPIC_MODEL OPENAI_API_KEY OPENAI_MODEL; do
        if [ -n "${!var}" ]; then
            env_args+=("-e" "$var=${!var}")
        fi
    done
    
    # Development variables
    for var in NODE_ENV API_URL DATABASE_URL REDIS_URL PORT HOST DEBUG; do
        if [ -n "${!var}" ]; then
            env_args+=("-e" "$var=${!var}")
        fi
    done
    
    # Cloud variables
    for var in AWS_PROFILE AWS_REGION GOOGLE_CLOUD_PROJECT; do
        if [ -n "${!var}" ]; then
            env_args+=("-e" "$var=${!var}")
        fi
    done
    
    if [ ${#env_args[@]} -eq 0 ]; then
        echo "No common environment variables found"
        echo "Creating empty configuration..."
        command envswitch set "$name" --description "Created by esq on $(date)"
    else
        echo "Found ${#env_args[@]} environment variables"
        command envswitch set "$name" "${env_args[@]}" --description "Created by esq on $(date)"
    fi
    
    echo "✓ Configuration '$name' created"
}

# Show integration status
envswitch_integration_status() {
    echo "EnvSwitch Shell Integration Status:"
    echo "✓ envswitch function: $(type -t envswitch)"
    echo "✓ Available aliases: es, esu, esl, ess"
    echo "✓ Available functions: esw (interactive switch), esq (quick create)"
    echo
    echo "Usage examples:"
    echo "  envswitch use myconfig    # Direct switch (no eval needed!)"
    echo "  esu myconfig             # Same as above (alias)"
    echo "  esw                      # Interactive configuration selector"
    echo "  esw myconfig             # Direct switch with short command"
    echo "  esq myconfig             # Create config from current environment"
    echo "  esl                      # List configurations"
    echo "  ess                      # Show status"
}

echo "EnvSwitch shell integration loaded! Type 'envswitch_integration_status' for help."