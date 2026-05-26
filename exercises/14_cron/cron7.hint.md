# 💡 cron7

## 1-bosqich
Cron maxsus shortcut'lar (5 maydon o'rniga):
| Shortcut    | Ekvivalent     |
|-------------|----------------|
| `@yearly`   | `0 0 1 1 *`    |
| `@monthly`  | `0 0 1 * *`    |
| `@weekly`   | `0 0 * * 0`    |
| `@daily`    | `0 0 * * *`    |
| `@hourly`   | `0 * * * *`    |
| `@reboot`   | har bootda     |

## 2-bosqich
Chiqishni butunlay yo'qotish:
```
> /dev/null 2>&1
```
- `> /dev/null` — stdout `/dev/null` ga
- `2>&1`        — stderr ham xuddi shu joyga

Tartib MUHIM: avval stdout redirect, keyin `2>&1`.

## 3-bosqich
Birlashtirish:
```bash
echo "@daily $SCRIPT > /dev/null 2>&1"
```
