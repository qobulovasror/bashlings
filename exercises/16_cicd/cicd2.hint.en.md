# 💡 cicd2

## Step 1
YAML structure depends on indentation:
```yaml
jobs:                          # 0 sp
  test:                        # 2 sp  — job name
    runs-on: ubuntu-latest     # 4 sp
    steps:                     # 4 sp
      - name: Run tests        # 6 sp  — list item
        run: npm test          # 8 sp  — inside the item
```

## Step 2
`- name:` — the first field of the list item.
`run:` is indented more because it BELONGS to the same list item
(8 sp).

## Step 3
Via heredoc — each line with EXACTLY this indentation:
```bash
cat <<EOF
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - name: Run tests
        run: npm test
EOF
```
