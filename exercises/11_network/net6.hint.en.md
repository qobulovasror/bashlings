# 💡 net6

## Step 1
URL structure:
```
https://api.github.com/users/octocat
└──┬──┘└──────┬──────┘└─────┬──────┘
protokol     host         yo'l
```

We need **only the host**. So:
1. Strip the leading `http://` or `https://`
2. Strip everything after the first `/` or `:`

## Step 2
With `sed -E` (extended regex), two substitutions joined by `;`:
```bash
echo "https://api.github.com/users" | sed -E 's|^https?://||; s|[:/].*$||'
# api.github.com
```

`|` here is the separator (instead of the default `/`) — because the URL contains `/`.

## Step 3
The same sed command works on multi-line input — separately for each line:
```bash
echo "$urls" | sed -E 's|^https?://||; s|[:/].*$||'
```
