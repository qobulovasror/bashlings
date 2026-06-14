# 💡 awk2

## Step 1
**Accumulate pattern** — a variable that grows on each line:
```bash
awk '{sum += $2}' file
```

In awk you don't need to declare a variable — on first use it is treated as 0.

## Step 2
To print the final result — the `END` block:
```bash
awk '{sum += $2} END {print sum}' scores.txt
```

`END { }` runs after all lines have been read.

## Step 3
To compute the average:
```bash
awk '{sum += $2; n++} END {print sum/n}'
```
