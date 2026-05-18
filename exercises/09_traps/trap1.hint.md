# 💡 trap1

## 1-bosqich
`trap` sintaksisi:
```bash
trap '<kod>' SIGNAL
```

EXIT — Bash pseudo-signali, **har holatda** ishlaydi (normal exit, xato, signal, ...).

## 2-bosqich
```bash
trap 'echo "tozalandi"' EXIT
```

## ✅ Yechim
```bash
trap 'echo "tozalandi"' EXIT
echo "ishlayapti"
```
