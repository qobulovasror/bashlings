---
title: "Docker bilan integratsiya"
description: "docker run, exec, logs, Dockerfile, docker compose va Bash skriptlardan konteynerlarni boshqarish."
---

# 5. Docker bilan integratsiya

> **🎯 Bu bobda nimani o'rganasiz:**
> - **Docker** asoslari — Image vs Container, layer modeli
> - `docker run`, `exec`, `logs` — kundalik buyruqlar
> - **Port forwarding** va **volume mount**
> - **Dockerfile** yozish — multi-stage build bilan
> - **`docker compose`** — multi-container app'lar
> - Bash skriptlardan Docker'ni boshqarish (sandbox pattern)
> - Cleanup va resource boshqaruvi
> - Real misol — **PostgreSQL test sandbox**
>
> **⏱ Vaqt:** ~35 daqiqa
> **🧪 Mashqlar:** `bashlings watch` — 7 ta interaktiv mashq tayyor ([`exercises/15_docker/`](https://github.com/qobulovasror/bashlings/tree/main/exercises/15_docker))

---

## 5.1. Docker nima?

**Konteyner** — ilovani **uning hamma dependency'lari bilan** o'rab, **izolatsiyalangan** muhitda ishga tushirish.

VM bilan farqi:
- VM — to'liq operatsion tizim emulyatsiyasi (og'ir, sekin boot)
- Konteyner — **bir kernelni baham ko'radi**, lekin process/network/filesystem darajasida izolatsiya (yengil, sekundlarda ishga tushadi)

::: tip Asosiy g'oya
**"Mening kompyuterimda ishlaydi"** muammosini hal qiladi. Konteyner — sizning kompyuteringizdagi muhit **+ ilovangiz** — bitta paket sifatida.
:::

### Image vs Container — kritik farq

| Image                          | Container                            |
|--------------------------------|--------------------------------------|
| Statik shablon (read-only)     | Image'ning **ishlovchi nusxasi**     |
| Layerlardan tashkil topgan     | Image + writable layer + runtime state |
| `docker images` ko'rinadi      | `docker ps` ko'rinadi                |
| Kompozitsiya: `Dockerfile`     | Yaratiladi: `docker run <image>`     |
| O'lchami: MB-GB                | Image hajmi + o'zgarishlar           |

Analogiya: **image — sinf**, **container — sinfning obyekti**.

---

## 5.2. O'rnatish va birinchi ulanish

```bash
# macOS — Docker Desktop yoki OrbStack
brew install --cask docker        # Docker Desktop
brew install orbstack             # alternativ (yengilroq)

# Ubuntu / Debian
curl -fsSL https://get.docker.com | sh

# Versiya tekshirish
docker --version           # Docker version 27.x...
docker info                # daemon ulangan-ulanmaganligini
```

Birinchi test:
```bash
docker run --rm hello-world
# Hello from Docker! ...
```

`--rm` — konteyner tugaganidan keyin avtomatik o'chirilsin.

---

## 5.3. Asosiy buyruqlar

### Konteyner ishga tushirish

```bash
# Sodda
docker run nginx                       # foregroundda (Ctrl+C bilan to'xtaydi)
docker run -d nginx                    # detached (background)
docker run -d --name web nginx         # nom bilan

# Port forwarding
docker run -d -p 8080:80 nginx
#                ↑      ↑
#                host   container

curl http://localhost:8080             # nginx ga ulanish

# Volume mount
docker run -d -v $(pwd)/html:/usr/share/nginx/html:ro nginx

# Environment variable
docker run -d -e POSTGRES_PASSWORD=secret postgres:16

# Hammasi birgalikda
docker run -d \
    --name web \
    -p 8080:80 \
    -v $(pwd)/html:/usr/share/nginx/html:ro \
    -e NGINX_HOST=example.com \
    nginx
```

### Klassik `run` flag'lari

| Flag                  | Mazmuni                                      |
|-----------------------|----------------------------------------------|
| `-d` (`--detach`)     | Background'ga                                |
| `-it`                 | **Interactive + TTY** (shell sessiya uchun)  |
| `--rm`                | Tugagandan keyin o'chirish                   |
| `--name <nom>`        | Konteyner nomi (default — tasodifiy)         |
| `-p HOST:CONT`        | Port mapping                                 |
| `-v HOST:CONT[:ro]`   | Volume mount (`ro` = read-only)              |
| `-e KEY=value`        | Environment variable                         |
| `--env-file .env`     | Env fayldan yuklash                          |
| `--network <net>`     | Custom network                               |
| `--restart unless-stopped` | Avto-restart                            |
| `--memory 512m`       | Xotira limit                                 |
| `--cpus 1.5`          | CPU limit                                    |

### Konteynerlarni ko'rish

```bash
docker ps                  # ishlovchi konteynerlar
docker ps -a               # **HAMMASI** (to'xtaganlar ham)
docker ps -aq              # faqat ID'lar
docker ps --filter status=exited
```

### Boshqarish

```bash
docker stop web            # SIGTERM yuborish (10s kutadi)
docker kill web            # SIGKILL (darhol)
docker restart web
docker rm web              # o'chirish (avval to'xtatilgan bo'lishi kerak)
docker rm -f web           # to'xtab va o'chirish birga
```

### Image'lar

```bash
docker images              # local image'lar
docker pull nginx:1.25     # serverdan yuklash
docker rmi nginx:1.25      # o'chirish
docker image prune         # ishlatilmaydigan image'larni tozalash
```

---

## 5.4. Interaktiv ishlash

### Konteynerga "kirib" ko'rish

```bash
# Yangi konteynerda bash sessiya
docker run -it --rm ubuntu:24.04 bash
# root@a1b2c3:/# ls
# root@a1b2c3:/# exit

# Ishlovchi konteyner ichida
docker exec -it web bash
docker exec -it web sh           # nginx, alpine — `sh` default
```

`-it`:
- **`-i`** — STDIN ochiq (input kira oladi)
- **`-t`** — TTY ajratish (terminal sifatida ko'rinish)

### Loglarni ko'rish

```bash
docker logs web                    # hammasini
docker logs -f web                 # follow (real-time)
docker logs --tail 50 web          # oxirgi 50 qator
docker logs --since 10m web        # oxirgi 10 daqiqa
docker logs --timestamps web       # vaqt belgisi bilan
```

::: tip `docker logs -f` = `tail -f`
Aksariyat konteyner image'lari log'larni stdout/stderr'ga yozadi. `docker logs` ularni ushlab oladi. Production muhitda — **markaziy log aggregation** (Loki, ELK) afzal.
:::

### Konteyner statistikasi

```bash
docker stats               # real-time CPU/memory/network
docker stats --no-stream   # bir snapshot

docker inspect web         # to'liq JSON metadata
docker inspect web | jq '.[0].NetworkSettings.IPAddress'
```

---

## 5.5. Volume mount va port mapping

### Bind mount (host → container)

```bash
# Mavjud katalogni mount qilish
docker run -d -v $(pwd):/app -w /app node:20 npm start

# Read-only
docker run -d -v /etc/myapp:/config:ro myapp

# Macros — $(pwd) absolyut yo'l beradi
docker run -v $(pwd):/data busybox ls /data
```

### Named volume (Docker-managed)

```bash
# Volume yaratish
docker volume create pgdata

# Foydalanish
docker run -d -v pgdata:/var/lib/postgresql/data postgres:16

# Hammasi
docker volume ls
docker volume inspect pgdata
docker volume rm pgdata
```

::: warning Bind mount permissions
Konteyner ichidagi user (odatda `root` yoki UID 1000) host'dagi fayllarga to'g'ri ruxsat bilan kirishi kerak. macOS'da odatda ishlaydi, Linux'da `chown` kerak bo'lishi mumkin.
:::

### Port mapping nyuans

```bash
docker run -p 8080:80 nginx
# Host 0.0.0.0:8080 → container 80

docker run -p 127.0.0.1:8080:80 nginx
# Faqat localhost'dan kirish mumkin (xavfsiz)

docker run -p 8080:80/udp myudp
# UDP port

docker run -P nginx
# HAMMA EXPOSE qilingan portlarni TASODIFIY hostda
docker ps        # qaysi port'ga kelganini ko'ring
```

---

## 5.6. Environment variables

```bash
# Bir nechta -e
docker run -d \
    -e POSTGRES_USER=admin \
    -e POSTGRES_PASSWORD=secret \
    -e POSTGRES_DB=myapp \
    postgres:16

# Env fayldan
cat > .env <<'EOF'
POSTGRES_USER=admin
POSTGRES_PASSWORD=secret
POSTGRES_DB=myapp
EOF

docker run --env-file .env -d postgres:16
```

::: danger Secrets va env vars
`docker inspect <container>` — barcha env vars'ni JSON'da ko'rsatadi. Secret'larni env var orqali berish — laboratoriya uchun OK, **production'da xavfli**. Foydalaning:
- **Docker secrets** (Swarm)
- **Kubernetes Secrets**
- Vault, AWS Secrets Manager
- Yoki minimal: `--env-file` + faylga 600 permissions
:::

---

## 5.7. `Dockerfile` — image yaratish

### Sodda misol

```dockerfile
# Dockerfile
FROM alpine:3.19

# Bog'liq paketlar
RUN apk add --no-cache curl jq

# Skript ko'chirish
COPY healthcheck.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/healthcheck.sh

# Ishchi katalog
WORKDIR /app

# Default buyruq
CMD ["/usr/local/bin/healthcheck.sh"]
```

Build va run:
```bash
docker build -t myhealthcheck:1.0 .
docker run --rm myhealthcheck:1.0
```

### Klassik direktivalar

| Direktiv      | Mazmuni                                       |
|---------------|-----------------------------------------------|
| `FROM`        | Base image (har Dockerfile boshida)           |
| `RUN`         | Build paytida buyruq bajarish                  |
| `COPY src dst`| Local'dan image'ga ko'chirish                  |
| `ADD src dst` | `COPY` + URL/archive support (camroq tavsiya)|
| `WORKDIR /x`  | Joriy katalogni belgilash                     |
| `ENV K=V`     | Environment variable                          |
| `EXPOSE 8080` | Hujjat — port ekspozitsiyasi (avtomatik mapping yo'q) |
| `CMD ["..."]` | Default ishga tushish buyrug'i (override etiladi) |
| `ENTRYPOINT [...]` | Asosiy buyruq (CMD argumentlar)          |
| `USER ali`    | Foydalanuvchi (security uchun muhim)          |
| `HEALTHCHECK` | Health check buyrug'i                         |
| `LABEL k=v`   | Metadata                                      |

### `.dockerignore`

```
# .dockerignore — `COPY .` paytida nimani SKIP qilish
node_modules
.git
*.log
.env*
dist
target
```

Build vaqtini kamaytiradi va xavfsizlikni oshiradi (secrets'ni image'ga tushirmaydi).

### Multi-stage build — kichik image hajmi

```dockerfile
# Stage 1 — build muhiti (katta, faqat build uchun)
FROM rust:1.78 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Stage 2 — runtime (kichik, faqat binary)
FROM debian:12-slim
COPY --from=builder /app/target/release/myapp /usr/local/bin/
CMD ["myapp"]
```

| Yondashuv          | Final image hajmi  |
|--------------------|---------------------|
| `FROM rust:1.78`   | ~1.5 GB             |
| Multi-stage `debian:12-slim` | ~80 MB     |
| Multi-stage `scratch` | ~10 MB           |

::: tip Layer caching
Har `RUN`, `COPY` — yangi layer. Layer'lar kashlanadi. **Tez-tez o'zgaradigan narsalarni keyinroqqa qo'ying**:
```dockerfile
COPY package.json package-lock.json ./
RUN npm ci          # ← kashlangan, dependencies o'zgarmasa qayta ishlamaydi
COPY . .            # ← har gal yangidan
```
:::

---

## 5.8. `docker compose` — multi-container app

Bitta `docker-compose.yml` — butun stack (web + db + cache + ...).

### `docker-compose.yml` misol

```yaml
services:
  db:
    image: postgres:16
    environment:
      POSTGRES_DB: myapp
      POSTGRES_USER: admin
      POSTGRES_PASSWORD: secret
    volumes:
      - pgdata:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U admin"]
      interval: 5s
      retries: 5

  cache:
    image: redis:7-alpine
    ports:
      - "127.0.0.1:6379:6379"

  web:
    build: .                    # joriy Dockerfile'dan
    ports:
      - "8080:80"
    environment:
      DATABASE_URL: postgres://admin:secret@db:5432/myapp
      REDIS_URL: redis://cache:6379
    depends_on:
      db:
        condition: service_healthy
      cache:
        condition: service_started

volumes:
  pgdata:
```

### Boshqarish

```bash
docker compose up -d          # background'da ishga tushirish
docker compose ps             # holatni ko'rish
docker compose logs -f web    # bitta service log
docker compose exec db psql -U admin myapp   # interaktiv
docker compose restart web
docker compose down           # to'xtatish + konteyner o'chirish
docker compose down -v        # + volume'larni ham
```

### Service nomlari = DNS

Compose ichidagi service'lar **bir-birini nom orqali** topadi:
```bash
# web konteyneri ichidan
curl http://db:5432      # `db` service'iga ulanadi
redis-cli -h cache       # `cache` service'iga
```

Bu — Compose'ning **eng sehrli xususiyati**. Network sozlash kerak emas.

---

## 5.9. Bash skriptlardan Docker'ni boshqarish

### Konteyner statusini parse qilish

```bash
# Status — string sifatida
status=$(docker inspect -f '{{.State.Status}}' web)
echo "$status"             # running / exited / paused / ...

# JSON + jq bilan kuchliroq
docker ps --format json | jq -r 'select(.Names == "web") | .Status'
```

### Health check loop

```bash
#!/usr/bin/env bash
# wait-for-healthy.sh — konteyner sog'lom bo'lishini kutish

NAME="${1:?Konteyner nomi kerak}"
MAX_WAIT=60

for ((i=0; i<MAX_WAIT; i++)); do
    state=$(docker inspect -f '{{.State.Health.Status}}' "$NAME" 2>/dev/null || echo "missing")
    case "$state" in
        healthy)   echo "✓ $NAME sog'lom"; exit 0 ;;
        starting)  echo "  Boshlanyapti... ($i/$MAX_WAIT)" ;;
        unhealthy) echo "✗ $NAME nosog'lom"; exit 1 ;;
        missing)   echo "Konteyner topilmadi"; exit 2 ;;
    esac
    sleep 1
done

echo "Timeout"
exit 1
```

### Cleanup pattern (trap bilan)

```bash
#!/usr/bin/env bash
set -euo pipefail

CONTAINER="test-pg-$$"        # $$ — joriy PID

cleanup() {
    docker rm -f "$CONTAINER" 2>/dev/null || true
}
trap cleanup EXIT

# Ishga tushirish
docker run -d --name "$CONTAINER" \
    -e POSTGRES_PASSWORD=test \
    -p 5432:5432 \
    postgres:16

# Kutish
sleep 5

# Test ishi
psql -h localhost -U postgres -c 'SELECT version();'

# trap cleanup avtomatik chaqiriladi
```

---

## 5.10. Real misol — Test database sandbox

Production-grade pattern: har test uchun toza PostgreSQL ishga tushirish.

```bash
#!/usr/bin/env bash
#
# with-pg.sh — toza PostgreSQL sandbox'da har komandani bajarish
#
# Foydalanish:
#   ./with-pg.sh psql -c 'SELECT 1;'
#   ./with-pg.sh ./integration-tests.sh
#

set -euo pipefail

readonly CONTAINER="pg-sandbox-$$"
readonly DB_PORT=15432
readonly DB_PASSWORD="${DB_PASSWORD:-test_$(date +%s)}"

cleanup() {
    local rc=$?
    echo "🧹 Sandbox tozalanmoqda..."
    docker rm -f "$CONTAINER" > /dev/null 2>&1 || true
    exit "$rc"
}
trap cleanup EXIT INT TERM

echo "📦 PostgreSQL sandbox ishga tushyapti..."
docker run -d \
    --name "$CONTAINER" \
    -e POSTGRES_PASSWORD="$DB_PASSWORD" \
    -e POSTGRES_USER=test \
    -e POSTGRES_DB=testdb \
    -p "$DB_PORT:5432" \
    postgres:16 > /dev/null

# Ready bo'lishini kutish
echo "⏳ Database tayyor bo'lishini kutyapman..."
for i in {1..30}; do
    if docker exec "$CONTAINER" pg_isready -U test > /dev/null 2>&1; then
        echo "✓ Tayyor (${i}s)"
        break
    fi
    sleep 1
    [[ $i -eq 30 ]] && { echo "❌ Timeout"; exit 1; }
done

# Connection ma'lumotlarini export qilish
export PGHOST=localhost
export PGPORT="$DB_PORT"
export PGUSER=test
export PGPASSWORD="$DB_PASSWORD"
export PGDATABASE=testdb

# Foydalanuvchi buyrug'ini bajarish
echo "🧪 Bajarish: $*"
"$@"
```

Foydalanish:
```bash
chmod +x with-pg.sh

# Sodda test
./with-pg.sh psql -c 'SELECT version();'

# Integration tests
./with-pg.sh ./tests/run-integration.sh

# Interaktiv shell
./with-pg.sh bash
# PGHOST, PGPORT, ... avtomatik export qilingan
```

Bu skript:
- Har ishga tushganda **toza** sandbox yaratadi
- PostgreSQL'ning ready bo'lishini sabr bilan kutadi
- Connection parameters'ni env vars sifatida ekspozitsiya qiladi
- Har holda (success/fail/Ctrl+C) — sandbox tozalanadi
- Cross-platform, hech narsa o'rnatish shart emas (Docker'dan tashqari)

---

## 5.11. Cleanup va resource boshqaruvi

Docker'ning eng katta tashvishi — **disk to'la bo'lib qolish**. Ishlatilmaydigan image, volume, network'lar yig'iladi.

### Klassik tozalash buyruqlari

```bash
# Hech narsa ishlatmaydiganlarni tozalash
docker container prune        # to'xtagan konteynerlar
docker image prune            # dangling image'lar
docker image prune -a         # HAMMA ishlatilmaydigan
docker volume prune           # ishlatilmaydigan volume'lar
docker network prune          # ishlatilmaydigan network'lar

# Hammasini birgalikda
docker system prune           # to'xtagan + dangling
docker system prune -a        # + ishlatilmaydigan image'lar
docker system prune -a --volumes   # + volume'lar (DIQQAT — data yo'qoladi)
```

### Disk ishlatishni ko'rish

```bash
docker system df              # umumiy hisobot
# TYPE              TOTAL     ACTIVE    SIZE
# Images            42        12        15.2GB
# Containers        18        4         312MB
# Local Volumes     7         3         2.1GB

docker system df -v           # batafsil
```

### Avtomatlashtirilgan cleanup (cron)

```cron
# Har yakshanba kechasi
0 3 * * 0 docker system prune -af --volumes --filter "until=168h" >> /var/log/docker-cleanup.log 2>&1
```

`--filter "until=168h"` — 168 soat (7 kun) eski'larni.

---

## 5.12. Xavfsizlik amaliyotlari

```dockerfile
# 1. Non-root user
FROM node:20-alpine
RUN addgroup -S app && adduser -S app -G app
USER app
WORKDIR /app
COPY --chown=app:app . .
CMD ["node", "server.js"]

# 2. Read-only filesystem
# docker run --read-only --tmpfs /tmp myapp

# 3. Cheklov capabilities
# docker run --cap-drop=ALL --cap-add=NET_BIND_SERVICE myapp

# 4. Specific tag, not :latest
FROM node:20.11.0-alpine        # ✓ specific
# FROM node:latest              # ❌ moving target

# 5. Image scanning
# docker scout cves myapp:1.0    (Docker Desktop)
# trivy image myapp:1.0           (Trivy)
```

### Best practice checklist

- [ ] **`:latest`** ishlatmang — har image'ga aniq versiya tag
- [ ] **Non-root** ichida ishga tushiring (`USER`)
- [ ] **`.dockerignore`** — `.env`, `.git`, secrets file'larni skip
- [ ] **Multi-stage** — kichik final image
- [ ] **Image scanning** — `docker scout` yoki `trivy`
- [ ] **Resource limits** — `--memory`, `--cpus`
- [ ] **Read-only filesystem** mumkin bo'lganda
- [ ] **Secrets** — `--env-file` (faylda 600), Docker secrets, yoki Vault

---

## 5.13. Tez-tez uchraydigan xatolar

::: danger Klassik tuzoqlar

1. **`localhost` konteyner ichida ≠ host'ning localhost'i.**
   Konteyner ichidan host'ga ulanish:
   - Mac/Windows: `host.docker.internal`
   - Linux: `--add-host=host.docker.internal:host-gateway`

2. **Volume mount UID mismatch.**
   Host'da uid 1000, konteynerda uid 0 — permissions xatolik. `--user $(id -u):$(id -g)` yordam beradi.

3. **`:latest` tag — production'da xavfli.**
   "Bir hafta oldin ishlardi" — chunki `latest` o'zgargan. Har doim aniq tag.

4. **`docker rm` o'rniga `docker rmi`.**
   - `rm` — konteyner
   - `rmi` — image

5. **`docker logs` faylga emas, stdout'ga.**
   Konteyner ichida `> /var/log/...` qilsangiz — Docker ko'rmaydi.

6. **Build context juda katta.**
   `COPY . .` — joriy katalog hammasi context bo'lib uzatiladi. `.dockerignore` majburiy.

7. **`CMD` vs `ENTRYPOINT` adashtirish.**
   - `CMD ["node", "x.js"]` — `docker run myimage` → ishga tushadi. `docker run myimage echo hi` — `CMD` override.
   - `ENTRYPOINT ["node"]` + `CMD ["x.js"]` — `docker run myimage y.js` → `node y.js`.

8. **Volume'ni nomli vs bind aralashtirish.**
   `-v /host/path:/in/container` — bind (host yo'l).
   `-v myvolume:/in/container` — named volume.

9. **Network izolatsiya'ni unutish.**
   Default'da konteynerlar bir network'da emas — `docker network create mynet` va `--network mynet` ishlating, yoki Compose ishlating.

10. **macOS'da fayl yozish sekinligi.**
    Bind mount macOS'da Linux'dan **10-100×** sekinroq. Performance kerak bo'lsa — named volume yoki OrbStack.
:::

---

## 5.14. Mashqlar

::: tip 🧪 Bashlings — interaktiv mashqlar
Bu bobning **7 ta** mashqi `bashlings` CLI orqali avto-tekshiruv bilan. Hammasi
Docker daemon talab qilmaydi — sintaksis va konfiguratsiya bilan ishlash:

```bash
bashlings watch              # birinchi pending mashqdan boshlash
bashlings run docker1        # bitta mashqni tekshirish
bashlings hint docker1       # bosqichli maslahat
```

Manba: [`exercises/15_docker/`](https://github.com/qobulovasror/bashlings/tree/main/exercises/15_docker)
:::

Quyidagi real-world mashqlarni Docker o'rnatilgan joyda sinab ko'ring:

1. **Nginx tezkor** — `docker run` orqali nginx ishga tushiring (8080 portda), `curl localhost:8080` ishlashini tasdiqlang, `docker rm -f` bilan tozalang.

2. **Custom Dockerfile** — alpine asosida `curl` va `jq` o'rnatilgan image quring (`-t myhealthcheck`). Run qilib `curl --version` chiqishini ko'ring.

3. **Compose** — `docker-compose.yml` yozing: nginx + postgres + redis. `up -d` qiling va har 3'tasi `running` ekanini tekshiring.

4. **Health wait skript** — `wait-for-healthy.sh` (yuqorida) ni kompyuteringizda ishlatib, postgres konteyner sog'lom bo'lishini kutuvchi skript yozing.

5. **Cleanup automation** — har yakshanba 03:00 da `docker system prune -af` ni cron orqali avtomatlashtiring (logging bilan).

---

## 5.15. Xulosa

| Tushuncha                  | Asosiy nuqta                                   |
|----------------------------|------------------------------------------------|
| Image                      | Statik shablon                                 |
| Container                  | Image'ning ishlovchi nusxasi                   |
| `docker run`               | Yangi konteyner ishga tushirish                |
| `docker ps`                | Ishlovchi konteynerlar                         |
| `docker ps -a`             | + to'xtaganlar                                 |
| `-d`                       | Detached                                       |
| `-it`                      | Interaktiv + TTY                                |
| `--rm`                     | Avto-tozalash                                   |
| `-p HOST:CONT`             | Port mapping                                   |
| `-v PATH:PATH`             | Volume mount                                   |
| `-e KEY=val`               | Env variable                                   |
| `docker exec -it X bash`   | Konteynerga "kirib" ko'rish                    |
| `docker logs -f X`         | Real-time loglar                               |
| **`Dockerfile`**           | Image yaratish retsepti                        |
| **Multi-stage**            | Kichik final image (10× kamayadi)              |
| **`docker compose up`**    | Multi-container stack                          |
| `docker inspect -f`        | Skriptlar uchun metadata                       |
| `docker system prune -af`  | Toza qilish                                    |

### 5 ta asosiy g'oya

1. **Image ≠ Container** — sinf vs obyekt.
2. **`:latest` tag dushman** — production'da har doim aniq versiya.
3. **Multi-stage build** — final image hajmini 10-100× kamaytiradi.
4. **Docker Compose** — har local dev environment uchun standart.
5. **`docker system prune`** muntazam — aks holda disk tezda to'ladi.

🎉 Endi siz **konteynerlarni boshqarish** va Docker bilan Bash skriptlarni integratsiya qilish ko'nikmasini oldingiz. Keyingi va **oxirgi bobda** — **GitHub Actions** orqali hamma narsani CI/CD'da avtomatlashtirish.

> **Keyingi sahifa:** [6. CI/CD — GitHub Actions →](./06-cicd)
