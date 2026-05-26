# 💡 arr4

## 1-bosqich
**Element + indeks** bilan iteratsiya — `"${!arr[@]}"` (indekslar ro'yxati):
```bash
for i in "${!nums[@]}"; do
    echo "Indeks $i: ${nums[$i]}"
done
```

## 2-bosqich
**Pozitsiya** (1'dan) — indeksga 1 qo'shish:
```bash
echo "Element $((i+1)): ${nums[$i]}"
```
