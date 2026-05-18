# 💡 func5 — Maslahatlar

## 1-bosqich
Har funksiyada `echo` orqali natija qaytariladi:
```bash
add() {
    echo $(( $1 + $2 ))
}

multiply() {
    echo $(( $1 * $2 ))
}
```

## 2-bosqich
Funksiya natijasini boshqa funksiyaga uzatish — **command substitution** orqali:
```bash
result=$(add 3 4)            # result = 7
final=$(multiply "$result" 2) # final = 14
```

## 3-bosqich
Yoki bitta qatorda (lekin o'qish qiyinroq):
```bash
echo $(multiply "$(add 3 4)" 2)
```

## ✅ Yechim
```bash
add() {
    echo $(( $1 + $2 ))
}

multiply() {
    echo $(( $1 * $2 ))
}

sum=$(add 3 4)
result=$(multiply "$sum" 2)
echo "$result"
```
