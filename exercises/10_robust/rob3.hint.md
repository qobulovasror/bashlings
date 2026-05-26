# 💡 rob3

## 1-bosqich
**Parameter expansion** — `${VAR:-default}`:
- VAR bo'sh/aniqlanmagan → default
- VAR mavjud → o'z qiymati

## 2-bosqich
```bash
level="${LOG_LEVEL:-info}"
```

Foydalanish:
```bash
./script.sh                # Level: info
LOG_LEVEL=debug ./script.sh # Level: debug
```
