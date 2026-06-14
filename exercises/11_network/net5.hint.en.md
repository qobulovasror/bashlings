# 💡 net5

## Step 1
Finding lines that match a PATTERN in a multi-line string:
```bash
echo "$scan_output" | grep "succeeded"
```

## Step 2
To get only the COUNT, the `-c` flag — prints the **number**, not the lines:
```bash
echo "$scan_output" | grep -c "succeeded"
```

## Step 3
Note: `wc -l` counts the number of lines in the whole string (5) — but we
only need the OPEN ports, so `grep -c` is the right choice.
