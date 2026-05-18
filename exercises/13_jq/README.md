# 13 — JSON va `jq`

📘 **Kitob bobi:** [`docs/part3/03-jq.md`](../../docs/part3/03-jq.md)

## Mashqlar ro'yxati

| # | Nomi   | Mavzu                                        | Daraja      |
|---|--------|----------------------------------------------|-------------|
| 1 | `jq1`  | Field access — `.field` + `-r`               | ★★☆☆☆       |
| 2 | `jq2`  | Array iteratsiya — `.[].field`               | ★★★☆☆       |
| 3 | `jq3`  | Filter — `select(...)` + `length`            | ★★★★☆       |
| 4 | `jq4`  | Object yaratish — `{a: .b}` + `-c`           | ★★★★☆       |
| 5 | `jq5`  | Eng kattaani tanlash — `max_by(...)`         | ★★★★☆       |
| 6 | `jq6`  | Yig'indi — `add`                             | ★★★★☆       |

## Prerequisite

```bash
# macOS
brew install jq

# Ubuntu / Debian
sudo apt install jq

# Tekshirish
jq --version    # jq-1.6 yoki kattaroq
```

## Boshlash

```bash
bashlings watch
```
