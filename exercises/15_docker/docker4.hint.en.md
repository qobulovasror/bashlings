# 💡 docker4

## Step 1
Skip the header — read starting from line 2:
```bash
echo "$ps_output" | tail -n +2
```

## Step 2
The first column (CONTAINER ID):
```bash
awk '{print $1}'
```

## Step 3
Pipeline:
```bash
echo "$ps_output" | tail -n +2 | awk '{print $1}'
```
