# 💡 docker3

## 1-bosqich
4 ta flag birga ishlatiladi:
- `--rm`         → tugagach o'chir
- `-v src:dst`   → volume
- `-w path`      → workdir
- `-e KEY=VAL`   → environment

## 2-bosqich
Format:
```
docker run --rm -v <HOST_DIR>:<CTR_DIR> -w <CTR_DIR> -e <K=V> <IMAGE> <CMD>
```

## 3-bosqich
O'zgaruvchilar bilan:
```bash
echo "docker run --rm -v $HOST_DIR:$CTR_DIR -w $CTR_DIR -e ENV=dev $IMAGE $CMD"
```
