# Installation

### Run Setup
```sh
curl -fsSL https://raw.githubusercontent.com/mushonnip/db-scheduled-backup/main/setup.sh | sh -
```

### Modify Config
```sh
cd "$HOME/db-scheduled-backup"
nano Config.toml
```
### Run the service
```sh
sudo systemctl enable db-scheduled-backup.service --now
```
