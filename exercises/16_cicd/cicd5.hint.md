# 💡 cicd5

## 1-bosqich
`env:` step ichida — faqat shu step uchun environment variable beradi:
```yaml
- name: Step
  run: cmd
  env:
    KEY: value
```

## 2-bosqich
Secret'lar `${{ secrets.NAME }}` ifoda orqali olinadi:
```yaml
NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}
```

## 3-bosqich
Bash'da `${{ }}` literal saqlash uchun BIR tirnoqli heredoc ishlatamiz:
```bash
cat <<'EOF'
- name: Publish to npm
  run: npm publish
  env:
    NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}
EOF
```
