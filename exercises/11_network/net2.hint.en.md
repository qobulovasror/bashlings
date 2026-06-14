# 💡 net2

## Step 1
Taking ONLY the first line from a multi-line string:
```bash
echo "$response" | head -1
# HTTP/2 200
```

## Step 2
Taking the 2nd column from whitespace-separated columns:
```bash
echo "HTTP/2 200" | awk '{print $2}'
# 200
```

`awk` treats whitespace/tabs as the separator by default.

## Step 3
We combine the two via a pipe:
```bash
echo "$response" | head -1 | awk '{print $2}'
```
