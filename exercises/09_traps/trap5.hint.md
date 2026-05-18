# 💡 trap5

## 1-bosqich
Idempotent cleanup pattern:
```bash
cleanup() {
    [[ $__done -eq 1 ]] && return   # ikkinchi marta chaqirilsa — return
    __done=1
    # ... haqiqiy cleanup ...
}
```

## 2-bosqich
Birinchi chaqiruvda `__done` = 0 → davom etadi va o'zini "done" deb belgilaydi.
Ikkinchi chaqiruvda — darhol return.

Bu pattern bir nechta trap ulagan paytda muhim:
```bash
trap cleanup EXIT
trap 'cleanup; exit 130' INT   # INT keyin EXIT — ikkalasi ham cleanup chaqiradi
```

## ✅ Yechim
```bash
__done=0
cleanup() {
    [[ $__done -eq 1 ]] && return
    __done=1
    echo "tozalandi"
}
trap cleanup EXIT

echo "main"
cleanup
```
