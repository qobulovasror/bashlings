# 💡 cron5

## 1-bosqich
To'liq crontab qatori formati:
```
<TIME>  <COMMAND>  >> <LOG>  2>&1
```

## 2-bosqich
O'zgaruvchi interpolatsiyasi `echo "..."` ichida — quoting:
```bash
echo "$SCHEDULE $SCRIPT >> $LOG 2>&1"
```

Eslatma: `>>` va `2>&1` — bu yerda LITERAL matn (cron faylga yoziladi).
Bash redirect operatorlari emas (chunki echo argumenti sifatida ishlatilmoqda).

## 3-bosqich
Eslatma: cron'da har doim stdout VA stderr ikkalasini ham log'ga yo'naltirish
yaxshi amaliyot — aks holda xato xabarlari yo'qoladi.
