# 💡 cicd4

## Step 1
Matrix — running one job in parallel with different parameters:
```yaml
strategy:
  matrix:
    os: [ubuntu-latest, macos-latest]
```

## Step 2
`${{ matrix.os }}` — expands at GitHub Actions runtime (not in bash).
Our script must print this text EXACTLY as it is.

To keep it literal in Bash:
- **single quotes**: `'${{ matrix.os }}'`  → bash changes nothing
- or a heredoc (by default interpolation happens, but bash does not
  understand the `${{ }}` form and leaves it literal)

## Step 3
Try it:
```bash
echo '${{ matrix.os }}'        # literal
```
