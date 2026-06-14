# 💡 cicd7

## Step 1
Filter only the failing lines:
```bash
echo "$log" | grep '^✗'
# ✗ FAIL  Type check
# ✗ FAIL  Build
```

## Step 2
Extracting the step name — line format:
```
✗ FAIL  Type check
└┬┘ └┬─┘ └────┬───┘
 $1   $2    $3+
```

A step name can contain spaces (for example "Type check"), so
`awk '{print $3}'` does not capture the full name. A better solution:
```bash
sed -E 's/^✗ FAIL  //'
```

## Step 3
Alphabetical order:
```bash
sort
```

Pipeline:
```bash
echo "$log" | grep '^✗' | sed -E 's/^✗ FAIL  //' | sort
```
