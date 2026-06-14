---
title: "Network commands — curl, ping, wget"
description: "The HTTP client curl, ping, wget, dig, nc, ss — the everyday tools of DevOps and SRE."
---

# 1. Network commands

> **🎯 What you'll learn in this chapter:**
> - `ping` — the simplest reachability test
> - **`curl`** — the HTTP client (DevOps's everyday tool)
> - GET/POST/PUT/DELETE requests to a REST API
> - JSON body + headers + authentication (Basic, Bearer)
> - `wget` — file download + retry logic
> - `dig`, `nc`, `ss` — DNS and port basics
> - Real example — a **URL monitor** (detecting when a site goes down)
>
> **⏱ Time:** ~40 minutes
> **🧪 Exercises:** `bashlings watch` — 7 interactive exercises ready ([`exercises/11_network/`](https://github.com/qobulovasror/bashlings/tree/main/exercises/11_network))

---

## 1.1. Why do you need networking tools?

A day on the job:
- Check if an API is down — `curl -fsS https://api.example.com/health`
- Test the port of a production database — `nc -zv db.internal 5432`
- Confirm a DNS change — `dig +short example.com`
- Download a file from a site — `wget https://example.com/data.tar.gz`
- Analyze a status code for internal monitoring — `curl -o /dev/null -w "%{http_code}"`

**All** of this is done with standard Unix tools, without writing any Python or Go.

::: tip Key idea
The shell + `curl` combination is the **fastest** path for production debugging. No browser, no IDE — just the terminal.
:::

---

## 1.2. `ping` — the simplest reachability test

```bash
ping google.com
# PING google.com (142.250.184.46): 56 data bytes
# 64 bytes from 142.250.184.46: icmp_seq=0 ttl=115 time=8.5 ms
# ...
```

It sends `ICMP echo` packets and waits for a reply. A basic diagnostic tool.

### Useful flags

| Flag           | Meaning                                  |
|----------------|------------------------------------------|
| `-c <num>`     | Send an exact number of requests, then stop |
| `-W <ms>`      | Timeout (milliseconds)                   |
| `-i <num>`     | Send interval (seconds)                  |
| `-s <bytes>`   | Packet size                              |

```bash
# Send 4 times then stop
ping -c 4 google.com

# Quicker check (3 times, 1 second timeout for each)
ping -c 3 -W 1000 myserver.com
```

::: warning Corporate networks
Many cloud hosts **disable ICMP** — ping not working does not mean the server is down. An HTTP/TCP test is more reliable.
:::

---

## 1.3. `curl` — HTTP client basics

`curl` is the **most important** network command. It's available on every Unix system (or installed with a single command).

### The simplest GET

```bash
curl https://api.github.com/zen
# Practicality beats purity.
```

`curl <URL>` — a GET request by default, prints the response body to stdout.

### Classic useful flags

| Flag       | Meaning                                                        |
|------------|---------------------------------------------------------------|
| `-s`       | **Silent** — hide progress and error messages                  |
| `-S`       | Together with `-s`: show error messages (best practice)        |
| `-f`       | Non-zero exit code if status code is 4xx/5xx (for scripts)     |
| `-i`       | Print response headers together with the body                  |
| `-I`       | Headers only (HEAD request)                                    |
| `-L`       | Automatically follow redirects                                 |
| `-o <file>`| Write to a file                                               |
| `-O`       | Save with the name from the URL                                |
| `-X <method>`| Set the HTTP method (GET/POST/PUT/DELETE/...)                |
| `-H 'K: V'`| Custom header                                                  |
| `-d '...'` | Request body (POST by default)                                |
| `-u user:pass` | Basic auth                                                |
| `-w "format"`| Final data (status code, time, size)                        |

### The standard combination for scripts

```bash
curl -fsSL https://example.com/install.sh | bash
```

| Flag | Reason                                          |
|------|------------------------------------------------|
| `-f` | If 404, don't pipe the script into bash         |
| `-s` | Turn off progress bar noise                    |
| `-S` | But make real errors visible                   |
| `-L` | Follow redirects (download servers)            |

::: tip In production, `-fsSL` is the standard
This combination is the most common `curl` invocation in the shell. Remember it: **`-fsSL`**.
:::

---

## 1.4. Response inspecting — status code and timing

### Status code only

```bash
curl -s -o /dev/null -w "%{http_code}\n" https://example.com
# 200
```

`%{http_code}` is a special token in the `-w` format. Other useful tokens:

| Token                  | Meaning                              |
|------------------------|--------------------------------------|
| `%{http_code}`         | HTTP status                          |
| `%{time_total}`        | Total time (seconds)                 |
| `%{time_connect}`      | TCP connect time                     |
| `%{time_starttransfer}`| TTFB (Time To First Byte)            |
| `%{size_download}`     | Downloaded size (bytes)              |
| `%{url_effective}`     | Final URL (after redirects)          |

### Full analysis

```bash
curl -s -o /dev/null -w \
  "status: %{http_code}\ntime: %{time_total}s\nsize: %{size_download} bytes\n" \
  https://example.com
```

```text
status: 200
time: 0.234s
size: 1256 bytes
```

### Viewing headers

```bash
curl -I https://example.com
# HTTP/2 200
# content-type: text/html; charset=UTF-8
# cache-control: max-age=604800
# ...
```

`-I` — headers only (HEAD request). Ideal for checking the server type, cache settings, and redirects.

---

## 1.5. POST + JSON body — working with an API

### Simple POST (form-urlencoded)

```bash
curl -X POST https://httpbin.org/post \
  -d 'name=Ali&age=25'
```

When the `-d` flag is given, `curl` automatically does a POST.

### Sending a JSON body

```bash
curl -X POST https://httpbin.org/post \
  -H "Content-Type: application/json" \
  -d '{"name":"Ali","age":25}'
```

### JSON body from a file

```bash
cat > payload.json <<EOF
{
  "user": "Ali",
  "city": "Toshkent"
}
EOF

curl -X POST https://httpbin.org/post \
  -H "Content-Type: application/json" \
  -d @payload.json
```

`@filename` — take the content from a file.

### PUT and DELETE

```bash
curl -X PUT https://api.example.com/users/42 \
  -H "Content-Type: application/json" \
  -d '{"name":"Ali"}'

curl -X DELETE https://api.example.com/users/42
```

---

## 1.6. Authentication

### Basic auth

```bash
curl -u username:password https://api.example.com/data
```

Or interactively (it asks for the password):
```bash
curl -u username https://api.example.com/data
```

### Bearer token (the most widespread)

```bash
TOKEN="xyz123abc..."
curl -H "Authorization: Bearer $TOKEN" https://api.example.com/me
```

### Cookie

```bash
# Save cookies to a file and use them in subsequent requests
curl -c cookies.txt -d "user=ali&pass=secret" https://example.com/login
curl -b cookies.txt https://example.com/dashboard
```

::: warning Secrets and shell history
If you pass the password as an argument (`-u user:password`), it ends up in your **bash history**. A better way:
```bash
read -sp "Password: " PASS
curl -u "user:$PASS" https://...
```

Or an environment variable + a secrets manager.
:::

---

## 1.7. `wget` — downloading files

`wget` is an alternative to `curl`, but it's designed for **downloading files**.

```bash
# Simple download
wget https://example.com/data.tar.gz

# Save under a different name
wget -O myfile.tar.gz https://example.com/data.tar.gz

# Quiet mode (no progress)
wget -q https://example.com/file

# Retry — try 5 times, 10s between each
wget --tries=5 --wait=10 https://example.com/file

# Recursive (archiving a site) — warning: dangerous for large sites
wget -r -np -k https://example.com/docs/
```

### `curl` vs `wget`

| Feature                         | `curl`                  | `wget`                |
|---------------------------------|-------------------------|------------------------|
| Default output                  | stdout                  | file                   |
| HTTP methods (POST, PUT, ...)   | ✅ full                 | limited                |
| Retry / recursive download      | manual                  | ✅ built-in            |
| Resume a partial download       | `-C -`                  | `-c`                   |
| Working with headers            | ✅ core feature         | limited                |

::: tip Rule of thumb
**Working with an API** — use `curl`. **Downloading a file from a site** — use `wget`.
:::

---

## 1.8. DNS — `dig`, `host`, `nslookup`

`dig` (Domain Information Groper) is the most powerful tool for DNS queries.

```bash
# Simple A record query
dig example.com
# (long output — all sections)

# IP only — shortened
dig +short example.com
# 93.184.216.34

# MX record (mail server)
dig +short MX gmail.com

# NS record (name servers)
dig +short NS example.com

# Through a different DNS server (Google's 8.8.8.8)
dig @8.8.8.8 example.com

# Reverse DNS
dig -x 8.8.8.8
```

### `host` and `nslookup` — shortened

```bash
host example.com
# example.com has address 93.184.216.34
# example.com has IPv6 address 2606:2800:220:1:248:1893:25c8:1946

nslookup example.com
# (the old style, still in use)
```

### Real example — checking DNS caching

```bash
# 6 times at 5-second intervals
for i in $(seq 1 6); do
    dig +short example.com
    sleep 5
done
```

If the IP comes back as different answers — there is load balancing or round-robin DNS.

---

## 1.9. `nc` (netcat) — the Swiss army knife

`nc` is a universal tool for TCP/UDP.

### Checking whether a port is open

```bash
# -z = zero-I/O (check only)
# -v = verbose
nc -zv example.com 80
# Connection to example.com port 80 [tcp/http] succeeded!

nc -zv example.com 443
# Connection to example.com port 443 [tcp/https] succeeded!

nc -zv example.com 22
# nc: connectx to example.com port 22 (tcp) failed: Operation timed out
```

### With a timeout (for proper scripts)

```bash
nc -zv -w 3 example.com 80   # 3 second timeout
```

### Quick TCP server (for debugging)

```bash
# Terminal A — listen
nc -l 8080

# Terminal B — connect
echo "salom" | nc localhost 8080

# Terminal A now receives "salom"
```

::: tip Production debug
Check a database port — `nc -zv db.internal 5432`. The fastest "is the DB working?" test.
:::

---

## 1.10. `ss` and `netstat` — local ports

### `ss` — modern (Linux)

```bash
# Listening ports
ss -tlnp
# t — TCP, l — listening, n — numbers (don't DNS resolve), p — PID

# All TCP connections
ss -tan

# UDP ports
ss -ulnp
```

### `netstat` — the old classic

```bash
# Listening TCP ports
netstat -tlnp     # Linux
netstat -an | grep LISTEN   # macOS (BSD netstat is different)

# All TCP connections
netstat -tan
```

### `lsof` — the most informative (port + process)

```bash
# Which process is using port 8080?
lsof -i :8080
# COMMAND   PID  USER   FD   TYPE  NODE NAME
# node    12345 mac    23u  IPv6  ...  TCP *:8080 (LISTEN)
```

::: tip Classic debug
"Port 8080 is busy" → `lsof -i :8080` → PID → `kill PID`.
:::

---

## 1.11. Real example — URL Monitor

A complete production-grade example:

```bash
#!/usr/bin/env bash
#
# monitor.sh — check several URLs every 60 seconds
#

set -euo pipefail

readonly URLS=(
    "https://example.com"
    "https://api.github.com/zen"
    "https://httpbin.org/status/200"
)

readonly TIMEOUT=10           # seconds
readonly INTERVAL=60          # check interval
readonly LOG_FILE="${LOG_FILE:-/tmp/monitor.log}"

# Colors (if TTY)
if [[ -t 1 ]]; then
    GREEN='\033[32m'; RED='\033[31m'; RESET='\033[0m'
else
    GREEN=''; RED=''; RESET=''
fi

log() {
    local ts; ts=$(date '+%Y-%m-%d %H:%M:%S')
    printf '[%s] %s\n' "$ts" "$*" | tee -a "$LOG_FILE"
}

check_url() {
    local url="$1"
    local status time_total
    
    # -s silent, -o /dev/null discard the body,
    # capture the format with -w, --max-time timeout
    if ! read -r status time_total < <(
        curl -s -o /dev/null \
            --max-time "$TIMEOUT" \
            -w '%{http_code} %{time_total}\n' \
            "$url" 2>/dev/null
    ); then
        echo "TIMEOUT 0"
        return
    fi
    echo "$status $time_total"
}

check_all() {
    for url in "${URLS[@]}"; do
        read -r status time_total < <(check_url "$url")
        
        if [[ "$status" == "200" ]]; then
            printf '%b✓%b %s — %s in %ss\n' "$GREEN" "$RESET" "$url" "$status" "$time_total"
            log "OK $url $status ${time_total}s"
        else
            printf '%b✗%b %s — %s\n' "$RED" "$RESET" "$url" "$status"
            log "FAIL $url $status"
        fi
    done
}

# Main loop
log "Monitor boshlandi (PID: $$)"
trap 'log "Monitor to`xtatildi"; exit 0' INT TERM

while true; do
    echo "──── $(date '+%H:%M:%S') ────"
    check_all
    sleep "$INTERVAL"
done
```

Running it:
```bash
chmod +x monitor.sh
./monitor.sh             # foreground
./monitor.sh &           # background
nohup ./monitor.sh &     # keep going even if the terminal closes
```

### What does this script do?

| Feature                            | Where                                  |
|------------------------------------|----------------------------------------|
| Multiple URLs                      | the `URLS=()` array                    |
| Timeout per request                | `--max-time $TIMEOUT`                  |
| Get status and time in one shot    | `curl -w '%{http_code} %{time_total}'` |
| Color (TTY only)                   | the `[[ -t 1 ]]` check                 |
| Write to a log file too            | `tee -a "$LOG_FILE"`                   |
| Graceful shutdown                  | `trap ... INT TERM`                    |
| Infinite loop                      | `while true; do ... sleep ... done`    |

---

## 1.12. Common mistakes

::: danger Classic pitfalls

1. **Hiding errors with `-s`.**
   `curl -s` mode also hides error messages. `curl -sS` — silent + show errors. Always use the **`-sS`** combination.

2. **Not checking the status code in the shell.**
   ```bash
   curl https://api.com/fail   # exit 0, but status 500!
   curl -f https://api.com/fail # exit 22 (4xx/5xx)
   ```

3. **Forgetting redirects.**
   `curl https://github.com` → 301. Your script won't see the redirect. Use `curl -L`.

4. **Passing the password as an argument.**
   `curl -u user:secret` — it's stored in bash history. Get it via `read -sp`.

5. **Carelessness with `wget -r`.**
   Recursive — it may download millions of files. Add `--max-depth=3` and `--no-parent`.

6. **macOS netstat and Linux netstat — different flags.**
   `netstat -tlnp` works on Linux, but not on macOS. `ss` is Linux-only; on macOS use `lsof -i`.

7. **Be careful with DNS cache.**
   `dig` sends a fresh query every time. But the system may rely on the `getent` or `/etc/hosts` cache.

8. **`nc` versions differ.**
   `nc -z` (zero-I/O) exists in GNU netcat and also in BSD netcat (macOS). `nc -k` (keep-listening) is only in BSD. Check your version with `nc -h`.
:::

---

## 1.13. Exercises

::: tip 🧪 Bashlings — interactive exercises
This chapter's **7** exercises come with auto-checking via the `bashlings` CLI. All are
**offline-friendly** — they don't require a real internet connection:

```bash
bashlings watch              # start from the first pending exercise
bashlings run net1           # check a single exercise
bashlings hint net1          # step-by-step hint
```

Source: [`exercises/11_network/`](https://github.com/qobulovasror/bashlings/tree/main/exercises/11_network)
:::

Try the following real-world exercises somewhere with internet access:

1. **Status checker** — print the status codes for the URLs `https://httpbin.org/status/200`, `https://httpbin.org/status/404`, `https://httpbin.org/status/500` (each on a separate line).

2. **Response time** — measure the TTFB (`%{time_starttransfer}`) for some site and print it in the format "TTFB: X.XXs".

3. **JSON POST** — send a JSON body (`{"city": "Toshkent"}`) to `https://httpbin.org/post` and extract `headers.Content-Type` from the response with `jq` (`jq` is in the next chapter).

4. **Port scanner** — write a mini script that checks ports 22, 80, 443, 8080 on a host with `nc -zv`. For each port, print "OPEN" or "CLOSED".

5. **DNS comparator** — check one domain against 3 different DNS servers (`8.8.8.8` Google, `1.1.1.1` Cloudflare, `9.9.9.9` Quad9) and print whether the answers are different or the same.

---

## 1.14. Summary

| Concept                         | Key point                                          |
|---------------------------------|----------------------------------------------------|
| `ping -c N host`                | ICMP reachability check                            |
| `curl URL`                      | Simple GET                                         |
| **`curl -fsSL URL`**            | The standard combination for scripts               |
| `curl -X POST -d ...`           | POST request                                       |
| `curl -H "K: V"`                | Custom header                                      |
| `curl -u user:pass`             | Basic auth                                         |
| `curl -H "Authorization: Bearer $T"` | Token auth                                     |
| `curl -w "%{http_code}"`        | Status + metrics                                   |
| `curl -L`                       | Follow redirects                                   |
| `wget URL`                      | Download a file                                    |
| `wget -c URL`                   | Resume a partial download                          |
| `dig +short example.com`        | Simple DNS query                                   |
| `nc -zv host port`              | Check whether a port is open                        |
| `lsof -i :8080`                 | Which process is using port 8080?                  |

### 5 key ideas

1. **`curl -fsSL`** — the standard combination used in every script
2. **The `-w` flag** — capture the status code and timing in a single command
3. **Working with an API:** `curl` supports every HTTP method (GET/POST/PUT/DELETE/PATCH/...)
4. **`nc -zv host port`** — the fastest "is the port open?" test
5. **`dig +short`** — every production engineer should know this for DNS debugging

🎉 Now you have the basics of **working with a REST API**. In the next chapter — connecting to remote servers over **SSH** and transferring files with `scp`/`rsync`.

> **Next page:** [2. SSH and remote management →](./02-ssh)
