# 11 — Tarmoq buyruqlari

📘 **Kitob bobi:** [`docs/part3/01-network.md`](../../docs/part3/01-network.md)

## Mashqlar ro'yxati

| # | Nomi   | Mavzu                                          | Daraja      |
|---|--------|------------------------------------------------|-------------|
| 1 | `net1` | `curl -fsSL` standart flag kombinatsiyasi      | ★☆☆☆☆       |
| 2 | `net2` | HTTP status code'ni javobdan ajratib olish     | ★★☆☆☆       |
| 3 | `net3` | Retry + timeout bilan ishonchli curl           | ★★☆☆☆       |
| 4 | `net4` | JSON POST so'rov (`-X`, `-H`, `-d`)            | ★★★☆☆       |
| 5 | `net5` | `nc -zv` chiqishidan ochiq portlarni sanash    | ★★★☆☆       |
| 6 | `net6` | URL ro'yxatidan hostname ajratish (`sed`)      | ★★★☆☆       |
| 7 | `net7` | Health-check loop (mock `curl` bilan)          | ★★★★☆       |

> Bu mashqlar **offline-friendly** — haqiqiy internet talab qilmaydi.
> Real tarmoq mashqlari uchun kitob bobining oxiridagi misollarni ko'ring.

## Boshlash

```bash
bashlings watch
```
