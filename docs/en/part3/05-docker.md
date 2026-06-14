---
title: "Docker integration"
description: "docker run, exec, logs, Dockerfile, docker compose, and managing containers from Bash scripts."
---

# 5. Docker integration

> **🎯 What you'll learn in this chapter:**
> - **Docker** basics — Image vs Container, the layer model
> - `docker run`, `exec`, `logs` — everyday commands
> - **Port forwarding** and **volume mount**
> - Writing a **Dockerfile** — with multi-stage build
> - **`docker compose`** — multi-container apps
> - Managing Docker from Bash scripts (sandbox pattern)
> - Cleanup and resource management
> - A real example — **PostgreSQL test sandbox**
>
> **⏱ Time:** ~35 minutes
> **🧪 Exercises:** `bashlings watch` — 7 interactive exercises ready ([`exercises/15_docker/`](https://github.com/qobulovasror/bashlings/tree/main/exercises/15_docker))

---

## 5.1. What is Docker?

A **container** wraps an application **together with all its dependencies** and runs it in an **isolated** environment.

Difference from a VM:
- VM — full operating system emulation (heavy, slow boot)
- Container — **shares one kernel**, but is isolated at the process/network/filesystem level (lightweight, starts in seconds)

::: tip Core idea
It solves the **"works on my machine"** problem. A container is the environment on your machine **+ your application** — as a single package.
:::

### Image vs Container — the critical difference

| Image                          | Container                            |
|--------------------------------|--------------------------------------|
| Static template (read-only)    | A **running instance** of the image  |
| Made up of layers              | Image + writable layer + runtime state |
| Shown by `docker images`       | Shown by `docker ps`                 |
| Composition: `Dockerfile`      | Created with: `docker run <image>`   |
| Size: MB-GB                    | Image size + changes                 |

Analogy: **image — class**, **container — instance of the class**.

---

## 5.2. Installation and first connection

```bash
# macOS — Docker Desktop or OrbStack
brew install --cask docker        # Docker Desktop
brew install orbstack             # alternative (lighter)

# Ubuntu / Debian
curl -fsSL https://get.docker.com | sh

# Check the version
docker --version           # Docker version 27.x...
docker info                # whether the daemon is connected or not
```

First test:
```bash
docker run --rm hello-world
# Hello from Docker! ...
```

`--rm` — automatically remove the container after it finishes.

---

## 5.3. Core commands

### Starting a container

```bash
# Simple
docker run nginx                       # in the foreground (stops with Ctrl+C)
docker run -d nginx                    # detached (background)
docker run -d --name web nginx         # with a name

# Port forwarding
docker run -d -p 8080:80 nginx
#                ↑      ↑
#                host   container

curl http://localhost:8080             # connect to nginx

# Volume mount
docker run -d -v $(pwd)/html:/usr/share/nginx/html:ro nginx

# Environment variable
docker run -d -e POSTGRES_PASSWORD=secret postgres:16

# All together
docker run -d \
    --name web \
    -p 8080:80 \
    -v $(pwd)/html:/usr/share/nginx/html:ro \
    -e NGINX_HOST=example.com \
    nginx
```

### Classic `run` flags

| Flag                  | Meaning                                      |
|-----------------------|----------------------------------------------|
| `-d` (`--detach`)     | To the background                            |
| `-it`                 | **Interactive + TTY** (for a shell session)  |
| `--rm`                | Remove after it finishes                     |
| `--name <name>`       | Container name (default — random)            |
| `-p HOST:CONT`        | Port mapping                                 |
| `-v HOST:CONT[:ro]`   | Volume mount (`ro` = read-only)              |
| `-e KEY=value`        | Environment variable                         |
| `--env-file .env`     | Load from an env file                        |
| `--network <net>`     | Custom network                               |
| `--restart unless-stopped` | Auto-restart                            |
| `--memory 512m`       | Memory limit                                 |
| `--cpus 1.5`          | CPU limit                                    |

### Viewing containers

```bash
docker ps                  # running containers
docker ps -a               # **ALL** (including stopped ones)
docker ps -aq              # IDs only
docker ps --filter status=exited
```

### Managing

```bash
docker stop web            # send SIGTERM (waits 10s)
docker kill web            # SIGKILL (immediately)
docker restart web
docker rm web              # remove (must be stopped first)
docker rm -f web           # stop and remove together
```

### Images

```bash
docker images              # local images
docker pull nginx:1.25     # download from the server
docker rmi nginx:1.25      # remove
docker image prune         # clean up unused images
```

---

## 5.4. Working interactively

### "Getting inside" a container

```bash
# A bash session in a new container
docker run -it --rm ubuntu:24.04 bash
# root@a1b2c3:/# ls
# root@a1b2c3:/# exit

# Inside a running container
docker exec -it web bash
docker exec -it web sh           # nginx, alpine — `sh` is the default
```

`-it`:
- **`-i`** — STDIN open (can receive input)
- **`-t`** — allocate a TTY (appears as a terminal)

### Viewing logs

```bash
docker logs web                    # everything
docker logs -f web                 # follow (real-time)
docker logs --tail 50 web          # last 50 lines
docker logs --since 10m web        # last 10 minutes
docker logs --timestamps web       # with timestamps
```

::: tip `docker logs -f` = `tail -f`
Most container images write logs to stdout/stderr. `docker logs` captures them. In production — **centralized log aggregation** (Loki, ELK) is preferred.
:::

### Container statistics

```bash
docker stats               # real-time CPU/memory/network
docker stats --no-stream   # a single snapshot

docker inspect web         # full JSON metadata
docker inspect web | jq '.[0].NetworkSettings.IPAddress'
```

---

## 5.5. Volume mount and port mapping

### Bind mount (host → container)

```bash
# Mount an existing directory
docker run -d -v $(pwd):/app -w /app node:20 npm start

# Read-only
docker run -d -v /etc/myapp:/config:ro myapp

# Macros — $(pwd) gives an absolute path
docker run -v $(pwd):/data busybox ls /data
```

### Named volume (Docker-managed)

```bash
# Create a volume
docker volume create pgdata

# Use it
docker run -d -v pgdata:/var/lib/postgresql/data postgres:16

# All of them
docker volume ls
docker volume inspect pgdata
docker volume rm pgdata
```

::: warning Bind mount permissions
The user inside the container (usually `root` or UID 1000) must access the files on the host with the correct permissions. On macOS it usually works; on Linux a `chown` may be needed.
:::

### Port mapping nuances

```bash
docker run -p 8080:80 nginx
# Host 0.0.0.0:8080 → container 80

docker run -p 127.0.0.1:8080:80 nginx
# Accessible only from localhost (secure)

docker run -p 8080:80/udp myudp
# UDP port

docker run -P nginx
# ALL EXPOSED ports on RANDOM host ports
docker ps        # see which port it got
```

---

## 5.6. Environment variables

```bash
# Several -e
docker run -d \
    -e POSTGRES_USER=admin \
    -e POSTGRES_PASSWORD=secret \
    -e POSTGRES_DB=myapp \
    postgres:16

# From an env file
cat > .env <<'EOF'
POSTGRES_USER=admin
POSTGRES_PASSWORD=secret
POSTGRES_DB=myapp
EOF

docker run --env-file .env -d postgres:16
```

::: danger Secrets and env vars
`docker inspect <container>` — shows all env vars in JSON. Passing secrets via env vars is OK for a lab, but **dangerous in production**. Use:
- **Docker secrets** (Swarm)
- **Kubernetes Secrets**
- Vault, AWS Secrets Manager
- Or at minimum: `--env-file` + the file with 600 permissions
:::

---

## 5.7. `Dockerfile` — building an image

### Simple example

```dockerfile
# Dockerfile
FROM alpine:3.19

# Dependency packages
RUN apk add --no-cache curl jq

# Copy the script
COPY healthcheck.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/healthcheck.sh

# Working directory
WORKDIR /app

# Default command
CMD ["/usr/local/bin/healthcheck.sh"]
```

Build and run:
```bash
docker build -t myhealthcheck:1.0 .
docker run --rm myhealthcheck:1.0
```

### Classic directives

| Directive     | Meaning                                       |
|---------------|-----------------------------------------------|
| `FROM`        | Base image (at the start of every Dockerfile) |
| `RUN`         | Run a command during build                     |
| `COPY src dst`| Copy from local into the image                 |
| `ADD src dst` | `COPY` + URL/archive support (less recommended)|
| `WORKDIR /x`  | Set the current directory                     |
| `ENV K=V`     | Environment variable                          |
| `EXPOSE 8080` | Documentation — port exposure (no automatic mapping) |
| `CMD ["..."]` | Default startup command (can be overridden)   |
| `ENTRYPOINT [...]` | Main command (CMD provides arguments)     |
| `USER ali`    | User (important for security)                 |
| `HEALTHCHECK` | Health check command                          |
| `LABEL k=v`   | Metadata                                      |

### `.dockerignore`

```
# .dockerignore — what to SKIP during `COPY .`
node_modules
.git
*.log
.env*
dist
target
```

It reduces build time and improves security (keeps secrets out of the image).

### Multi-stage build — smaller image size

```dockerfile
# Stage 1 — build environment (large, only for building)
FROM rust:1.78 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Stage 2 — runtime (small, only the binary)
FROM debian:12-slim
COPY --from=builder /app/target/release/myapp /usr/local/bin/
CMD ["myapp"]
```

| Approach           | Final image size  |
|--------------------|---------------------|
| `FROM rust:1.78`   | ~1.5 GB             |
| Multi-stage `debian:12-slim` | ~80 MB     |
| Multi-stage `scratch` | ~10 MB           |

::: tip Layer caching
Each `RUN`, `COPY` is a new layer. Layers are cached. **Put frequently changing things later**:
```dockerfile
COPY package.json package-lock.json ./
RUN npm ci          # ← cached, won't rerun unless dependencies change
COPY . .            # ← every time, fresh
```
:::

---

## 5.8. `docker compose` — multi-container app

A single `docker-compose.yml` — the whole stack (web + db + cache + ...).

### `docker-compose.yml` example

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
    build: .                    # from the current Dockerfile
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

### Managing

```bash
docker compose up -d          # start in the background
docker compose ps             # view status
docker compose logs -f web    # log for a single service
docker compose exec db psql -U admin myapp   # interactive
docker compose restart web
docker compose down           # stop + remove containers
docker compose down -v        # + also the volumes
```

### Service names = DNS

Services within Compose find **each other by name**:
```bash
# From inside the web container
curl http://db:5432      # connects to the `db` service
redis-cli -h cache       # to the `cache` service
```

This is Compose's **most magical feature**. No network configuration needed.

---

## 5.9. Managing Docker from Bash scripts

### Parsing container status

```bash
# Status — as a string
status=$(docker inspect -f '{{.State.Status}}' web)
echo "$status"             # running / exited / paused / ...

# More powerful with JSON + jq
docker ps --format json | jq -r 'select(.Names == "web") | .Status'
```

### Health check loop

```bash
#!/usr/bin/env bash
# wait-for-healthy.sh — wait for a container to become healthy

NAME="${1:?Container name required}"
MAX_WAIT=60

for ((i=0; i<MAX_WAIT; i++)); do
    state=$(docker inspect -f '{{.State.Health.Status}}' "$NAME" 2>/dev/null || echo "missing")
    case "$state" in
        healthy)   echo "✓ $NAME healthy"; exit 0 ;;
        starting)  echo "  Starting up... ($i/$MAX_WAIT)" ;;
        unhealthy) echo "✗ $NAME unhealthy"; exit 1 ;;
        missing)   echo "Container not found"; exit 2 ;;
    esac
    sleep 1
done

echo "Timeout"
exit 1
```

### Cleanup pattern (with trap)

```bash
#!/usr/bin/env bash
set -euo pipefail

CONTAINER="test-pg-$$"        # $$ — current PID

cleanup() {
    docker rm -f "$CONTAINER" 2>/dev/null || true
}
trap cleanup EXIT

# Start it
docker run -d --name "$CONTAINER" \
    -e POSTGRES_PASSWORD=test \
    -p 5432:5432 \
    postgres:16

# Wait
sleep 5

# Test work
psql -h localhost -U postgres -c 'SELECT version();'

# trap cleanup is called automatically
```

---

## 5.10. A real example — Test database sandbox

A production-grade pattern: spin up a clean PostgreSQL for each test.

```bash
#!/usr/bin/env bash
#
# with-pg.sh — run each command in a clean PostgreSQL sandbox
#
# Usage:
#   ./with-pg.sh psql -c 'SELECT 1;'
#   ./with-pg.sh ./integration-tests.sh
#

set -euo pipefail

readonly CONTAINER="pg-sandbox-$$"
readonly DB_PORT=15432
readonly DB_PASSWORD="${DB_PASSWORD:-test_$(date +%s)}"

cleanup() {
    local rc=$?
    echo "🧹 Cleaning up the sandbox..."
    docker rm -f "$CONTAINER" > /dev/null 2>&1 || true
    exit "$rc"
}
trap cleanup EXIT INT TERM

echo "📦 Starting the PostgreSQL sandbox..."
docker run -d \
    --name "$CONTAINER" \
    -e POSTGRES_PASSWORD="$DB_PASSWORD" \
    -e POSTGRES_USER=test \
    -e POSTGRES_DB=testdb \
    -p "$DB_PORT:5432" \
    postgres:16 > /dev/null

# Wait until it's ready
echo "⏳ Waiting for the database to be ready..."
for i in {1..30}; do
    if docker exec "$CONTAINER" pg_isready -U test > /dev/null 2>&1; then
        echo "✓ Ready (${i}s)"
        break
    fi
    sleep 1
    [[ $i -eq 30 ]] && { echo "❌ Timeout"; exit 1; }
done

# Export the connection details
export PGHOST=localhost
export PGPORT="$DB_PORT"
export PGUSER=test
export PGPASSWORD="$DB_PASSWORD"
export PGDATABASE=testdb

# Run the user's command
echo "🧪 Running: $*"
"$@"
```

Usage:
```bash
chmod +x with-pg.sh

# Simple test
./with-pg.sh psql -c 'SELECT version();'

# Integration tests
./with-pg.sh ./tests/run-integration.sh

# Interactive shell
./with-pg.sh bash
# PGHOST, PGPORT, ... are exported automatically
```

This script:
- Creates a **clean** sandbox each time it runs
- Patiently waits for PostgreSQL to become ready
- Exposes the connection parameters as env vars
- In every case (success/fail/Ctrl+C) — the sandbox is cleaned up
- Cross-platform, nothing needs to be installed (other than Docker)

---

## 5.11. Cleanup and resource management

Docker's biggest worry is **the disk filling up**. Unused images, volumes, and networks accumulate.

### Classic cleanup commands

```bash
# Clean up things nothing is using
docker container prune        # stopped containers
docker image prune            # dangling images
docker image prune -a         # ALL unused
docker volume prune           # unused volumes
docker network prune          # unused networks

# All together
docker system prune           # stopped + dangling
docker system prune -a        # + unused images
docker system prune -a --volumes   # + volumes (CAUTION — data is lost)
```

### Viewing disk usage

```bash
docker system df              # general report
# TYPE              TOTAL     ACTIVE    SIZE
# Images            42        12        15.2GB
# Containers        18        4         312MB
# Local Volumes     7         3         2.1GB

docker system df -v           # detailed
```

### Automated cleanup (cron)

```cron
# Every Sunday night
0 3 * * 0 docker system prune -af --volumes --filter "until=168h" >> /var/log/docker-cleanup.log 2>&1
```

`--filter "until=168h"` — those older than 168 hours (7 days).

---

## 5.12. Security practices

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

# 3. Restrict capabilities
# docker run --cap-drop=ALL --cap-add=NET_BIND_SERVICE myapp

# 4. Specific tag, not :latest
FROM node:20.11.0-alpine        # ✓ specific
# FROM node:latest              # ❌ moving target

# 5. Image scanning
# docker scout cves myapp:1.0    (Docker Desktop)
# trivy image myapp:1.0           (Trivy)
```

### Best practice checklist

- [ ] Don't use **`:latest`** — a precise version tag for every image
- [ ] Run inside as **non-root** (`USER`)
- [ ] **`.dockerignore`** — skip `.env`, `.git`, secrets files
- [ ] **Multi-stage** — small final image
- [ ] **Image scanning** — `docker scout` or `trivy`
- [ ] **Resource limits** — `--memory`, `--cpus`
- [ ] **Read-only filesystem** when possible
- [ ] **Secrets** — `--env-file` (file with 600), Docker secrets, or Vault

---

## 5.13. Common mistakes

::: danger Classic pitfalls

1. **`localhost` inside a container ≠ the host's localhost.**
   To connect from inside a container to the host:
   - Mac/Windows: `host.docker.internal`
   - Linux: `--add-host=host.docker.internal:host-gateway`

2. **Volume mount UID mismatch.**
   uid 1000 on the host, uid 0 in the container — permission errors. `--user $(id -u):$(id -g)` helps.

3. **The `:latest` tag — dangerous in production.**
   "It worked a week ago" — because `latest` changed. Always use a precise tag.

4. **`docker rm` instead of `docker rmi`.**
   - `rm` — container
   - `rmi` — image

5. **`docker logs` is stdout, not a file.**
   If you do `> /var/log/...` inside the container — Docker won't see it.

6. **Build context is too large.**
   `COPY . .` — the entire current directory is sent as the context. `.dockerignore` is mandatory.

7. **Confusing `CMD` vs `ENTRYPOINT`.**
   - `CMD ["node", "x.js"]` — `docker run myimage` → it starts. `docker run myimage echo hi` — overrides `CMD`.
   - `ENTRYPOINT ["node"]` + `CMD ["x.js"]` — `docker run myimage y.js` → `node y.js`.

8. **Mixing up named vs bind volumes.**
   `-v /host/path:/in/container` — bind (host path).
   `-v myvolume:/in/container` — named volume.

9. **Forgetting network isolation.**
   By default containers are not on the same network — use `docker network create mynet` and `--network mynet`, or use Compose.

10. **Slow file writes on macOS.**
    Bind mounts on macOS are **10-100×** slower than on Linux. If you need performance — a named volume or OrbStack.
:::

---

## 5.14. Exercises

::: tip 🧪 Bashlings — interactive exercises
This chapter's **7** exercises come with auto-checking via the `bashlings` CLI. None of them
require a Docker daemon — they work with syntax and configuration:

```bash
bashlings watch              # start from the first pending exercise
bashlings run docker1        # check a single exercise
bashlings hint docker1       # step-by-step hint
```

Source: [`exercises/15_docker/`](https://github.com/qobulovasror/bashlings/tree/main/exercises/15_docker)
:::

Try the following real-world exercises wherever Docker is installed:

1. **Quick Nginx** — start nginx via `docker run` (on port 8080), confirm `curl localhost:8080` works, and clean up with `docker rm -f`.

2. **Custom Dockerfile** — build an alpine-based image with `curl` and `jq` installed (`-t myhealthcheck`). Run it and see the output of `curl --version`.

3. **Compose** — write a `docker-compose.yml`: nginx + postgres + redis. Run `up -d` and check that all 3 are `running`.

4. **Health wait script** — run `wait-for-healthy.sh` (above) on your machine and write a script that waits for the postgres container to become healthy.

5. **Cleanup automation** — automate `docker system prune -af` every Sunday at 03:00 via cron (with logging).

---

## 5.15. Summary

| Concept                    | Key point                                      |
|----------------------------|------------------------------------------------|
| Image                      | Static template                                |
| Container                  | A running instance of the image                |
| `docker run`               | Start a new container                          |
| `docker ps`                | Running containers                             |
| `docker ps -a`             | + stopped ones                                 |
| `-d`                       | Detached                                       |
| `-it`                      | Interactive + TTY                              |
| `--rm`                     | Auto-cleanup                                   |
| `-p HOST:CONT`             | Port mapping                                   |
| `-v PATH:PATH`             | Volume mount                                   |
| `-e KEY=val`               | Env variable                                   |
| `docker exec -it X bash`   | "Get inside" a container                       |
| `docker logs -f X`         | Real-time logs                                 |
| **`Dockerfile`**           | Recipe for building an image                   |
| **Multi-stage**            | Small final image (10× reduction)              |
| **`docker compose up`**    | Multi-container stack                          |
| `docker inspect -f`        | Metadata for scripts                           |
| `docker system prune -af`  | Clean up                                       |

### 5 key ideas

1. **Image ≠ Container** — class vs instance.
2. **The `:latest` tag is the enemy** — always use a precise version in production.
3. **Multi-stage build** — reduces the final image size by 10-100×.
4. **Docker Compose** — the standard for every local dev environment.
5. **`docker system prune`** regularly — otherwise the disk fills up quickly.

🎉 You now have the skill to **manage containers** and integrate Bash scripts with Docker. In the next and **final chapter** — automating everything in CI/CD via **GitHub Actions**.

> **Next page:** [6. CI/CD — GitHub Actions →](./06-cicd)
