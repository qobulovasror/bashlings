# 💡 sed3

## Step 1
**Capture groups**: `(...)` inside a regex — referenced later via `\1`, `\2`, ...

The `-E` flag enables extended regex (`(...)` is not literal):
```bash
sed -E 's/(.+)-(.+)/\2-\1/'
```

## Step 2
Date format: `YYYY-MM-DD`
- Year: `([0-9]{4})` or `(....)` 
- Month: `([0-9]{2})` or `(..)`
- Day: `([0-9]{2})` or `(..)`

```bash
sed -E 's/([0-9]+)-([0-9]+)-([0-9]+)/\3\/\2\/\1/'
```

## Step 3
You need to escape `/` as `\/` (or change the delimiter to `|`):
```bash
sed -E 's|(....)-(..)-(..)|\3/\2/\1|'
```
