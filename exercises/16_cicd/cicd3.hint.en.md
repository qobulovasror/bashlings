# 💡 cicd3

## Step 1
Using an action — the `uses:` directive:
```yaml
- name: Step name
  uses: <owner>/<action>@<version>
```

## Step 2
`actions/checkout` — the most commonly used official action. It clones
the repo.

`@v4` — a pinned version, not a mutable tag (for security).

## Step 3
Indentation: the second line starts with 2 sp (inside the list item):
```yaml
- name: Checkout code
  uses: actions/checkout@v4
```
