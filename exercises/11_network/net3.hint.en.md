# 💡 net3

## Step 1
Put the flags into the string in the given order:
```
curl --retry 3 -m 10 -fsSL <URL>
```

## Step 2
Using a Bash variable inside double quotes — it gets interpolated:
```bash
URL="https://example.com"
echo "curl --retry 3 -m 10 -fsSL $URL"
```

## Step 3
Remember: the order doesn't matter for curl, but **the test compares literally**,
so keep the order from the description.
