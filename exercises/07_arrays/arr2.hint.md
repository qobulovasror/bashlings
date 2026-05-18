# 💡 arr2

## 1-bosqich
Element qo'shish:
```bash
colors+=("yashil")
```

## 2-bosqich
Oxirgi element (Bash 4.2+):
```bash
echo "${colors[-1]}"
```

Eski bash uchun:
```bash
echo "${colors[${#colors[@]}-1]}"
```

## ✅ Yechim
```bash
colors+=("yashil")
echo "${colors[-1]}"
```
