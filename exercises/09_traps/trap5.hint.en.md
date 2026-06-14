# 💡 trap5

## Step 1
Idempotent cleanup pattern:
```bash
cleanup() {
    [[ $__done -eq 1 ]] && return   # if called a second time — return
    __done=1
    # ... the real cleanup ...
}
```

## Step 2
On the first call `__done` = 0 → it continues and marks itself "done".
On the second call — it returns immediately.

This pattern matters when several traps are attached:
```bash
trap cleanup EXIT
trap 'cleanup; exit 130' INT   # INT then EXIT — both call cleanup
```
