# 💡 arr2

## Step 1
Adding an element:
```bash
colors+=("yashil")
```

## Step 2
The last element (Bash 4.2+):
```bash
echo "${colors[-1]}"
```

For older bash:
```bash
echo "${colors[${#colors[@]}-1]}"
```
