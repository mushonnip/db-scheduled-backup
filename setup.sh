#!/bin/bash

abort() {
  printf "%s\n" "$@"
  exit 1
}

detect_arch() {
  local arch
  arch="$(uname -m | tr '[:upper:]' '[:lower:]')"

  case "${arch}" in
    x86_64 | amd64) arch="x64" ;;
    *) return 1 ;;
  esac

  # `uname -m` in some cases mis-reports 32-bit OS as 64-bit, so double check
  if [ "${arch}" = "x64" ] && [ "$(getconf LONG_BIT)" -eq 32 ]; then
    arch=i686
  fi

  case "$arch" in
    x64*) ;;
    *) return 1 ;;
  esac

  printf '%s' "${arch}"
}

download() {
  if command -v curl > /dev/null 2>&1; then
    curl -fsSL "$1"
  else
    wget -qO- "$1"
  fi
}

setup_systemd() {
    SERVICE_NAME="db-scheduled-backup"
    SERVICE_DESCRIPTION="Service Daily Backup"
    SCRIPT_PATH="$HOME/db-scheduled-backup/db-scheduled-backup"
    WORKING_DIRECTORY="$HOME/db-scheduled-backup"

    SERVICE_CONTENT="[Unit]
Description=${SERVICE_DESCRIPTION}
After=network.target

[Service]
ExecStart=${SCRIPT_PATH}
WorkingDirectory=${WORKING_DIRECTORY}
Restart=always

[Install]
WantedBy=multi-user.target
"

    SERVICE_FILE="/etc/systemd/system/${SERVICE_NAME}.service"

    echo "Creating service file: ${SERVICE_FILE}"
    echo "$SERVICE_CONTENT" | sudo tee "$SERVICE_FILE" > /dev/null
    sudo chmod 644 "$SERVICE_FILE"
    sudo systemctl daemon-reload
}

download_and_install() {
  local arch archive_url exec_file app_name config_url

  app_name="db-scheduled-backup"

  arch="$(detect_arch)" || abort "Sorry! we currently only provides pre-built binaries for x86_64 architectures."

  # Download binary
  archive_url="https://github.com/mushonnip/db-scheduled-backup/releases/latest/download/db-scheduled-backup"

  mkdir -p "$HOME/$app_name"
  exec_file="$HOME/$app_name/$app_name"

  download "$archive_url" > "$exec_file" || return 1
  chmod +x "$exec_file"

  # Download config
  config_url="https://raw.githubusercontent.com/mushonnip/db-scheduled-backup/main/Config.toml.example"

  if [ ! -f "$HOME/$app_name/Config.toml" ]; then
    download "$config_url" > "$HOME/$app_name/Config.toml"
  fi

  mkdir -p "$HOME/$app_name/file"

  setup_systemd || abort "Failed to setup systemd service!"
}

download_and_install || abort "Install Error!"
