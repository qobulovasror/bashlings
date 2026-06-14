# 💡 cicd6

## Step 1
`if:` — conditional execution at the step or job level:
```yaml
- name: Step
  if: <expression>
  run: cmd
```

## Step 2
`github.ref` — the ref of the pushed branch. Examples:
- `refs/heads/main` — main branch
- `refs/heads/develop` — develop branch
- `refs/pull/42/merge` — PR

## Step 3
String comparison inside single quotes:
```yaml
if: github.ref == 'refs/heads/main'
```

In Bash, when printing such a line, the text inside single quotes prints
fine with `echo "..."` or `printf`:
```bash
echo "  if: github.ref == 'refs/heads/main'"
```
