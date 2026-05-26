---
title: "Atamalar lug'ati"
description: "Bash, Linux va DevOps atamalari — o'zbekcha izoh va kontekst bilan."
---

# Atamalar lug'ati

Bu sahifa — darslikda uchraydigan barcha texnik atamalar uchun **yagona haqiqat manbai**. Tarjima nomuvofiqligi bo'lmasligi uchun har atama bir xil ishlatiladi.

Atamalar alifbo tartibida.

---

## A

### `alias`
Buyruq uchun qisqartma. `alias ll='ls -la'` — `ll` deb yozsangiz `ls -la` bajariladi. Faqat joriy shell sessiyasida amal qiladi (doimiy uchun `~/.bashrc` ga qo'shing).

### Argument
Skript yoki funksiyaga uzatiladigan qiymat. Bash'da `$1`, `$2`, ... `$@`, `$#`.

### `awk`
Matnli ma'lumotni "ustun-qator" mantig'ida ishlovchi domain-specific dasturlash tili. Misol: `awk '{print $2}'` — har qatorning 2-ustunini chiqaradi.

---

## B

### `bash`
"Bourne Again SHell" — Linux'da default shell. Bizning butun darslik shu shell uchun.

### `bashlings`
Bu loyihaning **CLI runner'i** (Rust'da yozilgan). Mashqlarni avto-tekshiradi, hint render qiladi, watch rejimida ishlaydi.

### `binary`
**Kompilyatsiya qilingan** dastur (matn skript emas). Misol: `/bin/ls` — binary.

### Built-in
Shell'ning O'ZINING ichida bo'lgan buyruq (`cd`, `echo`, `pwd`, `read`, `[`, `test`). Tashqi dastur emas — shuning uchun tez ishlaydi.

---

## C

### Command substitution
`$(cmd)` yoki backtick `\`cmd\`` — buyruqning stdout natijasini bir o'zgaruvchiga olish. Misol: `now=$(date +%F)`.

### Container
Docker'da — image'ning ishlovchi nusxasi. Bir image'dan ko'plab container yaratish mumkin.

### `cron`
Vaqt jadvali bo'yicha takroriy vazifalar ishlovchi tizim daemon'i. `crontab -e` orqali tahrirlanadi.

### Crontab
`cron` uchun konfiguratsiya fayli. Har qator: `minute hour day month dow command`.

### `curl`
HTTP klient — URL'ga so'rov yuborish va javobini olish. Skriptlar uchun `-fsSL` standart kombinatsiyasi.

---

## D

### Daemon
Fonda doimiy ishlovchi jarayon. Misol: `cron`, `sshd`, `dockerd`. Linux'da odatda nomi `d` bilan tugaydi.

### `docker`
Konteynerizatsiya platformasi. Image — shablon, container — uning ishlovchi nusxasi.

### `Dockerfile`
Docker image'ni qurish ko'rsatmasi. `FROM`, `RUN`, `COPY`, `CMD` direktivalari.

---

## E

### `echo`
Argumentlarni stdout'ga chiqarish. Bash built-in. Misol: `echo "Salom"`.

### Environment variable
Jarayonga uzatiladigan kalit-qiymat juftligi. `export VAR=value` orqali eksport qilinadi. Misol: `$PATH`, `$HOME`, `$USER`.

### Exit code (return code)
Buyruq tugagach qaytaradigan butun son (0–255). `0` = muvaffaqiyat, `>0` = xato. `$?` orqali oxirgi exit code'ni olish mumkin.

---

## F

### File descriptor (FD)
Jarayonga ochilgan fayl/oqim raqami. Standartlari:
- `0` = stdin
- `1` = stdout
- `2` = stderr

### `for`
Loop konstruksiyasi. Bash'da: `for x in a b c; do echo "$x"; done`.

### Function
Qayta ishlatiluvchi nomli kod bloki. Sintaksis: `name() { ... }`.

---

## G

### `git`
Tarqalgan versiyalash tizimi. Bizning loyihaga klon qilish va mashqlarni saqlash uchun.

### Glob
Fayl-tizim pattern: `*.sh`, `file?.txt`, `[abc].md`. Bash o'zi kengaytiradi (regex emas).

### `grep`
Matnda PATTERN topish vositasi. `grep -i` (case-insensitive), `-r` (rekursiv), `-c` (sanash).

---

## H

### Heredoc
Multi-line string sintaksisi: `cat <<EOF\n...\nEOF`. `<<'EOF'` (tirnoq bilan) — o'zgaruvchi interpolatsiyasi YO'Q.

### `home` katalog
Foydalanuvchi shaxsiy katalogi (`$HOME` yoki `~`). Misol: `/Users/ali` (macOS) yoki `/home/ali` (Linux).

---

## I

### Image (Docker)
Konteyner uchun fayl-tizim shabloni. **O'qish-uchun** (immutable).

### Interactive mode
Shell foydalanuvchi bilan suhbatda — promptga buyruq kiritadi (vs. **skript rejimi** — fayl ichidagi buyruqlar bajariladi).

---

## J

### `jq`
JSON uchun command-line parser. `jq '.field'` — field olish, `jq -r` — raw (tirnoqsiz).

### Job
Shell tarkibida ishlovchi jarayon. `jobs` — ro'yxat, `bg`/`fg` — boshqaruv.

---

## K

### Kernel
Operatsion tizimning yadrosi — apparatura bilan to'g'ridan-to'g'ri ishlaydi. Linux'da `Linux kernel`; macOS'da `XNU`.

---

## L

### Locale
Til/format sozlamasi (`LC_ALL`, `LANG`). Misol: `en_US.UTF-8`. `awk` va `sort` natijalariga ta'sir qiladi.

### Loop
Takroriy bajarish konstruksiyasi. Bash'da: `for`, `while`, `until`.

---

## M

### `man`
Manual sahifa — buyruq haqida to'liq qo'llanma. `man bash`, `man curl`.

### Marker (I AM NOT DONE)
Bashlings'da har mashq faylida `# I AM NOT DONE` qatori. Mashqni tugatganingizdan keyin o'chiriladi — CLI buni `is_done` indikatori sifatida ishlatadi.

### Multiplexing (SSH)
Bir TCP ulanish ustida ko'p SSH sessiyasi (`ControlMaster`). Tez qayta-ulanish uchun.

---

## P

### `PATH`
Environment variable — buyruqlarni qayerdan qidirish kerakligini ko'rsatadi. `:` bilan ajratilgan kataloglar ro'yxati. Misol: `/usr/local/bin:/usr/bin:/bin`.

### Pipe (`|`)
Bir buyruqning stdout'ini boshqaning stdin'iga ulash. `ls | wc -l` — fayllar soni.

### Pipeline
Pipe orqali bog'langan ikki yoki ko'p buyruq. Misol: `cat log | grep error | sort | uniq -c`.

### PID
Process ID — har ishlovchi jarayonning yagona raqami. `echo $$` — joriy shell'ning PID'i.

### Process
Ishlovchi dastur nusxasi. Har process'ning PID'i, parent (PPID), env va FD'lari bor.

---

## Q

### Quoting
Bash'da string'larni o'rab olish usuli:
- `"..."` qo'shtirnoq — o'zgaruvchi kengayadi, `$(cmd)` ishlaydi
- `'...'` bir tirnoq — hech narsa o'zgarmaydi, literal
- Quotsiz — bo'sh joy va glob expansiyaga sezgir

---

## R

### Redirect
Oqimni faylga yo'naltirish:
- `>` — stdout, fayl QAYTA YOZILADI
- `>>` — stdout, fayl OXIRIGA QO'SHILADI
- `<` — stdin fayldan
- `2>&1` — stderr ham stdout joyiga

### `rsync`
Faqat O'ZGARGAN qismlarni ko'chiruvchi sinxronlash vositasi. Klassik flag: `-avz --delete`.

---

## S

### `scp`
Secure Copy — SSH ustida fayl ko'chirish. Sintaksis: `scp src user@host:dst`.

### `sed`
Stream EDitor — matnda almashtirish va tahrirlash. Misol: `sed 's/old/new/g'`.

### Shebang
Skript faylining birinchi qatori — qaysi interpreter bilan ishga tushishini belgilaydi. Misol: `#!/usr/bin/env bash`.

### Shell
Foydalanuvchi va kernel o'rtasidagi interpretator. Misollar: `bash`, `zsh`, `fish`, `sh`.

### `ShellCheck`
Bash skriptlar uchun static linter. Mumkin xato va anti-pattern'larni topadi.

### Signal
Jarayonga yuboriladigan asinxron xabar. `SIGINT` (Ctrl+C), `SIGTERM` (kibrik tugatish), `SIGKILL` (majburiy o'lim).

### Skript
Matn fayl, ichidagi buyruqlar shell tomonidan birin-ketin bajariladi. Bizning loyihada — `.sh` fayllar.

### `ssh`
Secure SHell — remote serverga shifrlangan ulanish. `ssh user@host`.

### Stage (Docker multi-stage)
`FROM ... AS name` — Dockerfile ichida alohida build bosqichi. Final image'da faqat oxirgi stage qoladi.

### stderr (FD 2)
Standart error oqimi — xato xabarlari odatda shu yerga yo'naltiriladi.

### stdin (FD 0)
Standart kirish oqimi — buyruq foydalanuvchidan yoki pipe'dan oladigan ma'lumot.

### stdout (FD 1)
Standart chiqish oqimi — buyruqning normal natijasi.

### Subshell
Asosiy shell ichida ishlovchi alohida shell jarayon. Qavslar `(...)` ichidagi buyruqlar subshell'da bajariladi.

---

## T

### `tar`
Arxiv vositasi. Klassik flag: `tar -czvf archive.tar.gz dir/` (create, gzip, verbose, file).

### `tee`
Pipe natijasini HAM faylga, HAM stdout'ga yo'naltirish. `echo X | tee log.txt`.

### Terminal
Buyruqlarni kiritish/natijani ko'rish uchun grafik dastur (oyna). Misol: Terminal.app, iTerm2, GNOME Terminal. Shell terminal ICHIDA ishlaydi.

### `trap`
Bash signal handler — skript signal olganda nima qilishni belgilaydi. `trap cleanup EXIT`.

---

## V

### Variable
O'zgaruvchi. Bash'da `name=value` (bo'sh joy YO'Q) bilan o'rnatiladi, `$name` orqali o'qiladi.

### VitePress
Markdown'dan static sayt generatori (Vue.js asosida). Bu darslik shu vosita bilan render qilinadi.

---

## W

### `watch` (bashlings)
Avto-tekshiruv rejimi — fayl saqlanganda mashq qayta tekshiriladi. `bashlings watch`.

### `wget`
HTTP klient (curl muqobili) — fayl yuklashga moslangan. `wget -c` — yarim yuklanganni davom ettirish.

---

## Y

### YAML
"YAML Ain't Markup Language" — ma'lumot serializatsiya formati. Chekinishga bog'liq. Docker compose va GitHub Actions YAML ishlatadi.

---

## Z

### `zsh`
Bash bilan deyarli mos shell — macOS'da default. Bu darslik **bash**'ga mo'ljallangan, lekin ko'p mashqlar zsh'da ham ishlaydi.

---

## Tarjima e'tiborlari

| English        | O'zbekcha (tavsiya etilgan) | Sabab                          |
|----------------|-------------------------------|--------------------------------|
| array          | massiv                        | matematikadan tanish           |
| associative array | lug'at / assotsiativ massiv | ikkalasi ham qabul qilinadi    |
| flag           | flag                          | qisqaroq variant yo'q          |
| shell          | shell                         | "qobiq" tarjimasi noaniq       |
| pipe           | quvur / pipe                  | tex. kontekstda `pipe` qoldiriladi |
| exit code      | exit code / chiqish kodi       | har ikkisi ham mumkin          |
| redirect       | yo'naltirish / redirect        | tarjimada `>` semantikasi yo'qoladi |
| script         | skript                        | ko'p ishlatiladi                |
| process        | jarayon / process              | ikkalasi ham qabul qilinadi    |

Atamani yo'q deb hisoblamasangiz — [Issue oching](https://github.com/qobulovasror/bashlings/issues), qo'shamiz.
