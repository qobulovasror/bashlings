# 💡 arr4

## Step 1
Iterating with **element + index** — `"${!arr[@]}"` (the list of indexes):
```bash
for i in "${!nums[@]}"; do
    echo "Indeks $i: ${nums[$i]}"
done
```

## Step 2
**Position** (from 1) — add 1 to the index:
```bash
echo "Element $((i+1)): ${nums[$i]}"
```
