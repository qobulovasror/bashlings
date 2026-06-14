# 💡 nav5 — Hints

## Step 1
The **wildcard** `*` — means any character(s) (including **none** at all). So `*.txt` — is any file ending with `.txt`.

## Step 2
`ls` accepts a wildcard as an argument. The shell first expands it into a **list of files**, then passes it to `ls`.

## Step 3
In our case `*.txt` expands to `a.txt` and `b.txt` — `c.log` does not match.
