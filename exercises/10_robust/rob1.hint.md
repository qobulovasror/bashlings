# 💡 rob1

## 1-bosqich
Default Bash xulq-atvori — pipeline'ning **oxirgi** buyrug'i exit code'ini olamiz:
```bash
false | true   # exit 0 (true ning exit'i)
```

Bu xavfli — pipeline ichidagi xatolar yashirinadi.

## 2-bosqich
`set -o pipefail` — endi pipeline'ning eng oxirgi nol bo'lmagan exit kodi qaytariladi:
```bash
set -o pipefail
false | true   # exit 1 (false ning exit'i)
```

## ✅ Yechim
```bash
set -o pipefail
echo "ish"
false | echo "yashirildi"
```
