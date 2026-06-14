# 💡 pipe2 — Hints

## Step 1
Difference between `>` and `>>`:
- `>`  — if the file exists, the **old contents are erased** and it is written anew
- `>>` — if the file exists, it is **appended to the end** (append)

## Step 2
You need `>>` — because the "first" line must be kept.

## Step 3
After appending, print the file's contents with `cat`.
