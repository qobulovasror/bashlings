# 💡 cron4

## 1-bosqich
day-of-week qiymatlari:
```
0 = Yakshanba (yoki 7)
1 = Dushanba
2 = Seshanba
3 = Chorshanba
4 = Payshanba
5 = Juma
6 = Shanba
```

## 2-bosqich
Bir nechta qiymatni ro'yxat qilish: `1,3,5` (bo'sh joysiz).

Boshqa cron operatorlari:
- `1-5`   → dushanba-juma (interval)
- `1,3,5` → faqat shu kunlar (ro'yxat)

## 3-bosqich
Vaqt 08:00 = `0 8`, kun-of-week = `1,3,5`:
```
0 8 * * 1,3,5
```
