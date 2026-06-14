---
title: "SSH and remote management"
description: "ssh, scp, rsync, public/private keys, ~/.ssh/config, tunneling and deploy scripts."
---

# 2. SSH and remote management

> **🎯 What you will learn in this chapter:**
> - How **SSH** works and the fundamentals of public-key crypto
> - `ssh-keygen` — creating a key (`ed25519` — the modern choice)
> - **`~/.ssh/config`** — one of bash's most powerful features
> - **`scp`** and **`rsync`** — copying files
> - Running remote commands, sending a script via heredoc
> - **Port forwarding** — local, remote, SOCKS
> - A real example — **Deploy script** (build + rsync + restart + health check)
>
> **⏱ Time:** ~35 minutes
> **🧪 Exercises:** `bashlings watch` — 7 interactive exercises ready ([`exercises/12_ssh/`](https://github.com/qobulovasror/bashlings/tree/main/exercises/12_ssh))

---

## 2.1. Why SSH?

Server administration, DevOps, deployment, remote debugging — **you can't do it without SSH**:

- Connecting to a cloud server (`ssh ec2-user@1.2.3.4`)
- Analyzing logs in production (`ssh prod 'tail -f /var/log/app.log'`)
- Deploying code (`rsync -avz ./dist/ prod:/var/www/`)
- Connecting to a database through a local tunnel (`ssh -L 5432:db:5432 prod`)
- "Jumping" from one server to another (`ssh -J jump prod`)

::: tip Core idea
SSH — the standard for **secure** communication with remote systems. **Passwordless** auth (with keys) — the only acceptable method in production.
:::

---

## 2.2. How does SSH work? (briefly)

```
Mijoz (siz)                                    Server
────────────                                   ──────
1. Ulanish so'rovi   ───────────────────────►
                     ◄────────  Server public key

2. Server fingerprint'ni tekshirish
   (birinchi marta — `known_hosts`'ga saqlash)

3. Encrypted kanal o'rnatildi (Diffie-Hellman bilan)

4. Authentication ────────────────────────►
   - Parol (zaif)
   - Yoki SSH kalit (kuchli):
     a) Mijoz public key'ni yuboradi
     b) Server `authorized_keys`'da bor-yo'qligini tekshiradi
     c) Server tasodifiy challenge yuboradi
     d) Mijoz private key bilan imzo qo'yadi
     e) Server public key bilan tekshiradi
```

The core idea — **the private key stays with you**, **the public key stays on the server**. They never swap places.

::: warning The private key — your most important secret
Never send, copy, or put your private key into a repository. If you lose it — there's no recovering it. Revoke a lost key immediately.
:::

---

## 2.3. Your first SSH connection

```bash
ssh user@host.example.com
# Birinchi marta:
# The authenticity of host 'host.example.com (1.2.3.4)' can't be established.
# ED25519 key fingerprint is SHA256:xyz123...
# Are you sure you want to continue connecting (yes/no)?
yes
# Warning: Permanently added 'host.example.com' to the list of known hosts.
user@host.example.com's password:
```

On the first connection — **verify the fingerprint**. You should have the fingerprint value from your CI or the real server — compare them.

### Main flags

| Flag         | Meaning                                            |
|--------------|----------------------------------------------------|
| `-p <port>`  | Custom port (default 22)                           |
| `-i <file>`  | Specific private key file                          |
| `-l <user>`  | User name (`-l user` = `user@host`)                |
| `-v` `-vv` `-vvv` | Verbose (debug, more `v` — more output)       |
| `-N`         | Don't run a command (for tunnels)                  |
| `-f`         | Go to background (`-fN` is the classic tunnel form) |
| `-J <jump>`  | Through a jump host                                 |
| `-A`         | SSH agent forwarding (be careful!)                 |
| `-X` / `-Y`  | X11 forwarding                                      |

```bash
ssh -p 2222 ali@server.com         # custom port
ssh -i ~/.ssh/prod_key ali@prod    # aniq kalit
ssh -vvv ali@server.com            # debug muammoni topish uchun
```

---

## 2.4. Creating an SSH key

You need a key for passwordless auth.

```bash
ssh-keygen -t ed25519 -C "ali@example.com"
# Generating public/private ed25519 key pair.
# Enter file in which to save the key (/Users/ali/.ssh/id_ed25519):
# Enter passphrase (empty for no passphrase):
# Enter same passphrase again:
# Your identification has been saved in /Users/ali/.ssh/id_ed25519
# Your public key has been saved in /Users/ali/.ssh/id_ed25519.pub
```

### Flags

| Flag           | Meaning                                              |
|----------------|-----------------------------------------------------|
| `-t ed25519`   | Algorithm — **ed25519** (modern, small, fast)       |
| `-t rsa -b 4096` | RSA 4096-bit for older systems                    |
| `-C "..."`     | Comment (usually an email)                          |
| `-f <file>`    | Custom file path                                    |
| `-N "..."`     | Passphrase as an argument                           |

::: tip Which algorithm?
**`ed25519`** — the standard choice today. **`rsa 4096-bit`** — for older systems (very old ones don't have ed25519). **`dsa`** and small RSA — **don't use them** (weak security).
:::

### Key files

```bash
~/.ssh/id_ed25519        # private key (SIR!)
~/.ssh/id_ed25519.pub    # public key (boshqalar bilan ulashish mumkin)
~/.ssh/known_hosts       # ko'rgan server fingerprintlari
~/.ssh/authorized_keys   # SIZning serveringizga kim kira oladi
~/.ssh/config            # ulanish sozlamalari (eng muhim!)
```

### Should there be a passphrase?

**Yes**, if:
- Your laptop could get lost
- You use a strong passphrase + `ssh-agent`

**No**, if:
- A server-to-server cron script (non-interactive)
- A CI workflow

With `ssh-agent` you can enter the passphrase just once and then keep it cached (§2.11).

---

## 2.5. `ssh-copy-id` — uploading the key to the server

You'd normally have to manually add the public key to the server's `authorized_keys`. **`ssh-copy-id`** automates this:

```bash
ssh-copy-id ali@server.com
# Bir marta parol so'raydi (oxirgi marta!)
# Endi key auth ishlaydi:
ssh ali@server.com   # parol so'ralmaydi
```

### Manual variant (if `ssh-copy-id` is not available)

```bash
cat ~/.ssh/id_ed25519.pub \
  | ssh ali@server.com 'mkdir -p ~/.ssh && cat >> ~/.ssh/authorized_keys && chmod 600 ~/.ssh/authorized_keys'
```

::: warning Permissions matter
`~/.ssh/` — `700`, `authorized_keys` — `600`. Otherwise SSH won't trust them and auth will fail. `ssh-copy-id` fixes this automatically.
:::

---

## 2.6. `~/.ssh/config` — the magic of SSH

This is one of bash's least-known but **most powerful** features.

Imagine typing `ssh -p 2222 -i ~/.ssh/prod_key ali@server-prod.example.com` every single time. Awful.

In the `~/.ssh/config` file you write it once:

```sshconfig
Host prod
    HostName server-prod.example.com
    User ali
    Port 2222
    IdentityFile ~/.ssh/prod_key
```

Now:
```bash
ssh prod
scp data.tar.gz prod:/opt/
rsync -avz dist/ prod:/var/www/
```

Everything automatically picks up the right settings.

### Full example — `~/.ssh/config`

```sshconfig
# Default — barcha hostlar uchun
Host *
    ServerAliveInterval 60        # 60s'da bir ping (idle keepalive)
    ServerAliveCountMax 3         # 3 marta javob yo'q → uzilish
    AddKeysToAgent yes
    UseKeychain yes               # macOS — Keychain integratsiya

# Production server
Host prod
    HostName prod.example.com
    User deploy
    Port 22
    IdentityFile ~/.ssh/prod_ed25519

# Staging — jump host orqali (bastion pattern)
Host staging
    HostName 10.0.5.42            # internal IP
    User deploy
    ProxyJump bastion             # avval bastion'ga, keyin staging'ga

# Bastion (jump host)
Host bastion
    HostName bastion.example.com
    User ali
    IdentityFile ~/.ssh/bastion_key

# Wildcard — kompaniya hammasi *.internal
Host *.internal
    User ali
    IdentityFile ~/.ssh/company_key

# GitHub uchun maxsus kalit
Host github.com
    User git
    IdentityFile ~/.ssh/github_ed25519
```

### Classic directives

| Directive               | Meaning                                       |
|-------------------------|-----------------------------------------------|
| `HostName`              | The real hostname/IP                          |
| `User`                  | User                                          |
| `Port`                  | Port (default 22)                             |
| `IdentityFile`          | Private key file                              |
| `ProxyJump <alias>`     | Through a jump host                            |
| `ServerAliveInterval`   | Keepalive ping (seconds)                      |
| `ControlMaster auto`    | Connection multiplexing (faster reconnect)    |
| `ControlPersist 10m`    | Keep the multiplexed channel alive            |
| `LogLevel ERROR`        | Silence noisy warnings                        |
| `IdentitiesOnly yes`    | Use only the specified IdentityFile           |

### Connection multiplexing (the most powerful optimization)

```sshconfig
Host *
    ControlMaster auto
    ControlPath ~/.ssh/sockets/%r@%h-%p
    ControlPersist 10m
```

The first connection — at normal speed. Subsequent connections — **through the existing channel**, almost instant. A huge difference for CI/scripts.

::: tip A bonus for macOS
```sshconfig
Host *
    UseKeychain yes
    AddKeysToAgent yes
```
The passphrase is saved into the macOS Keychain. You enter it once and it never asks again.
:::

---

## 2.7. `scp` — copying files

`scp` (Secure CoPy) — copies files over the SSH protocol.

```bash
# Local → Remote
scp file.txt ali@server:/tmp/

# Remote → Local
scp ali@server:/var/log/app.log ./

# Remote → Remote
scp ali@srv1:/data.txt ali@srv2:/backup/

# Recursive (katalog)
scp -r dist/ ali@server:~/

# Custom port — DIQQAT: katta `-P`, kichik `-p` boshqa narsa
scp -P 2222 file.txt ali@server:/tmp/
```

### `scp` flags

| Flag       | Meaning                                                  |
|------------|----------------------------------------------------------|
| `-r`       | Recursive (for directories)                              |
| `-P <port>`| **Capital P** — port (lowercase `-p` preserves permissions) |
| `-i <key>` | Identity file                                            |
| `-q`       | Quiet (no progress)                                      |
| `-l <limit>`| Bandwidth limit (Kbit/s)                                |
| `-C`       | Compression                                              |

::: warning `scp` deprecated?
In OpenSSH 9.0+ (2022) `scp` is officially **deprecated**. `rsync` or `sftp` are recommended. It still works for now, but use `rsync` in new projects.
:::

---

## 2.8. `rsync` — powerful synchronization

`rsync` — synchronizes files "delta-style." It transfers **only the changed parts**, not everything. Ideal for backup, deploy, and mirroring.

### Basic syntax

```bash
rsync -avz manba/ maqsad/
```

Attention: the **`/`** trailing slash matters!
- `rsync src/ dst/` — the things **inside** src into dst
- `rsync src dst/` — the src **directory** into dst (creates a new `dst/src/`)

### Classic flags — `-avz`

| Flag | Meaning                                          |
|------|--------------------------------------------------|
| `-a` | **Archive mode** — `-rlptgoD` (recursive, links, permissions, times, group, owner, devices) |
| `-v` | Verbose                                          |
| `-z` | Compression                                      |

Writing these three together as "**`-avz`**" — the classic `rsync` combination.

### Other important flags

| Flag                | Meaning                                  |
|---------------------|------------------------------------------|
| `--delete`          | Delete files at the destination that aren't in the source (mirror) |
| `--dry-run` (`-n`)  | Test mode — shows what it would do        |
| `--exclude='*.log'` | Skip by pattern                          |
| `--exclude-from=file` | Patterns in a file                     |
| `--progress`        | Progress bar                             |
| `-h`                | Human-readable sizes                     |
| `--bwlimit=1000`    | Bandwidth limit (KB/s)                   |
| `-e 'ssh -p 2222'`  | Custom SSH command (port + key)          |

### Real examples

```bash
# Production deploy — eski fayllarni ham o'chiradi
rsync -avz --delete \
  --exclude='node_modules' \
  --exclude='.git' \
  --exclude='*.log' \
  ./dist/ prod:/var/www/

# Test ko'rish, lekin bajarmaslik
rsync -avzn --delete ./src/ prod:/opt/
# (yangi va o'chiriladiganlarni ko'rsatadi)

# Backup — eski fayllarni saqlash (--backup)
rsync -avz --backup --backup-dir=/backups/$(date +%F) \
  ~/Documents/ backup-server:/backups/current/

# Katta fayllar uchun resume + bandwidth limit
rsync -avz --partial --bwlimit=5000 \
  big.iso ali@server:/data/
```

::: tip `--dry-run` — always first
**Before** deploying to production, always run `rsync -avzn --delete` (n = dry-run). Especially with `--delete` — you'll see which files will be deleted.
:::

---

## 2.9. Running remote commands

### A single command

```bash
ssh ali@server 'uptime'
# 14:22:01 up 30 days, ...
```

### Multiple commands

```bash
ssh ali@server 'cd /var/log && ls -la'

# Yoki && bilan
ssh ali@server 'cd /tmp && tar -czf backup.tar.gz data/ && ls -lh backup.tar.gz'
```

### Multi-line via heredoc

```bash
ssh ali@server bash <<'EOF'
set -euo pipefail
cd /opt/app
echo "Joriy versiya: $(cat VERSION)"
git pull
./build.sh
sudo systemctl restart app
echo "Yangi versiya: $(cat VERSION)"
EOF
```

`'EOF'` (inside quotes) — there is **no** interpolation, everything runs on the server, not locally.

### Capturing output

```bash
load=$(ssh ali@server "uptime | awk '{print \$10}' | tr -d ','")
echo "Server load: $load"
```

::: warning Quote escaping
The `'` and `"` inside remote commands get confusing. Heredoc is the safest way. Or double-escape:
```bash
ssh server "echo \"hi\""
ssh server 'echo "hi"'   # afzal
```
:::

---

## 2.10. SSH tunneling — port forwarding

### Local forward (`-L`) — the most commonly used

```bash
# Local 5432 portni server'dagi db.internal:5432'ga ulash
ssh -L 5432:db.internal:5432 ali@jumphost
```

Now if you connect to `localhost:5432` — you're actually reaching `db.internal:5432` through `jumphost`.

Usage:
```bash
psql -h localhost -p 5432 -U postgres
# Aslida internal DB'ga ulanyapsiz!
```

### Background tunnel — `-fN`

```bash
ssh -fN -L 5432:db.internal:5432 ali@jumphost
# Background'ga ketadi, terminal'da turmaydi
# To'xtatish uchun:
ps aux | grep 'ssh -fN' | grep -v grep
kill <PID>
```

`-f` — background, `-N` — don't run a command (tunnel only).

### Remote forward (`-R`)

```bash
# Server'dagi 8080 portni local 3000'ga uzatish
ssh -R 8080:localhost:3000 ali@server
```

Useful: a dev server running locally, exposed to your team through the server (or for webhooks).

### Dynamic forward — SOCKS proxy (`-D`)

```bash
ssh -D 1080 ali@jumphost
# Browser'ni SOCKS5 proxy localhost:1080'ga sozlash
# Endi har trafik — jumphost orqali
```

Useful for getting into a corporate network without a VPN.

### Tunnel table

| Flag          | Direction                | Typical use                          |
|---------------|--------------------------|--------------------------------------|
| `-L LOCAL:HOST:REMOTE` | Local to remote     | Connecting to an internal DB         |
| `-R REMOTE:HOST:LOCAL` | Remote to local     | Webhook channel, dev preview         |
| `-D PORT`              | Dynamic SOCKS       | Browser proxy                        |

---

## 2.11. `ssh-agent` — key management

Entering the passphrase every time is tedious. `ssh-agent` keeps it in memory:

```bash
# Agent ishga tushirish (sessiya boshida)
eval "$(ssh-agent -s)"

# Kalitni qo'shish (passphrase bir marta so'raladi)
ssh-add ~/.ssh/id_ed25519

# Yuklangan kalitlar ro'yxati
ssh-add -l

# Hammasini o'chirish
ssh-add -D
```

### Built-in for macOS

```sshconfig
Host *
    UseKeychain yes
    AddKeysToAgent yes
```

Keychain integration — stores the passphrase in the macOS Keychain. It persists even after a system reboot.

### Agent forwarding (`-A`) — be careful!

```bash
ssh -A jumphost
# Endi jumphost'da bo'lib turib boshqa serverga ulanganingizda
# local kalitingiz ishlatiladi (jump'da saqlanmaydi)
```

::: danger The danger of agent forwarding
The **`-A`** flag — the jump host **root** can use your key (if the jump host is compromised). Instead — **`ProxyJump`** (`-J`) is recommended:
```bash
ssh -J jumphost destination
```
`ProxyJump` doesn't leave your key on the jump host.
:::

---

## 2.12. Real example — Deploy script

```bash
#!/usr/bin/env bash
#
# deploy.sh — local'da build → server'ga rsync → restart → health check
#
# Foydalanish:
#   ./deploy.sh staging
#   ./deploy.sh prod

set -euo pipefail
IFS=$'\n\t'

readonly ENV="${1:?Foydalanish: $0 <staging|prod>}"

# Konfiguratsiya — har environment uchun
case "$ENV" in
    staging)
        SSH_HOST="staging"
        APP_DIR="/var/www/staging"
        HEALTH_URL="https://staging.example.com/health"
        ;;
    prod)
        SSH_HOST="prod"
        APP_DIR="/var/www/app"
        HEALTH_URL="https://example.com/health"
        ;;
    *)
        echo "Noma'lum environment: $ENV" >&2
        exit 1
        ;;
esac

log() { printf '[%s] %s\n' "$(date +%T)" "$*"; }

# --- 1. Local build ---
log "📦 Local build..."
npm ci --silent
npm run build

# --- 2. Smoke test (build OK ekanligini tekshirish) ---
[[ -f dist/index.html ]] || {
    log "❌ Build muvaffaqiyatsiz — dist/index.html yo'q"
    exit 1
}

# --- 3. Rsync ---
log "🚀 Rsync → $SSH_HOST:$APP_DIR ..."
rsync -avz --delete \
    --exclude='*.log' \
    --exclude='.env.local' \
    ./dist/ "$SSH_HOST:$APP_DIR/"

# --- 4. Remote restart ---
log "🔄 Server restart..."
ssh "$SSH_HOST" bash <<EOF
set -euo pipefail
cd "$APP_DIR"
sudo systemctl restart app
sudo systemctl status app --no-pager | head -5
EOF

# --- 5. Health check (max 30s) ---
log "🩺 Health check: $HEALTH_URL"
for i in {1..15}; do
    if curl -fsS --max-time 5 "$HEALTH_URL" > /dev/null; then
        log "✅ Server ishlamoqda (urinish $i/15)"
        exit 0
    fi
    sleep 2
done

log "❌ Health check muvaffaqiyatsiz"
exit 1
```

Running it:
```bash
chmod +x deploy.sh
./deploy.sh staging   # avval staging'da sinash
./deploy.sh prod      # production
```

### What does this script do?

| Step                   | Technique                                         |
|------------------------|---------------------------------------------------|
| Environment selection  | `case` statement, `$1` argument                   |
| Local build            | `npm ci && npm run build`                         |
| Smoke test             | Checking the build succeeded                       |
| Atomic upload          | `rsync -avz --delete` — old files cleaned up       |
| Remote orchestration   | `ssh ... bash <<EOF` heredoc                      |
| Service restart        | `systemctl restart`                               |
| **Health check loop**  | `curl -fsS --max-time 5` retried 15 times          |
| Error handling         | `set -euo pipefail` everywhere                     |
| Structured logging     | `log()` function + timestamp                       |

---

## 2.13. Security practices

::: warning Production configuration
On the server side (`/etc/ssh/sshd_config`):

```sshconfig
PasswordAuthentication no            # faqat key auth
PermitRootLogin prohibit-password    # root parol bilan kirmasin
PubkeyAuthentication yes
MaxAuthTries 3
ClientAliveInterval 300              # 5 daqiqa idle → uzilish
ClientAliveCountMax 2
AllowUsers ali deploy                # ruxsat berilganlar
```

After configuring:
```bash
sudo sshd -t                # config syntaxni tekshirish
sudo systemctl restart sshd
```
:::

### The minimum skill set

| Practice                                | Reason                             |
|-----------------------------------------|------------------------------------|
| Password auth disabled                  | Removes brute-force risk           |
| `fail2ban` installed                    | Automatic ban (3 fails = 1 hour IP) |
| Default port 22 → another (optional)    | A little protection from scanners  |
| `MFA` (Google Authenticator)            | Key + code = two layers            |
| Audit log monitoring                    | `/var/log/auth.log`                |
| Keys rotated every 1-2 years            | Hygiene                            |

---

## 2.14. Frequently encountered errors

::: danger Classic traps

1. **`Permission denied (publickey)` — `~/.ssh/` permissions are wrong.**
   `chmod 700 ~/.ssh && chmod 600 ~/.ssh/authorized_keys` on the server side.

2. **`Host key verification failed` — the IP/key changed.**
   Only after a **known** reason: `ssh-keygen -R hostname` — removes the old entry.

3. **`ssh -A` is a security gap on the jump host.**
   Recommendation: `ProxyJump` (`-J`) or `ProxyJump` in the config.

4. **`scp -P` vs `-p`.**
   Capital P — port. Lowercase p — preserve permissions.

5. **`rsync src dst` vs `rsync src/ dst/`.**
   The trailing slash difference is big — check with `--dry-run` every time.

6. **Carelessness with `rsync --delete`.**
   If the source is missing or empty — it deletes **everything** at the destination. Always preview with `-n` first.

7. **Quote interpolation in heredocs.**
   `<<EOF` — interpolates locally (`$var` is the local one). `<<'EOF'` — on the remote. Don't mix them up.

8. **SSH not working in cron.**
   In the cron environment there's no `SSH_AUTH_SOCK` — `ssh-agent` doesn't work. Workaround: start the agent at the beginning of the script, or specify `IdentityFile` explicitly.
:::

---

## 2.15. Exercises

::: tip 🧪 Bashlings — interactive exercises
This chapter's **7** exercises with auto-checking via the `bashlings` CLI. All are
**offline-friendly** — they don't require a real SSH server (you build the commands as
STRINGS):

```bash
bashlings watch              # birinchi pending mashqdan boshlash
bashlings run ssh1           # bitta mashqni tekshirish
bashlings hint ssh1          # bosqichli maslahat
```

Source: [`exercises/12_ssh/`](https://github.com/qobulovasror/bashlings/tree/main/exercises/12_ssh)
:::

Try the following real-world exercises where you have a server:

1. **Create a key** — create a key via `ssh-keygen -t ed25519 -f ~/test_key -N ""`. Confirm that the public and private files exist.

2. **Test `~/.ssh/config`** — create the following alias: `Host gh` → `github.com`, user `git`. Does `ssh -T gh` work?

3. **`rsync` dry-run** — try `--delete` with `--dry-run` on a local directory against another directory. Read through the output.

4. **Remote command** — on some server, run `df -h /` remotely and write a pipeline that outputs only the use-percentage value (`%`).

5. **Tunnel test** — start `ssh -fN -L 8080:google.com:80 user@yourserver`. Check the result of local `curl http://localhost:8080 -H "Host: google.com"`.

---

## 2.16. Summary

| Concept                         | Key point                                          |
|---------------------------------|----------------------------------------------------|
| `ssh user@host`                 | Basic connection                                   |
| `ssh-keygen -t ed25519`         | Modern key creation                                |
| `ssh-copy-id user@host`         | Uploading the public key to the server             |
| **`~/.ssh/config`**             | Host alias, port, identity — the most powerful feature |
| `ProxyJump <alias>`             | Through a jump host                                 |
| `ControlMaster auto`            | Connection multiplexing — faster reconnect         |
| `scp` / `rsync`                 | Copying files                                       |
| **`rsync -avz --delete`**       | Production mirror                                  |
| `rsync -avzn`                   | Dry-run — **always check first**                   |
| `ssh host 'cmd'`                | A single command                                   |
| `ssh host bash <<'EOF'`         | A multi-line script                                |
| `-L LOCAL:HOST:REMOTE`          | Local forward (the most common)                    |
| `-fN`                           | Background tunnel                                  |
| `ssh-add`                       | Adding a key to the agent                          |

### 5 core ideas

1. **`ed25519` keys** — not RSA, but ed25519. Modern and small.
2. **`~/.ssh/config`** — half an hour of setup saves a lifetime of time.
3. **`rsync -avzn` (dry-run)** — always test first with `--delete`.
4. **Don't use `-A` instead of `ProxyJump`** — it breaks security.
5. **Connection multiplexing** (`ControlMaster auto`) — a 10x speedup in CI and scripts.

🎉 Now you've gained the skill of **managing remote servers**. In the next chapter — we'll learn to parse API responses with **`jq`**.

> **Next page:** [3. JSON and YAML — jq, yq →](./03-jq)
