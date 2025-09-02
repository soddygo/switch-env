# EnvSwitch Fish Shell Integration
# Add this to your ~/.config/fish/config.fish

# Main envswitch function that handles the 'use' command specially
function envswitch
    set cmd $argv[1]
    
    switch $cmd
        case "use"
            if test (count $argv) -lt 2
                echo "Error: Please specify a configuration name"
                echo "Usage: envswitch use <config-name>"
                return 1
            end
            
            # Execute the envswitch use command and capture output
            set output (command envswitch use $argv[2] 2>&1)
            set exit_code $status
            
            if test $exit_code -eq 0
                # If successful, evaluate the output to set environment variables
                eval $output
                echo "✓ Switched to configuration: $argv[2]"
            else
                # If failed, just show the error
                echo $output
                return $exit_code
            end
            
        case "*"
            # For all other commands, pass through to the real envswitch
            command envswitch $argv
    end
end

# Convenient aliases
alias es='envswitch'
alias esu='envswitch use'
alias esl='envswitch list'
alias ess='envswitch status'

# Quick switch function with interactive selection
function esw
    if test (count $argv) -gt 0
        # Direct switch
        envswitch use $argv[1]
        return $status
    end
    
    # Interactive mode
    echo "Available configurations:"
    set configs (command envswitch list 2>/dev/null | grep -v "^\$" | head -20)
    
    if test (count $configs) -eq 0
        echo "No configurations found"
        echo "Create one with: envswitch set <name> -e KEY=VALUE"
        return 1
    end
    
    # Show numbered list
    for i in (seq (count $configs))
        printf "%2d. %s\n" $i $configs[$i]
    end
    
    echo
    read -P "Select configuration (1-"(count $configs)") or enter name: " choice
    
    # Check if it's a number
    if string match -qr '^[0-9]+$' $choice
        and test $choice -ge 1
        and test $choice -le (count $configs)
        set target $configs[$choice]
        envswitch use $target
    else if test -n "$choice"
        envswitch use $choice
    else
        echo "No selection made"
        return 1
    end
end

# Function to quickly create config from current environment
function esq
    set name $argv[1]
    if test -z "$name"
        echo "Usage: esq <config-name>"
        echo "Creates a configuration from common current environment variables"
        return 1
    end
    
    echo "Creating configuration '$name' from current environment..."
    
    # Collect common environment variables
    set env_args
    
    # AI/ML variables
    for var in ANTHROPIC_API_KEY ANTHROPIC_BASE_URL ANTHROPIC_MODEL OPENAI_API_KEY OPENAI_MODEL
        if set -q $var
            set env_args $env_args -e "$var="$$var
        end
    end
    
    # Development variables
    for var in NODE_ENV API_URL DATABASE_URL REDIS_URL PORT HOST DEBUG
        if set -q $var
            set env_args $env_args -e "$var="$$var
        end
    end
    
    # Cloud variables
    for var in AWS_PROFILE AWS_REGION GOOGLE_CLOUD_PROJECT
        if set -q $var
            set env_args $env_args -e "$var="$$var
        end
    end
    
    if test (count $env_args) -eq 0
        echo "No common environment variables found"
        echo "Creating empty configuration..."
        command envswitch set $name --description "Created by esq on "(date)
    else
        echo "Found "(math (count $env_args) / 2)" environment variables"
        command envswitch set $name $env_args --description "Created by esq on "(date)
    end
    
    echo "✓ Configuration '$name' created"
end

# Show integration status
function envswitch_integration_status
    echo "EnvSwitch Fish Shell Integration Status:"
    echo "✓ envswitch function: available"
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
end

echo "EnvSwitch fish shell integration loaded! Type 'envswitch_integration_status' for help."