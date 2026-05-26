# 💡 cicd6

## 1-bosqich
`if:` — step yoki job darajasida shartli ishlash:
```yaml
- name: Step
  if: <expression>
  run: cmd
```

## 2-bosqich
`github.ref` — push qilingan branch ref'i. Misol:
- `refs/heads/main` — main branch
- `refs/heads/develop` — develop branch
- `refs/pull/42/merge` — PR

## 3-bosqich
Bir tirnoq ichida string taqqoslash:
```yaml
if: github.ref == 'refs/heads/main'
```

Bash'da bunday qatorni chiqarishda — bir tirnoq ichidagi matn `echo "..."`
yoki `printf` bilan bemalol chiqadi:
```bash
echo "  if: github.ref == 'refs/heads/main'"
```
