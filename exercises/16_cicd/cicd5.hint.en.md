# 💡 cicd5

## Step 1
`env:` inside a step — provides an environment variable only for that step:
```yaml
- name: Step
  run: cmd
  env:
    KEY: value
```

## Step 2
Secrets are accessed via the `${{ secrets.NAME }}` expression:
```yaml
NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}
```

## Step 3
In Bash, to keep `${{ }}` literal we use a single-quoted heredoc:
```bash
cat <<'EOF'
- name: Publish to npm
  run: npm publish
  env:
    NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}
EOF
```
