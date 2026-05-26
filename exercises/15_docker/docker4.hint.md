# 💡 docker4

## 1-bosqich
Header'ni o'tkazib yuborish — 2-qatordan boshlab o'qish:
```bash
echo "$ps_output" | tail -n +2
```

## 2-bosqich
Birinchi ustun (CONTAINER ID):
```bash
awk '{print $1}'
```

## 3-bosqich
Pipeline:
```bash
echo "$ps_output" | tail -n +2 | awk '{print $1}'
```
