# 💡 trap3

## 1-bosqich
`$?` — eng oxirgi buyruq exit code. Cleanup'da ushlash uchun darhol o'zgaruvchiga saqlang:
```bash
cleanup() {
    local rc=$?    # birinchi qator MUHIM
    # endi rc qiymati saqlangan
}
```

## 2-bosqich
Agar `local rc=$?` boshqa qatorlardan keyin yozsangiz — undan oldingi buyruqning exit code bo'ladi, sizniki emas.
