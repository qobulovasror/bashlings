# 14 — Cron, at va systemd timer'lar

📘 **Kitob bobi:** [`docs/part3/04-cron.md`](../../docs/part3/04-cron.md)

## Mashqlar ro'yxati

| # | Nomi    | Mavzu                                          | Daraja      |
|---|---------|------------------------------------------------|-------------|
| 1 | `cron1` | Har daqiqada (`* * * * *`)                     | ★☆☆☆☆       |
| 2 | `cron2` | Har kuni aniq vaqtda (`30 3 * * *`)            | ★★☆☆☆       |
| 3 | `cron3` | Step operator (`*/15 * * * *`)                 | ★★☆☆☆       |
| 4 | `cron4` | Day-of-week ro'yxat (`* * * * 1,3,5`)          | ★★★☆☆       |
| 5 | `cron5` | To'liq qator + log redirect                    | ★★★☆☆       |
| 6 | `cron6` | Crontab qatoridan buyruqni ajratish (`cut`)    | ★★★☆☆       |
| 7 | `cron7` | `@daily` shortcut + silent redirect            | ★★★★☆       |

> Mashqlar **cron daemon talab qilmaydi** — sintaksis bilan ishlash uchun.
> Real crontab'ni `crontab -e` orqali tahrirlash kitob bobida tushuntirilgan.

## Boshlash

```bash
bashlings watch
```
