# 💡 docker3

## Step 1
4 flags are used together:
- `--rm`         → remove when finished
- `-v src:dst`   → volume
- `-w path`      → workdir
- `-e KEY=VAL`   → environment

## Step 2
Format:
```
docker run --rm -v <HOST_DIR>:<CTR_DIR> -w <CTR_DIR> -e <K=V> <IMAGE> <CMD>
```

## Step 3
With variables:
```bash
echo "docker run --rm -v $HOST_DIR:$CTR_DIR -w $CTR_DIR -e ENV=dev $IMAGE $CMD"
```
