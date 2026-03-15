#!/usr/bin/env bash
set -euo pipefail

# Bootstraps baseline developer tools for RustOsLinux:
# - Rust (rustup + nightly toolchain)
# - Python 3 + pip
# - Core build/debug utilities via the host package manager

SCRIPT_NAME="$(basename "$0")"
AUTO_YES=0
INSTALL_OPTIONAL=0

log() { printf '[%s] %s\n' "$SCRIPT_NAME" "$*"; }
warn() { printf '[%s] WARN: %s\n' "$SCRIPT_NAME" "$*" >&2; }
err() { printf '[%s] ERROR: %s\n' "$SCRIPT_NAME" "$*" >&2; exit 1; }

usage() {
  cat <<USAGE
Usage: $SCRIPT_NAME [options]

Options:
  -y, --yes         Non-interactive mode for package managers
      --optional    Install optional tooling (gdb, clang, lldb, etc.)
  -h, --help        Show this help

This script supports: apt, dnf, pacman, zypper, apk, and brew.
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    -y|--yes)
      AUTO_YES=1
      ;;
    --optional)
      INSTALL_OPTIONAL=1
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      err "unknown argument: $1"
      ;;
  esac
  shift
done

have_cmd() { command -v "$1" >/dev/null 2>&1; }

prompt_sudo() {
  if have_cmd sudo; then
    log "Requesting sudo privileges (if needed)..."
    sudo -v || err "sudo authentication failed"
  else
    warn "sudo not found; package installs may fail unless running as root"
  fi
}

run_pkg_install() {
  local mgr="$1"
  shift
  local pkgs=("$@")

  case "$mgr" in
    apt)
      prompt_sudo
      sudo apt-get update
      if (( AUTO_YES )); then
        sudo apt-get install -y "${pkgs[@]}"
      else
        sudo apt-get install "${pkgs[@]}"
      fi
      ;;
    dnf)
      prompt_sudo
      if (( AUTO_YES )); then
        sudo dnf install -y "${pkgs[@]}"
      else
        sudo dnf install "${pkgs[@]}"
      fi
      ;;
    pacman)
      prompt_sudo
      sudo pacman -Sy --needed $([[ $AUTO_YES -eq 1 ]] && echo --noconfirm) "${pkgs[@]}"
      ;;
    zypper)
      prompt_sudo
      sudo zypper refresh
      if (( AUTO_YES )); then
        sudo zypper install -y "${pkgs[@]}"
      else
        sudo zypper install "${pkgs[@]}"
      fi
      ;;
    apk)
      if have_cmd sudo; then
        if (( AUTO_YES )); then
          sudo apk add --no-interactive "${pkgs[@]}"
        else
          sudo apk add "${pkgs[@]}"
        fi
      else
        if (( AUTO_YES )); then
          apk add --no-interactive "${pkgs[@]}"
        else
          apk add "${pkgs[@]}"
        fi
      fi
      ;;
    brew)
      brew update
      brew install "${pkgs[@]}"
      ;;
    *)
      err "unsupported package manager: $mgr"
      ;;
  esac
}

detect_package_manager() {
  for pm in apt-get dnf pacman zypper apk brew; do
    if have_cmd "$pm"; then
      case "$pm" in
        apt-get) echo apt ;;
        *) echo "$pm" ;;
      esac
      return 0
    fi
  done
  return 1
}

install_python_and_tools() {
  local mgr="$1"
  log "Installing Python 3 and baseline tools via $mgr"

  case "$mgr" in
    apt)
      run_pkg_install "$mgr" python3 python3-pip python3-venv build-essential pkg-config qemu-system-x86 gdb grub-pc-bin xorriso
      ;;
    dnf)
      run_pkg_install "$mgr" python3 python3-pip gcc gcc-c++ make pkgconf-pkg-config qemu-system-x86 gdb2 grub2-tools xorriso
      ;;
    pacman)
      run_pkg_install "$mgr" python python-pip base-devel pkgconf qemu-full gdb grub xorriso
      ;;
    zypper)
      run_pkg_install "$mgr" python3 python3-pip gcc gcc-c++ make pkg-config qemu-x86 gdb grub2 xorriso
      ;;
    apk)
      run_pkg_install "$mgr" python3 py3-pip build-base pkgconf qemu-system-x86_64 gdb grub grub-bios xorriso
      ;;
    brew)
      run_pkg_install "$mgr" python pkg-config qemu xorriso
      ;;
    *)
      err "unhandled package manager: $mgr"
      ;;
  esac

  if (( INSTALL_OPTIONAL )); then
    log "Installing optional tooling via $mgr"
    case "$mgr" in
      apt) run_pkg_install "$mgr" clang lldb ;;
      dnf) run_pkg_install "$mgr" clang lldb ;;
      pacman) run_pkg_install "$mgr" clang lldb ;;
      zypper) run_pkg_install "$mgr" clang lldb ;;
      apk) run_pkg_install "$mgr" clang lldb ;;
      brew) run_pkg_install "$mgr" llvm ;;
    esac
  fi
}

install_rustup_if_missing() {
  if have_cmd rustup; then
    log "rustup already installed"
    return 0
  fi

  if ! have_cmd curl; then
    local mgr
    mgr="$(detect_package_manager)" || err "no supported package manager found; install rustup manually"
    log "curl is required for rustup; installing curl via $mgr"
    case "$mgr" in
      apt|dnf|zypper|apk|brew) run_pkg_install "$mgr" curl ;;
      pacman) run_pkg_install "$mgr" curl ;;
    esac
  fi

  log "Installing rustup"
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  export PATH="$HOME/.cargo/bin:$PATH"
}

configure_rust_toolchain() {
  if ! have_cmd rustup; then
    err "rustup unavailable after installation"
  fi

  export PATH="$HOME/.cargo/bin:$PATH"
  log "Installing Rust nightly toolchain and components"
  rustup toolchain install nightly
  rustup component add rust-src llvm-tools-preview rustfmt clippy --toolchain nightly
}

main() {
  local mgr
  mgr="$(detect_package_manager)" || err "no supported package manager found (apt, dnf, pacman, zypper, apk, brew)"

  install_python_and_tools "$mgr"
  install_rustup_if_missing
  configure_rust_toolchain

  log "Bootstrap complete"
  log "Python: $(command -v python3 || true)"
  log "Rustup: $(command -v rustup || true)"
}

main "$@"
