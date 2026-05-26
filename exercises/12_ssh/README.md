# 12 — SSH va remote management

📘 **Kitob bobi:** [`docs/part3/02-ssh.md`](../../docs/part3/02-ssh.md)

## Mashqlar ro'yxati

| # | Nomi   | Mavzu                                          | Daraja      |
|---|--------|------------------------------------------------|-------------|
| 1 | `ssh1` | SSH ulanish komandasi (`-p`, user@host)        | ★☆☆☆☆       |
| 2 | `ssh2` | scp bilan fayl yuklash                          | ★★☆☆☆       |
| 3 | `ssh3` | ssh-keygen — ed25519 kalit yaratish             | ★★☆☆☆       |
| 4 | `ssh4` | ~/.ssh/config bloki (heredoc)                   | ★★★☆☆       |
| 5 | `ssh5` | rsync sinxronlash (`-avz --delete`)             | ★★★☆☆       |
| 6 | `ssh6` | SSH local port forwarding (`-L`)                | ★★★☆☆       |
| 7 | `ssh7` | auth.log parsing — kirgan foydalanuvchilar     | ★★★★☆       |

> Bu mashqlar **offline-friendly** — haqiqiy SSH server talab qilmaydi.
> Komandalarni STRING sifatida qurib chiqarish bilan sintaksisni mustahkamlaysiz.

## Boshlash

```bash
bashlings watch
```
