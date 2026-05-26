# 💡 net4

## 1-bosqich
Komandaning shabloni:
```
curl -X POST -H 'Content-Type: application/json' -d '<JSON>' <URL>
```

## 2-bosqich
Bash'da BIR tirnoq (`'`) ichida o'zgaruvchi **kengayMAYDI**. Ammo `echo`
qo'shtirnoq (`"`) ichida bir tirnoq ko'rinishi mumkin — bash bir tirnoqni
shu joyda literal harf deb qabul qiladi:
```bash
echo "curl -H 'X-Foo: bar'"
# curl -H 'X-Foo: bar'
```

## 3-bosqich
`$BODY` ichida `"` bor, lekin biz uni qo'shtirnoq ichida `echo` qilamiz —
shuning uchun `BODY` qiymati AYNAN body string'iga aylanadi (escape kerak emas,
chunki BODY o'zi bir tirnoqli string).

```bash
echo "curl -X POST -H 'Content-Type: application/json' -d '$BODY' $URL"
```
