# 💡 cicd7

## 1-bosqich
Faqat fail qatorlarini filter qiling:
```bash
echo "$log" | grep '^✗'
# ✗ FAIL  Type check
# ✗ FAIL  Build
```

## 2-bosqich
Step nomini ajratish — qator formati:
```
✗ FAIL  Type check
└┬┘ └┬─┘ └────┬───┘
 $1   $2    $3+
```

Step nomida bo'sh joy bo'lishi mumkin (masalan "Type check"), shuning uchun
`awk '{print $3}'` to'liq nomni olmaydi. Yaxshi yechim:
```bash
sed -E 's/^✗ FAIL  //'
```

## 3-bosqich
Alifbo tartibi:
```bash
sort
```

Pipeline:
```bash
echo "$log" | grep '^✗' | sed -E 's/^✗ FAIL  //' | sort
```
