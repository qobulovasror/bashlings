# 💡 cron7

## Step 1
Special cron shortcuts (instead of 5 fields):
| Shortcut    | Equivalent     |
|-------------|----------------|
| `@yearly`   | `0 0 1 1 *`    |
| `@monthly`  | `0 0 1 * *`    |
| `@weekly`   | `0 0 * * 0`    |
| `@daily`    | `0 0 * * *`    |
| `@hourly`   | `0 * * * *`    |
| `@reboot`   | at every boot  |

## Step 2
Discard the output entirely:
```
> /dev/null 2>&1
```
- `> /dev/null` — stdout to `/dev/null`
- `2>&1`        — stderr to the same place

The order MATTERS: first redirect stdout, then `2>&1`.

## Step 3
Combining it all:
```bash
echo "@daily $SCRIPT > /dev/null 2>&1"
```
