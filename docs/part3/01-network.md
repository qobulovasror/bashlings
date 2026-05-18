---
title: "Tarmoq buyruqlari — curl, ping, wget"
description: "HTTP klient curl, ping, wget, dig, nc, ss — DevOps va SRE'ning kundalik vositalari."
---

# 1. Tarmoq buyruqlari

> **🎯 Bu bobda nimani o'rganasiz:**
> - `ping` — eng oddiy reachability test
> - **`curl`** — HTTP klient (DevOps'ning kundalik vositasi)
> - REST API'ga GET/POST/PUT/DELETE so'rovlar
> - JSON body + headers + autentifikatsiya (Basic, Bearer)
> - `wget` — fayl yuklash + retry mantiq
> - `dig`, `nc`, `ss` — DNS va port asoslari
> - Real misol — **URL monitor** (sayt yiqilganini aniqlash)
>
> **⏱ Vaqt:** ~40 daqiqa
> **🧪 Mashqlar:** `bashlings watch 11_network` (kelajak sprint)

---

## 1.1. Tarmoq vositalari nima uchun kerak?

Bir kun ish:
- API yiqilganini tekshirish — `curl -fsS https://api.example.com/health`
- Production database'ning portini sinash — `nc -zv db.internal 5432`
- DNS o'zgarishini tasdiqlash — `dig +short example.com`
- Saytdan fayl yuklash — `wget https://example.com/data.tar.gz`
- Internal monitoring uchun status code tahlili — `curl -o /dev/null -w "%{http_code}"`

Bularning **hammasi** standart Unix vositalari bilan, hech qanday Python yoki Go yozmasdan amalga oshiriladi.

::: tip Asosiy g'oya
Shell + `curl` kombinatsiyasi — production debugging uchun **eng tezkor** yo'l. Browser ochmasdan, IDE'siz, faqat terminal.
:::

---

## 1.2. `ping` — eng oddiy reachability test

```bash
ping google.com
# PING google.com (142.250.184.46): 56 data bytes
# 64 bytes from 142.250.184.46: icmp_seq=0 ttl=115 time=8.5 ms
# ...
```

`ICMP echo` paketlarini yuboradi va javobni kutadi. Asosiy diagnostik vosita.

### Foydali flaglar

| Flag           | Mazmuni                                  |
|----------------|------------------------------------------|
| `-c <son>`     | Aniq son so'rov yuborib chiqarish        |
| `-W <ms>`      | Timeout (millisekund)                    |
| `-i <son>`     | Yuboruvchi oraliq (soniya)               |
| `-s <bayt>`    | Paket hajmi                              |

```bash
# 4 marta yuborib chiqish
ping -c 4 google.com

# Tezroq tekshirish (3 ta, 1 soniya timeout har bittasi uchun)
ping -c 3 -W 1000 myserver.com
```

::: warning Korporativ tarmoqlar
Ko'pchilik bulutli hostlar **ICMP'ni o'chirib qo'yadi** — ping ishlamasligi server yiqilganini anglatmaydi. HTTP/TCP testi ishonchliroq.
:::

---

## 1.3. `curl` — HTTP klient asoslari

`curl` — **eng muhim** tarmoq buyrug'i. Har Unix tizimda mavjud (yoki bir buyruq bilan o'rnatiladi).

### Eng oddiy GET

```bash
curl https://api.github.com/zen
# Practicality beats purity.
```

`curl <URL>` — sukut bo'yicha GET so'rov, response body'ni stdout'ga chiqaradi.

### Klassik foydali flaglar

| Flag       | Mazmuni                                                       |
|------------|---------------------------------------------------------------|
| `-s`       | **Silent** — progress va xato xabarlarini yashirish            |
| `-S`       | `-s` bilan birga: xato xabarlarni ko'rsatish (best practice)   |
| `-f`       | Status code 4xx/5xx bo'lsa exit code non-zero (skriptlar uchun)|
| `-i`       | Response headers'ni body bilan birga chiqarish                 |
| `-I`       | Faqat headers (HEAD so'rov)                                    |
| `-L`       | Redirect'larni avtomatik kuzatish                              |
| `-o <fayl>`| Faylga yozish                                                  |
| `-O`       | URL'dagi nomi bilan saqlash                                    |
| `-X <metod>`| HTTP metodni belgilash (GET/POST/PUT/DELETE/...)              |
| `-H 'K: V'`| Custom header                                                  |
| `-d '...'` | Request body (sukut bo'yicha POST qiladi)                      |
| `-u user:pass` | Basic auth                                                |
| `-w "format"`| Yakuniy ma'lumotlar (status code, vaqt, hajm)                |

### Skript uchun standart kombinatsiya

```bash
curl -fsSL https://example.com/install.sh | bash
```

| Flag | Sabab                                          |
|------|------------------------------------------------|
| `-f` | 404 bo'lsa script'ni bash'ga uzatmaslik         |
| `-s` | Progress bar shovqinini o'chirish              |
| `-S` | Lekin haqiqiy xatolar ko'rinsin                |
| `-L` | Redirect'larni kuzatish (download serverlari)  |

::: tip Production'da `-fsSL` — standart
Bu kombinatsiya shellda eng ko'p uchraydigan `curl` invokatsiyasi. Esda saqlang: **`-fsSL`**.
:::

---

## 1.4. Response inspecting — status code va vaqtlar

### Faqat status code

```bash
curl -s -o /dev/null -w "%{http_code}\n" https://example.com
# 200
```

`%{http_code}` — `-w` format'dagi maxsus token. Boshqa foydali tokenlar:

| Token                  | Mazmuni                              |
|------------------------|--------------------------------------|
| `%{http_code}`         | HTTP status                          |
| `%{time_total}`        | Umumiy vaqt (soniya)                 |
| `%{time_connect}`      | TCP connect vaqti                    |
| `%{time_starttransfer}`| TTFB (Time To First Byte)            |
| `%{size_download}`     | Yuklangan hajm (bayt)                |
| `%{url_effective}`     | Yakuniy URL (redirect'lardan keyin)  |

### To'liq tahlil

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

### Headers ko'rish

```bash
curl -I https://example.com
# HTTP/2 200
# content-type: text/html; charset=UTF-8
# cache-control: max-age=604800
# ...
```

`-I` — faqat headers (HEAD so'rov). Server tipini, cache sozlamalarini, redirectlarni tekshirish uchun ideal.

---

## 1.5. POST + JSON body — API bilan ishlash

### Oddiy POST (form-urlencoded)

```bash
curl -X POST https://httpbin.org/post \
  -d 'name=Ali&age=25'
```

`-d` flag berilsa, `curl` avtomatik POST qiladi.

### JSON body yuborish

```bash
curl -X POST https://httpbin.org/post \
  -H "Content-Type: application/json" \
  -d '{"name":"Ali","age":25}'
```

### Faylda JSON body

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

`@filename` — fayldan content olish.

### PUT va DELETE

```bash
curl -X PUT https://api.example.com/users/42 \
  -H "Content-Type: application/json" \
  -d '{"name":"Ali"}'

curl -X DELETE https://api.example.com/users/42
```

---

## 1.6. Autentifikatsiya

### Basic auth

```bash
curl -u username:password https://api.example.com/data
```

Yoki interaktiv (parol so'raydi):
```bash
curl -u username https://api.example.com/data
```

### Bearer token (eng keng tarqalgan)

```bash
TOKEN="xyz123abc..."
curl -H "Authorization: Bearer $TOKEN" https://api.example.com/me
```

### Cookie

```bash
# Cookie'larni faylga saqlash va keyingi so'rovlarda ishlatish
curl -c cookies.txt -d "user=ali&pass=secret" https://example.com/login
curl -b cookies.txt https://example.com/dashboard
```

::: warning Secrets va shell history
Parolni argument sifatida bersangiz (`-u user:password`), u **bash history**'ga tushadi. Yaxshiroq yo'l:
```bash
read -sp "Parol: " PASS
curl -u "user:$PASS" https://...
```

Yoki environment variable + secrets manager.
:::

---

## 1.7. `wget` — fayl yuklash

`wget` `curl` ga muqobil, lekin **fayl yuklash** uchun mo'ljallangan.

```bash
# Sodda yuklash
wget https://example.com/data.tar.gz

# Boshqa nom bilan saqlash
wget -O myfile.tar.gz https://example.com/data.tar.gz

# Tinch rejim (progress yo'q)
wget -q https://example.com/file

# Retry — 5 marta urinish, har biri orasida 10s
wget --tries=5 --wait=10 https://example.com/file

# Recursive (sayt arxivlash) — diqqat: katta saytlar uchun xavfli
wget -r -np -k https://example.com/docs/
```

### `curl` vs `wget`

| Xususiyat                       | `curl`                  | `wget`                |
|---------------------------------|-------------------------|------------------------|
| Default chiqish                 | stdout                  | fayl                   |
| HTTP method'lar (POST, PUT, ...)| ✅ to'liq               | cheklangan             |
| Retry / recursive download      | manual                  | ✅ built-in            |
| Resume yarim tushgan yuklash    | `-C -`                  | `-c`                   |
| Header'lar bilan ishlash        | ✅ asosiy               | cheklangan             |

::: tip Qoida
**API bilan ishlasangiz** — `curl`. **Saytdan fayl yuklasangiz** — `wget`.
:::

---

## 1.8. DNS — `dig`, `host`, `nslookup`

`dig` (Domain Information Groper) — DNS so'rovlari uchun eng kuchli vosita.

```bash
# Sodda A record so'rovi
dig example.com
# (uzun chiqish — barcha sections)

# Faqat IP — qisqartirilgan
dig +short example.com
# 93.184.216.34

# MX record (mail server)
dig +short MX gmail.com

# NS record (name servers)
dig +short NS example.com

# Boshqa DNS server orqali (Google's 8.8.8.8)
dig @8.8.8.8 example.com

# Teskari (reverse) DNS
dig -x 8.8.8.8
```

### `host` va `nslookup` — qisqartirilgan

```bash
host example.com
# example.com has address 93.184.216.34
# example.com has IPv6 address 2606:2800:220:1:248:1893:25c8:1946

nslookup example.com
# (eski uslub, hali ham ishlatiladi)
```

### Real misol — DNS caching tekshirish

```bash
# 5 sekundlik intervallar bilan 6 marta
for i in $(seq 1 6); do
    dig +short example.com
    sleep 5
done
```

Agar IP turli javoblardan kelsa — load balancing yoki round-robin DNS bor.

---

## 1.9. `nc` (netcat) — Swiss army knife

`nc` — TCP/UDP uchun universal vosita.

### Port band ekanligini tekshirish

```bash
# -z = zero-I/O (faqat tekshirish)
# -v = verbose
nc -zv example.com 80
# Connection to example.com port 80 [tcp/http] succeeded!

nc -zv example.com 443
# Connection to example.com port 443 [tcp/https] succeeded!

nc -zv example.com 22
# nc: connectx to example.com port 22 (tcp) failed: Operation timed out
```

### Timeout bilan (kerakli skriptlar uchun)

```bash
nc -zv -w 3 example.com 80   # 3 soniya timeout
```

### Quick TCP server (debug uchun)

```bash
# Terminal A — listen
nc -l 8080

# Terminal B — connect
echo "salom" | nc localhost 8080

# Terminal A endi "salom" qabul qiladi
```

::: tip Production debug
Database portini tekshirish — `nc -zv db.internal 5432`. Eng tezkor "DB ishlayaptimi?" testi.
:::

---

## 1.10. `ss` va `netstat` — lokal portlar

### `ss` — zamonaviy (Linux)

```bash
# Listen qilayotgan portlar
ss -tlnp
# t — TCP, l — listening, n — raqamlar (DNS resolve qilmaslik), p — PID

# Hamma TCP ulanishlar
ss -tan

# UDP portlar
ss -ulnp
```

### `netstat` — eski klassik

```bash
# Listening TCP portlar
netstat -tlnp     # Linux
netstat -an | grep LISTEN   # macOS (BSD netstat boshqacha)

# Hamma TCP ulanishlar
netstat -tan
```

### `lsof` — eng informativ (port + process)

```bash
# 8080 portni qaysi jarayon ishlatadi?
lsof -i :8080
# COMMAND   PID  USER   FD   TYPE  NODE NAME
# node    12345 mac    23u  IPv6  ...  TCP *:8080 (LISTEN)
```

::: tip Klassik debug
"8080 port band" → `lsof -i :8080` → PID → `kill PID`.
:::

---

## 1.11. Real misol — URL Monitor

To'liq production-grade misol:

```bash
#!/usr/bin/env bash
#
# monitor.sh — bir nechta URL'ni har 60 soniyada tekshirib turish
#

set -euo pipefail

readonly URLS=(
    "https://example.com"
    "https://api.github.com/zen"
    "https://httpbin.org/status/200"
)

readonly TIMEOUT=10           # soniya
readonly INTERVAL=60          # tekshirish oralig'i
readonly LOG_FILE="${LOG_FILE:-/tmp/monitor.log}"

# Ranglar (TTY bo'lsa)
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
    
    # -s silent, -o /dev/null body'ni tashlash,
    # -w bilan formatni ushlash, --max-time timeout
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

# Asosiy loop
log "Monitor boshlandi (PID: $$)"
trap 'log "Monitor to`xtatildi"; exit 0' INT TERM

while true; do
    echo "──── $(date '+%H:%M:%S') ────"
    check_all
    sleep "$INTERVAL"
done
```

Ishga tushirish:
```bash
chmod +x monitor.sh
./monitor.sh             # foreground
./monitor.sh &           # background
nohup ./monitor.sh &     # terminal yopilsa ham davom etsin
```

### Bu skript nima qiladi?

| Xususiyat                          | Qaerda                                |
|------------------------------------|----------------------------------------|
| Multiple URL                       | `URLS=()` massiv                       |
| Timeout har so'rov uchun           | `--max-time $TIMEOUT`                  |
| Status va vaqtni bir paketda olish | `curl -w '%{http_code} %{time_total}'` |
| Color (faqat TTY)                  | `[[ -t 1 ]]` check                     |
| Log faylga ham yozish              | `tee -a "$LOG_FILE"`                   |
| Graceful shutdown                  | `trap ... INT TERM`                    |
| Cheksiz loop                       | `while true; do ... sleep ... done`    |

---

## 1.12. Tez-tez uchraydigan xatolar

::: danger Klassik tuzoqlar

1. **`-s` bilan xatolarni yashirish.**
   `curl -s` rejimi xato xabarlarini ham yashiradi. `curl -sS` — silent + show errors. Har doim **`-sS`** kombinatsiyasi.

2. **Status code'ni shell'da tekshirmaslik.**
   ```bash
   curl https://api.com/fail   # exit 0, lekin status 500!
   curl -f https://api.com/fail # exit 22 (4xx/5xx)
   ```

3. **Redirect'larni unutish.**
   `curl https://github.com` → 301. Sizning skriptingiz redirect ko'rmaydi. `curl -L` ishlating.

4. **Parolni argument sifatida berish.**
   `curl -u user:secret` — bash history'da saqlanadi. `read -sp` orqali oling.

5. **`wget -r` ehtiyotsizlik.**
   Recursive — millionlab fayl yuklashi mumkin. `--max-depth=3` va `--no-parent` qo'shing.

6. **macOS netstat va Linux netstat — boshqa flaglar.**
   `netstat -tlnp` Linux'da ishlaydi, macOS'da ishlamaydi. `ss` Linux-only, macOS'da `lsof -i` ishlating.

7. **DNS cache ehtiyotkorlik.**
   `dig` har gal yangi so'rov yuboradi. Lekin tizim `getent` yoki `/etc/hosts` cache'iga ishonishi mumkin.

8. **`nc` versiyalari farqli.**
   `nc -z` (zero-I/O) — GNU netcat'da bor, BSD netcat (macOS)'da ham bor. `nc -k` (keep-listening) faqat BSD'da. Versiyangizni `nc -h` bilan tekshiring.
:::

---

## 1.13. Mashqlar

> 🧪 Kelajakda `bashlings watch 11_network` paketida.

1. **Status checker** — `https://httpbin.org/status/200`, `https://httpbin.org/status/404`, `https://httpbin.org/status/500` URL'lari uchun status code'larni chiqaring (har bittasi alohida qatorda).

2. **Response time** — biror sayt uchun TTFB (`%{time_starttransfer}`) ni o'lchang va "TTFB: X.XXs" formatida chiqaring.

3. **JSON POST** — `https://httpbin.org/post` ga JSON body yuboring (`{"city": "Toshkent"}`) va javobdan `headers.Content-Type` ni `jq` bilan ajratib oling (`jq` keyingi bobda).

4. **Port scanner** — bir hostda 22, 80, 443, 8080 portlarini `nc -zv` bilan tekshiruvchi mini skript yozing. Har port uchun "OPEN" yoki "CLOSED" chiqarsin.

5. **DNS comparator** — bir domeni 3 ta turli DNS serverda (`8.8.8.8` Google, `1.1.1.1` Cloudflare, `9.9.9.9` Quad9) tekshirib, javoblar farqli yoki teng ekanligini chiqarsin.

---

## 1.14. Xulosa

| Tushuncha                       | Asosiy nuqta                                       |
|---------------------------------|----------------------------------------------------|
| `ping -c N host`                | ICMP reachability tekshiruv                        |
| `curl URL`                      | Sodda GET                                          |
| **`curl -fsSL URL`**            | Skriptlar uchun standart kombinatsiya              |
| `curl -X POST -d ...`           | POST so'rov                                        |
| `curl -H "K: V"`                | Custom header                                      |
| `curl -u user:pass`             | Basic auth                                         |
| `curl -H "Authorization: Bearer $T"` | Token auth                                     |
| `curl -w "%{http_code}"`        | Status + metrika                                   |
| `curl -L`                       | Redirect'larni kuzatish                            |
| `wget URL`                      | Fayl yuklash                                       |
| `wget -c URL`                   | Yarim tushgan'ni davom ettirish                    |
| `dig +short example.com`        | Sodda DNS so'rovi                                  |
| `nc -zv host port`              | Port band-bandligini tekshirish                    |
| `lsof -i :8080`                 | 8080 ni qaysi jarayon ishlatadi?                   |

### 5 ta asosiy g'oya

1. **`curl -fsSL`** — har skriptda ishlatiladigan standart kombinatsiya
2. **`-w` flag** — status code va vaqtlarni bir buyruqda ushlash
3. **API bilan ishlash:** `curl` HTTP'ning hamma metodini qo'llab-quvvatlaydi (GET/POST/PUT/DELETE/PATCH/...)
4. **`nc -zv host port`** — eng tezkor "port band-bandligini" testi
5. **`dig +short`** — DNS debug'i uchun har production muhandisi bilishi kerak

🎉 Endi sizda **REST API bilan ishlash** asoslari bor. Keyingi bobda — **SSH** orqali masofadagi serverlarga ulanish va `scp`/`rsync` orqali fayllarni ko'chirish.

> **Keyingi sahifa:** [2. SSH va remote management →](./02-ssh)
