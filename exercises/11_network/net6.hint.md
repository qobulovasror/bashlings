# 💡 net6

## 1-bosqich
URL strukturasi:
```
https://api.github.com/users/octocat
└──┬──┘└──────┬──────┘└─────┬──────┘
protokol     host         yo'l
```

Bizga **faqat host** kerak. Demak:
1. Boshidagi `http://` yoki `https://` ni olib tashlash
2. Birinchi `/` yoki `:` dan keyingi hammasini olib tashlash

## 2-bosqich
`sed -E` (extended regex) bilan ikki o'zgartirish, `;` bilan ulanadi:
```bash
echo "https://api.github.com/users" | sed -E 's|^https?://||; s|[:/].*$||'
# api.github.com
```

`|` bu yerda separator (default `/` o'rniga) — URL ichida `/` bo'lgani uchun.

## 3-bosqich
Multi-line input'ga bir xil sed buyrug'i ishlaydi — har qatorga alohida:
```bash
echo "$urls" | sed -E 's|^https?://||; s|[:/].*$||'
```
