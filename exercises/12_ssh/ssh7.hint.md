# 💡 ssh7

## 1-bosqich
Faqat muvaffaqiyatli kirishlarni filter qiling:
```bash
echo "$log" | grep "Accepted publickey"
```

## 2-bosqich
Bir qator misoli:
```
May 26 08:14:01 host sshd[1234]: Accepted publickey for deploy from 10.0.0.5 ...
 $1   $2   $3   $4     $5         $6     $7        $8   $9     $10  $11
```

USER nomi 9-ustun:
```bash
awk '{print $9}'
```

## 3-bosqich
Takrorlanmagan va alifbo tartibida:
```bash
sort -u
```

`sort -u` = `sort | uniq` ekvivalenti.
