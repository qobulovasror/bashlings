# 💡 cicd2

## 1-bosqich
YAML strukturasi chekinishga bog'liq:
```yaml
jobs:                          # 0 sp
  test:                        # 2 sp  — job nomi
    runs-on: ubuntu-latest     # 4 sp
    steps:                     # 4 sp
      - name: Run tests        # 6 sp  — list item
        run: npm test          # 8 sp  — item ichida
```

## 2-bosqich
`- name:` — list elementining birinchi maydoni.
`run:` — xuddi shu list elementiga TEGISHLI ekanligi uchun ko'proq chekinadi
(8 sp).

## 3-bosqich
Heredoc orqali — har qator AYNAN shu chekinish bilan:
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
