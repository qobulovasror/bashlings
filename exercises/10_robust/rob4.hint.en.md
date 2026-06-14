# 💡 rob4

## Step 1
`mkdir` default:
- If it doesn't exist — creates it (OK)
- If it exists — **error** (stops with set -e)

## Step 2
`mkdir -p`:
- If it doesn't exist — creates it
- If it exists — **OK** (no error)
- If parent directories on the path don't exist — creates them too

## Step 3
**Idempotent script** — doesn't error even if you run it several times.
