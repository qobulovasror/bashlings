# 16 — CI/CD va GitHub Actions

📘 **Kitob bobi:** [`docs/part3/06-cicd.md`](../../docs/part3/06-cicd.md)

## Mashqlar ro'yxati

| # | Nomi    | Mavzu                                          | Daraja      |
|---|---------|------------------------------------------------|-------------|
| 1 | `cicd1` | Minimal workflow header (`name`, `on:`)        | ★☆☆☆☆       |
| 2 | `cicd2` | Job + steps bloki                              | ★★☆☆☆       |
| 3 | `cicd3` | `actions/checkout@v4` step                     | ★★☆☆☆       |
| 4 | `cicd4` | Matrix strategy (OS bo'yicha)                  | ★★★☆☆       |
| 5 | `cicd5` | Secret'lardan foydalanish (`env:`)             | ★★★☆☆       |
| 6 | `cicd6` | Conditional step (`if: github.ref == ...`)     | ★★★☆☆       |
| 7 | `cicd7` | Workflow log'idan FAIL step'larni topish       | ★★★★☆       |

> Mashqlar **GitHub Actions runner talab qilmaydi** — YAML va shell parsing
> bilan ishlash. Real workflow tahrirlash uchun kitob bobining oxiridagi
> misolni ko'ring.

## Boshlash

```bash
bashlings watch
```
