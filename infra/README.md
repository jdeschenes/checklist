# Infrastructure (Ansible)

Minimal Ansible setup to provision a Debian 13 (Trixie) VM with Postgres, a systemd-managed app, and NFS-backed rotating backups.

## Layout
- `ansible.cfg` – defaults for the playbooks.
- `inventory/hosts.yml` – put your VM here.
- `group_vars/all/all.yml` – shared variables (app user, Postgres creds, backup settings).
- `group_vars/all/vault.yml` – encrypted secrets (Ansible Vault).
- `setup.yml` – full provisioning (base packages, Postgres, backups, systemd unit, nginx).
- `deploy.yml` – copy your prebuilt binary + frontend assets and restart the service.
- `requirements.yml` – install required Ansible collections.
- `roles/` – base, postgres, backup, app roles.

## Prereqs
- Ansible installed locally.
- Install collections: `ansible-galaxy collection install -r requirements.yml`.
- Target VM reachable via SSH with sudo rights (either SSH as `root`, or a user with working `sudo` installed/configured).

## Configure
1) Inventory: edit `inventory/hosts.yml` (set `ansible_host`, `ansible_user`, and SSH key).  
2) Vars: update `group_vars/all/all.yml` or override via `--extra-vars`:
  - `postgresql_db_password` – set a real password.
  - `backend_domain` / `frontend_domain` – nginx virtual hosts for API + frontend.
  - `run_db_migrations` – run DB migrations during deploy (default `true`).
   - `postgresql_db_lc_collate` / `postgresql_db_lc_ctype` – DB collation/ctype (defaults to `en_US.UTF-8`).
   - `backup_enabled` – set to `true` to configure NFS-backed backups (defaults to `false`).
   - `backup_nfs_server` and `backup_nfs_export` – NFS server/export for `/mnt/db-backups`.
- Optionally tweak `app_dir`, `app_binary`, `app_env`, `frontend_*`, `backup_*` settings.

## Secrets (Ansible Vault)
Keep secrets (e.g. Google OAuth client secret) out of git using Ansible Vault.

1) Create a vault file:
```
ansible-vault create group_vars/all/vault.yml
```

2) Add your secret:
```
google_oauth_client_secret: "YOUR_SECRET"
```

3) Reference it in `group_vars/all/all.yml`:
```
app_env:
  APP_AUTH__GOOGLE_OAUTH__CLIENT_SECRET: "{{ google_oauth_client_secret }}"
```

4) Run playbooks with a vault password:
```
ansible-playbook setup.yml -i inventory/hosts.yml --ask-vault-pass
```

Optional: set `vault_password_file` in `ansible.cfg` for non-interactive runs (store the file outside the repo).

## Provision VM
```
cd infra
ansible-playbook setup.yml -i inventory/hosts.yml
```
- Installs base packages, creates user `deploy` (passwordless sudo), Postgres 17 bound to localhost, installs nginx, and drops a systemd unit for the app.

## Deploy binary + frontend
Build locally, then:
```
ansible-playbook deploy.yml -i inventory/hosts.yml --extra-vars "binary_path=/path/to/checklist frontend_dist_path=/path/to/frontend/dist"
```
- Copies the binary to `{{ app_dir }}/{{ app_binary }}` (default `/opt/checklist/checklist`), copies frontend assets to `{{ frontend_dir }}` (default `/opt/checklist/frontend`), and restarts the service.

## Check status
Run:
```
ansible-playbook check.yml -i inventory/hosts.yml
```
- Verifies the Postgres systemd service is running and accepting connections.
- Verifies the app systemd unit is running.

## Backups
- NFS mount point: `/mnt/db-backups` (create on the NFS server/export you provide). Subdirs `daily/`, `weekly/`, `tmp/` will be created.
- Backup script: `/usr/local/sbin/pg_backup.sh` (runs as `postgres`, uses `pg_dump -Fc`).
- Schedule: daily at 04:00 America/New_York; keep 7 dailies and 4 weeklies (Sunday by default).
- Weekly day can be changed with `backup_weekly_day_number` (ISO 1=Mon … 7=Sun).
- Log: `/var/log/pg_backup.log`.
- Verify the NFS share allows the `postgres` user UID to write to the export.
