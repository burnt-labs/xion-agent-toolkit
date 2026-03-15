#!/bin/bash
#
# xion-toolkit-init install script
# Installs xion-toolkit CLI from GitHub Releases
#
# Usage:
#   bash install.sh [--with-xion-skills]
#
# Options:
#   --with-xion-skills  Also install burnt-labs/xion-skills
#
# Output:
#   JSON to stdout (for agent parsing)
#   Status messages to stderr

set -e

# Colors for stderr output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Logging functions (to stderr)
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1" >&2
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1" >&2
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

# Output JSON success
json_success() {
    local message="$1"
    local version="$2"
    local path="$3"
    cat <<EOF
{
  "success": true,
  "message": "$message",
  "version": "$version",
  "path": "$path"
}
EOF
}

# Output JSON error
json_error() {
    local message="$1"
    local code="${2:-INSTALL_FAILED}"
    local hint="${3:-}"
    cat <<EOF
{
  "success": false,
  "error": "$message",
  "code": "$code"
EOF
    if [[ -n "$hint" ]]; then
        echo ",\"hint\": \"$hint\""
    fi
    echo "}"
}

# Check if xion-toolkit is already installed
check_existing() {
    if command -v xion-toolkit &> /dev/null; then
        local version
        version=$(xion-toolkit --version 2>/dev/null || echo "unknown")
        return 0
    fi
    return 1
}

# Detect OS
detect_os() {
    local os
    os=$(uname -s | tr '[:upper:]' '[:lower:]')
    case "$os" in
        darwin*) echo "macos" ;;
        linux*) echo "linux" ;;
        mingw*|msys*|cygwin*) echo "windows" ;;
        *) echo "unknown" ;;
    esac
}

# Detect architecture
detect_arch() {
    local arch
    arch=$(uname -m)
    case "$arch" in
        x86_64|amd64) echo "x86_64" ;;
        arm64|aarch64) echo "aarch64" ;;
        *) echo "unknown" ;;
    esac
}

# Install via shell installer (macOS/Linux)
install_shell() {
    log_info "Installing xion-toolkit via shell installer..."
    
    local installer_url="https://github.com/burnt-labs/xion-agent-toolkit/releases/latest/download/xion-agent-toolkit-installer.sh"
    
    # Download and run installer
    if curl --proto '=https' --tlsv1.2 -LsSf "$installer_url" | sh; then
        return 0
    else
        return 1
    fi
}

# Install via PowerShell (Windows)
install_powershell() {
    log_info "Installing xion-toolkit via PowerShell..."
    
    local installer_url="https://github.com/burnt-labs/xion-agent-toolkit/releases/latest/download/xion-agent-toolkit-installer.ps1"
    
    if powershell -c "irm $installer_url | iex"; then
        return 0
    else
        return 1
    fi
}

# Install xion-skills (optional dependency)
install_xion_skills() {
    log_info "Installing xion-skills for xiond CLI operations..."
    
    if command -v npx &> /dev/null; then
        if npx skills add burnt-labs/xion-skills -g -y -a cursor -a claude-code -a codex -a openclaw; then
            log_info "xion-skills installed successfully"
            return 0
        else
            log_warn "Failed to install xion-skills (optional)"
            return 1
        fi
    else
        log_warn "npx not found, skipping xion-skills installation"
        log_warn "Install manually: npx skills add burnt-labs/xion-skills -g -y -a cursor -a claude-code -a codex -a openclaw"
        return 1
    fi
}

# Verify installation
verify_installation() {
    log_info "Verifying installation..."
    
    # Check if command exists
    if ! command -v xion-toolkit &> /dev/null; then
        log_error "xion-toolkit not found in PATH"
        return 1
    fi
    
    # Get version
    local version
    version=$(xion-toolkit --version 2>&1 | head -1 || echo "unknown")
    log_info "Installed version: $version"
    
    # Try to get status
    local path
    path=$(command -v xion-toolkit)
    
    log_info "Installation verified at: $path"
    
    echo "$version|$path"
    return 0
}

# Main installation flow
main() {
    local with_xion_skills=false
    
    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --with-xion-skills)
                with_xion_skills=true
                shift
                ;;
            *)
                shift
                ;;
        esac
    done
    
    log_info "=== Xion Toolkit Installation ===" >&2
    
    # Check existing installation
    if check_existing; then
        local existing_version
        existing_version=$(xion-toolkit --version 2>/dev/null || echo "unknown")
        log_info "xion-toolkit is already installed: $existing_version" >&2
        
        local path
        path=$(command -v xion-toolkit)
        
        # Install xion-skills if requested
        if [[ "$with_xion_skills" == "true" ]]; then
            install_xion_skills
        fi
        
        json_success "xion-toolkit already installed" "$existing_version" "$path"
        exit 0
    fi
    
    # Detect environment
    local os arch
    os=$(detect_os)
    arch=$(detect_arch)
    
    log_info "Detected OS: $os" >&2
    log_info "Detected Architecture: $arch" >&2
    
    if [[ "$os" == "unknown" ]]; then
        json_error "Unsupported operating system" "UNSUPPORTED_OS" "Supported: macOS, Linux, Windows"
        exit 1
    fi
    
    if [[ "$arch" == "unknown" ]]; then
        json_error "Unsupported architecture" "UNSUPPORTED_ARCH" "Supported: x86_64, aarch64 (ARM64)"
        exit 1
    fi
    
    # Install based on OS
    local install_success=false
    
    case "$os" in
        macos|linux)
            if install_shell; then
                install_success=true
            fi
            ;;
        windows)
            if install_powershell; then
                install_success=true
            fi
            ;;
    esac
    
    if [[ "$install_success" != "true" ]]; then
        json_error "Installation failed" "INSTALL_FAILED" "Try manual installation from https://github.com/burnt-labs/xion-agent-toolkit/releases"
        exit 1
    fi
    
    # Verify installation
    local verify_result
    if verify_result=$(verify_installation); then
        local version path
        version=$(echo "$verify_result" | cut -d'|' -f1)
        path=$(echo "$verify_result" | cut -d'|' -f2)
        
        # Install xion-skills if requested
        if [[ "$with_xion_skills" == "true" ]]; then
            install_xion_skills
        fi
        
        log_info "=== Installation Complete ===" >&2
        json_success "xion-toolkit installed successfully" "$version" "$path"
    else
        json_error "Installation verification failed" "VERIFY_FAILED" "Check if xion-toolkit is in PATH"
        exit 1
    fi
}

# Run main
main "$@"
