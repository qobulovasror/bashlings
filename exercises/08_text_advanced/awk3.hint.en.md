# 💡 awk3

## Step 1
**Counter pattern** — with an awk associative array:
```bash
awk '{count[$1]++} END {for (k in count) print k, count[k]}'
```

`$1` — the first field of each line. `count[$1]++` — increments the counter for that field.

## Step 2
The order of an awk associative array is **non-deterministic**. For a definite order, sort via a pipe:
```bash
awk '{count[$1]++} END {for (k in count) print k, count[k]}' log.txt | sort
```

## Step 3
This pattern is one of the most-used idioms in DevOps — log statistics, IP frequency, and so on.
