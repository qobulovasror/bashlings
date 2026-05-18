---
title: "SSH va remote management"
description: "ssh, scp, rsync, public/private keys, ~/.ssh/config, tunneling va deploy skriptlar."
---

# 2. SSH va remote management

> **🎯 Bu bobda nimani o'rganasiz:**
> - **SSH** qanday ishlashi va public-key crypto asoslari
> - `ssh-keygen` — kalit yaratish (`ed25519` — zamonaviy tanlov)
> - **`~/.ssh/config`** — bashning eng kuchli xususiyatlaridan biri
> - **`scp`** va **`rsync`** — fayl ko'chirish
> - Remote buyruq bajarish, heredoc orqali skript yuborish
> - **Port forwarding** — local, remote, SOCKS
> - Real misol — **Deploy skript** (build + rsync + restart + health check)
>
> **⏱ Vaqt:** ~35 daqiqa
> **🧪 Mashqlar:** `bashlings watch 12_ssh` (kelajak sprint)

---

## 2.1. Nima uchun SSH?

Server administratsiyasi, DevOps, deployment, remote debugging — **SSH'siz qila olmaysiz**:

- Cloud server'ga ulanish (`ssh ec2-user@1.2.3.4`)
- Production'da log tahlili (`ssh prod 'tail -f /var/log/app.log'`)
- Kodni deploy qilish (`rsync -avz ./dist/ prod:/var/www/`)
- Database'ga lokal tunnel orqali ulanish (`ssh -L 5432:db:5432 prod`)
- Bir server orqali ikkinchisiga "sakrash" (`ssh -J jump prod`)

::: tip Asosiy g'oya
SSH — masofadagi tizimlar bilan **xavfsiz** muloqot uchun standart. **Parolsiz** auth (kalitlar bilan) — production'da yagona qabul qilinadigan usul.
:::

---

## 2.2. SSH qanday ishlaydi? (qisqacha)

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

Asosiy g'oya — **private key sizda qoladi**, **public key serverda**. Hech qachon o'rin almashmaydi.

::: warning Private key — eng muhim sir
Private key'ni hech qachon yubormang, copy qilmang, repository'ga qo'ymang. Yo'qotsangiz — qaytarib bo'lmaydi. Yo'qotgan kalitni darhol revoke qiling.
:::

---

## 2.3. Birinchi SSH ulanish

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

Birinchi ulanishda — **fingerprint'ni tekshirish**. Sizda CI'da yoki real serverda fingerprint qiymati bo'lishi kerak — taqqoslang.

### Asosiy flaglar

| Flag         | Mazmuni                                            |
|--------------|----------------------------------------------------|
| `-p <port>`  | Custom port (default 22)                           |
| `-i <fayl>`  | Aniq private key fayli                             |
| `-l <user>`  | User nomi (`-l user` = `user@host`)                |
| `-v` `-vv` `-vvv` | Verbose (debug, ko'p `v` — ko'p chiqish)      |
| `-N`         | Buyruq bajarmaslik (tunnel uchun)                  |
| `-f`         | Backgroundga o'tish (`-fN` tunnel uchun klassik)    |
| `-J <jump>`  | Jump host orqali                                   |
| `-A`         | SSH agent forwarding (ehtiyot bo'ling!)            |
| `-X` / `-Y`  | X11 forwarding                                     |

```bash
ssh -p 2222 ali@server.com         # custom port
ssh -i ~/.ssh/prod_key ali@prod    # aniq kalit
ssh -vvv ali@server.com            # debug muammoni topish uchun
```

---

## 2.4. SSH kalit yaratish

Parolsiz auth uchun kalit kerak.

```bash
ssh-keygen -t ed25519 -C "ali@example.com"
# Generating public/private ed25519 key pair.
# Enter file in which to save the key (/Users/ali/.ssh/id_ed25519):
# Enter passphrase (empty for no passphrase):
# Enter same passphrase again:
# Your identification has been saved in /Users/ali/.ssh/id_ed25519
# Your public key has been saved in /Users/ali/.ssh/id_ed25519.pub
```

### Flaglar

| Flag           | Mazmuni                                              |
|----------------|-----------------------------------------------------|
| `-t ed25519`   | Algoritm — **ed25519** (zamonaviy, kichik, tez)     |
| `-t rsa -b 4096` | Eski tizimlar uchun RSA 4096-bit                  |
| `-C "..."`     | Comment (odatda email)                              |
| `-f <fayl>`    | Custom fayl yo'li                                    |
| `-N "..."`     | Passphrase'ni argument sifatida                     |

::: tip Qaysi algoritm?
**`ed25519`** — bugun standart tanlov. **`rsa 4096-bit`** — eski tizimlar uchun (juda eskilarda ed25519 yo'q). **`dsa`** va kichik RSA — **ishlatmang** (xavfsizlik kuchsiz).
:::

### Kalit fayllar

```bash
~/.ssh/id_ed25519        # private key (SIR!)
~/.ssh/id_ed25519.pub    # public key (boshqalar bilan ulashish mumkin)
~/.ssh/known_hosts       # ko'rgan server fingerprintlari
~/.ssh/authorized_keys   # SIZning serveringizga kim kira oladi
~/.ssh/config            # ulanish sozlamalari (eng muhim!)
```

### Passphrase bo'lishi kerakmi?

**Ha**, agar:
- Laptop yo'qolib qolishi mumkin
- Kuchli passphrase + `ssh-agent` ishlatasiz

**Yo'q**, agar:
- Server-to-server cron skript (interaktiv yo'q)
- CI workflow

`ssh-agent` orqali passphrase'ni bir martagina kiritish va keyin kashlangan saqlash mumkin (§2.11).

---

## 2.5. `ssh-copy-id` — kalitni serverga yuklash

Manualda public key'ni serverga `authorized_keys`'ga qo'shish kerak. **`ssh-copy-id`** buni avtomatlashtiradi:

```bash
ssh-copy-id ali@server.com
# Bir marta parol so'raydi (oxirgi marta!)
# Endi key auth ishlaydi:
ssh ali@server.com   # parol so'ralmaydi
```

### Qo'lda variant (`ssh-copy-id` yo'q bo'lsa)

```bash
cat ~/.ssh/id_ed25519.pub \
  | ssh ali@server.com 'mkdir -p ~/.ssh && cat >> ~/.ssh/authorized_keys && chmod 600 ~/.ssh/authorized_keys'
```

::: warning Permissions muhim
`~/.ssh/` — `700`, `authorized_keys` — `600`. Aks holda SSH ishonmaydi va auth fail bo'ladi. `ssh-copy-id` buni avtomatik to'g'rilaydi.
:::

---

## 2.6. `~/.ssh/config` — SSH'ning sehri

Bu — bashning eng kam ma'lum lekin **eng kuchli** xususiyatlardan biri.

Tasavvur qiling — har gal `ssh -p 2222 -i ~/.ssh/prod_key ali@server-prod.example.com` yozish kerak. Yomon.

`~/.ssh/config` faylida bir marta yozasiz:

```sshconfig
Host prod
    HostName server-prod.example.com
    User ali
    Port 2222
    IdentityFile ~/.ssh/prod_key
```

Endi:
```bash
ssh prod
scp data.tar.gz prod:/opt/
rsync -avz dist/ prod:/var/www/
```

Hammasi avtomatik to'g'ri sozlamani oladi.

### To'liq misol — `~/.ssh/config`

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

### Klassik directive'lar

| Directive               | Mazmuni                                       |
|-------------------------|-----------------------------------------------|
| `HostName`              | Haqiqiy hostname/IP                           |
| `User`                  | Foydalanuvchi                                 |
| `Port`                  | Port (default 22)                             |
| `IdentityFile`          | Private key fayli                             |
| `ProxyJump <alias>`     | Jump host orqali                              |
| `ServerAliveInterval`   | Keepalive ping (soniya)                       |
| `ControlMaster auto`    | Connection multiplexing (tezroq qayta ulanish)|
| `ControlPersist 10m`    | Multiplexed kanalni saqlash                   |
| `LogLevel ERROR`        | Shovqinli ogohlantirishlarni o'chirish        |
| `IdentitiesOnly yes`    | Faqat aniq IdentityFile ishlatish             |

### Connection multiplexing (eng kuchli optimizatsiya)

```sshconfig
Host *
    ControlMaster auto
    ControlPath ~/.ssh/sockets/%r@%h-%p
    ControlPersist 10m
```

Birinchi ulanish — odatdagi tezlikda. Keyingi ulanishlar — **mavjud kanal orqali**, deyarli darhol. CI/skriptlar uchun ulkan farq.

::: tip macOS uchun bonus
```sshconfig
Host *
    UseKeychain yes
    AddKeysToAgent yes
```
macOS Keychain'ga passphrase saqlanadi. Bir marta kiritasiz, qayta-qayta so'ramaydi.
:::

---

## 2.7. `scp` — fayl ko'chirish

`scp` (Secure CoPy) — SSH protokoli orqali fayl ko'chiradi.

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

### `scp` flaglar

| Flag       | Mazmuni                                                  |
|------------|----------------------------------------------------------|
| `-r`       | Recursive (katalog uchun)                                |
| `-P <port>`| **Katta P** — port (kichik `-p` permissions saqlash)     |
| `-i <key>` | Identity file                                            |
| `-q`       | Quiet (progress yo'q)                                    |
| `-l <limit>`| Bandwidth chegarasi (Kbit/s)                            |
| `-C`       | Compression                                              |

::: warning `scp` deprecated?
OpenSSH 9.0+ (2022)'da `scp` rasmiy **deprecated**. `rsync` yoki `sftp` tavsiya etiladi. Hozir ham ishlaydi, lekin yangi loyihalarda `rsync` ishlating.
:::

---

## 2.8. `rsync` — kuchli sinxronlash

`rsync` — fayllarni "deltali" sinxronlaydi. **O'zgargan qismlarni** uzatadi, hammasini emas. Backup, deploy va mirror uchun ideal.

### Asosiy sintaksis

```bash
rsync -avz manba/ maqsad/
```

Diqqat: **`/`** trailing slash muhim!
- `rsync src/ dst/` — src **ichidagi** narsalarni dst ichiga
- `rsync src dst/` — src **katalogini** dst ichiga (yangi `dst/src/` yaratadi)

### Klassik flag'lar — `-avz`

| Flag | Mazmuni                                          |
|------|--------------------------------------------------|
| `-a` | **Archive mode** — `-rlptgoD` (recursive, links, permissions, times, group, owner, devices) |
| `-v` | Verbose                                          |
| `-z` | Compression                                      |

Bu uchaloni birga "**`-avz`**" deb yozish — `rsync`'ning klassik kombinatsiyasi.

### Boshqa muhim flag'lar

| Flag                | Mazmuni                                  |
|---------------------|------------------------------------------|
| `--delete`          | Maqsadda manbada yo'q fayllarni o'chirish (mirror) |
| `--dry-run` (`-n`)  | Test rejim — nima qilishini ko'rsatadi   |
| `--exclude='*.log'` | Pattern bo'yicha skip                    |
| `--exclude-from=fayl` | Pattern'lar faylda                     |
| `--progress`        | Progress bar                             |
| `-h`                | Human-readable hajmlar                   |
| `--bwlimit=1000`    | Bandwidth chegarasi (KB/s)               |
| `-e 'ssh -p 2222'`  | Custom SSH command (port + key)          |

### Real misollar

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

::: tip `--dry-run` — har doim avval
Production'ga deploy qilishdan **oldin** har doim `rsync -avzn --delete` (n = dry-run) ishlating. Ayniqsa `--delete` bilan — qaysi fayllar o'chirilishini ko'rasiz.
:::

---

## 2.9. Remote buyruq bajarish

### Bir buyruq

```bash
ssh ali@server 'uptime'
# 14:22:01 up 30 days, ...
```

### Bir nechta buyruq

```bash
ssh ali@server 'cd /var/log && ls -la'

# Yoki && bilan
ssh ali@server 'cd /tmp && tar -czf backup.tar.gz data/ && ls -lh backup.tar.gz'
```

### Multi-line heredoc orqali

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

`'EOF'` (qo'shtirnoq ichida) — interpolatsiya **bo'lmaydi**, har narsa local'da emas, server'da bajariladi.

### Output capture qilish

```bash
load=$(ssh ali@server "uptime | awk '{print \$10}' | tr -d ','")
echo "Server load: $load"
```

::: warning Quote escaping
Remote buyruqlar ichidagi `'` va `"` chalkash. Heredoc — eng xavfsiz yo'l. Yoki double escape:
```bash
ssh server "echo \"hi\""
ssh server 'echo "hi"'   # afzal
```
:::

---

## 2.10. SSH tunneling — port forwarding

### Local forward (`-L`) — eng ko'p ishlatiladigan

```bash
# Local 5432 portni server'dagi db.internal:5432'ga ulash
ssh -L 5432:db.internal:5432 ali@jumphost
```

Endi `localhost:5432` ga ulansangiz — aslida `jumphost` orqali `db.internal:5432`'ga.

Foydalanish:
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

`-f` — background, `-N` — buyruq bajarmaslik (faqat tunnel).

### Remote forward (`-R`)

```bash
# Server'dagi 8080 portni local 3000'ga uzatish
ssh -R 8080:localhost:3000 ali@server
```

Foydali: localda dev server, server orqali jamoaga ko'rsatish (yoki webhooks).

### Dynamic forward — SOCKS proxy (`-D`)

```bash
ssh -D 1080 ali@jumphost
# Browser'ni SOCKS5 proxy localhost:1080'ga sozlash
# Endi har trafik — jumphost orqali
```

VPN'siz korporativ tarmoqqa kirish uchun foydali.

### Tunnel jadval

| Flag          | Yo'nalish                | Tipik foydalanish                    |
|---------------|--------------------------|--------------------------------------|
| `-L LOCAL:HOST:REMOTE` | Local'dan remote'ga | Internal DB'ga ulanish              |
| `-R REMOTE:HOST:LOCAL` | Remote'dan local'ga | Webhook kanali, dev preview         |
| `-D PORT`              | Dynamic SOCKS       | Browser proxy                        |

---

## 2.11. `ssh-agent` — kalit boshqaruvi

Har gal passphrase kiritish chigarcha. `ssh-agent` xotirada saqlaydi:

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

### macOS uchun built-in

```sshconfig
Host *
    UseKeychain yes
    AddKeysToAgent yes
```

Keychain integratsiya — passphrase'ni macOS Keychain'da saqlaydi. Sistem qayta yuklanganida ham saqlanadi.

### Agent forwarding (`-A`) — ehtiyot bo'ling!

```bash
ssh -A jumphost
# Endi jumphost'da bo'lib turib boshqa serverga ulanganingizda
# local kalitingiz ishlatiladi (jump'da saqlanmaydi)
```

::: danger Agent forwarding xavfi
**`-A`** flagi — jump host **root** sizning kalitingizdan foydalanishi mumkin (jump kompromat bo'lsa). Aksincha — **`ProxyJump`** (`-J`) tavsiya etiladi:
```bash
ssh -J jumphost destination
```
`ProxyJump` jump'da kalitingizni qoldirmaydi.
:::

---

## 2.12. Real misol — Deploy skripti

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

Ishga tushirish:
```bash
chmod +x deploy.sh
./deploy.sh staging   # avval staging'da sinash
./deploy.sh prod      # production
```

### Bu skript nima qiladi?

| Qadam                  | Texnika                                           |
|------------------------|---------------------------------------------------|
| Environment tanlash    | `case` operatori, `$1` argument                   |
| Local build            | `npm ci && npm run build`                         |
| Smoke test             | Build muvaffaqiyatli ekanligini tekshirish        |
| Atomic upload          | `rsync -avz --delete` — eski fayllar tozalandi    |
| Remote orchestration   | `ssh ... bash <<EOF` heredoc                      |
| Service restart        | `systemctl restart`                               |
| **Health check loop**  | `curl -fsS --max-time 5` 15 marta urinish         |
| Error handling         | `set -euo pipefail` har joyda                     |
| Strukturalangan log    | `log()` funksiya + timestamp                      |

---

## 2.13. Xavfsizlik amaliyotlari

::: warning Production konfiguratsiya
Server tomonda (`/etc/ssh/sshd_config`):

```sshconfig
PasswordAuthentication no            # faqat key auth
PermitRootLogin prohibit-password    # root parol bilan kirmasin
PubkeyAuthentication yes
MaxAuthTries 3
ClientAliveInterval 300              # 5 daqiqa idle → uzilish
ClientAliveCountMax 2
AllowUsers ali deploy                # ruxsat berilganlar
```

Sozlamadan keyin:
```bash
sudo sshd -t                # config syntaxni tekshirish
sudo systemctl restart sshd
```
:::

### Eng kam ko'nikma to'plami

| Amaliyot                                | Sabab                              |
|-----------------------------------------|------------------------------------|
| Parol auth o'chirilgan                  | Brute-force xavfini olib tashlash  |
| `fail2ban` o'rnatilgan                  | Avtomatik ban (3 fail = 1 soat IP) |
| Default port 22 → boshqasi (optional)   | Skanerlardan biroz himoya          |
| `MFA` (Google Authenticator)            | Kalit + kod = ikki qatlam          |
| Audit log monitoring                    | `/var/log/auth.log`                |
| Kalitlar har 1-2 yilda yangilanadi      | Hygiene                            |

---

## 2.14. Tez-tez uchraydigan xatolar

::: danger Klassik tuzoqlar

1. **`Permission denied (publickey)` — `~/.ssh/` permissions noto'g'ri.**
   `chmod 700 ~/.ssh && chmod 600 ~/.ssh/authorized_keys` server tomonda.

2. **`Host key verification failed` — kichik IP/key o'zgargan.**
   Faqat **bilgan** sababdan keyin: `ssh-keygen -R hostname` — eski entry o'chiriladi.

3. **`ssh -A` jump host'da xavfsizlik bo'shliq.**
   Tavsiya: `ProxyJump` (`-J`) yoki config'da `ProxyJump`.

4. **`scp -P` vs `-p`.**
   Katta P — port. Kichik p — preserve permissions.

5. **`rsync src dst` vs `rsync src/ dst/`.**
   Trailing slash farqi katta — har gal `--dry-run` bilan tekshiring.

6. **`rsync --delete` bilan ehtiyotsizlik.**
   Manba yo'q yoki bo'sh bo'lsa — maqsadning **hammasini o'chiradi**. Har doim `-n` bilan oldindan ko'ring.

7. **Heredoc'da quote interpolatsiya.**
   `<<EOF` — local'da interpolate (`$var` local'dagi). `<<'EOF'` — remote'da. Adashtirmang.

8. **Cron'da SSH ishlamasligi.**
   Cron muhitida `SSH_AUTH_SOCK` yo'q — `ssh-agent` ishlamaydi. Yo'l: skript boshida agent ishga tushirish, yoki `IdentityFile` aniq berish.
:::

---

## 2.15. Mashqlar

> 🧪 Kelajakda `bashlings watch 12_ssh` paketida.

1. **Kalit yaratish** — `ssh-keygen -t ed25519 -f ~/test_key -N ""` orqali kalit yarating. Public va private fayllar borligini tasdiqlang.

2. **`~/.ssh/config` test** — quyidagi alias yarating: `Host gh` → `github.com`, user `git`. `ssh -T gh` ishlaydimi?

3. **`rsync` dry-run** — local katalogni boshqa katalogga `--delete` bilan `--dry-run` qilib sinab ko'ring. Output'ni o'qib chiqing.

4. **Remote command** — biror server'da `df -h /` ni masofadan bajarib, faqat to'la-foiz qiymatni (`%`) chiqaruvchi pipeline yozing.

5. **Tunnel test** — `ssh -fN -L 8080:google.com:80 user@yourserver` ishga tushiring. Local `curl http://localhost:8080 -H "Host: google.com"` natijasini tekshiring.

---

## 2.16. Xulosa

| Tushuncha                       | Asosiy nuqta                                       |
|---------------------------------|----------------------------------------------------|
| `ssh user@host`                 | Asosiy ulanish                                     |
| `ssh-keygen -t ed25519`         | Zamonaviy kalit yaratish                           |
| `ssh-copy-id user@host`         | Public key'ni serverga yuklash                     |
| **`~/.ssh/config`**             | Host alias, port, identity — eng kuchli xususiyat  |
| `ProxyJump <alias>`             | Jump host orqali                                   |
| `ControlMaster auto`            | Connection multiplexing — qayta ulanish tezroq     |
| `scp` / `rsync`                 | Fayl ko'chirish                                    |
| **`rsync -avz --delete`**       | Production mirror                                  |
| `rsync -avzn`                   | Dry-run — **har doim oldin tekshiring**            |
| `ssh host 'cmd'`                | Bir buyruq                                         |
| `ssh host bash <<'EOF'`         | Multi-line skript                                  |
| `-L LOCAL:HOST:REMOTE`          | Local forward (eng ko'p)                           |
| `-fN`                           | Background tunnel                                  |
| `ssh-add`                       | Agent'ga kalit qo'shish                            |

### 5 ta asosiy g'oya

1. **`ed25519` kalitlari** — RSA emas, ed25519. Zamonaviy va kichik.
2. **`~/.ssh/config`** — yarim soat sozlash bir umrlik vaqt tejaydi.
3. **`rsync -avzn` (dry-run)** — `--delete` bilan har doim oldin sinab ko'ring.
4. **`ProxyJump`** o'rniga **`-A`** ishlatmang — xavfsizlik buziladi.
5. **Connection multiplexing** (`ControlMaster auto`) — CI va skriptlarda 10× tezlanish.

🎉 Endi siz **masofadagi serverlarni boshqarish** ko'nikmasini oldingiz. Keyingi bobda — **`jq`** orqali API javoblarini parse qilishni o'rganamiz.

> **Keyingi sahifa:** [3. JSON va YAML — jq, yq →](./03-jq)
