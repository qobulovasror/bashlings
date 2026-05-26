# 💡 cicd3

## 1-bosqich
Action ishlatish — `uses:` direktivasi:
```yaml
- name: Step nom
  uses: <owner>/<action>@<version>
```

## 2-bosqich
`actions/checkout` — eng ko'p ishlatiladigan official action. Repo'ni
klon qiladi.

`@v4` — pinned versiya, mutable tag emas (xavfsizlik uchun).

## 3-bosqich
Chekinish: ikkinchi qator 2 sp bilan boshlanadi (list element ichida):
```yaml
- name: Checkout code
  uses: actions/checkout@v4
```
