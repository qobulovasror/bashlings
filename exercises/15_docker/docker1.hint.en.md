# 💡 docker1

## Step 1
Format:
```
docker run -d -p <HOST>:<CTR> <IMAGE>
```

## Step 2
Use the variables inside double quotes:
```bash
echo "docker run -d -p $HOST_PORT:$CTR_PORT $IMAGE"
```
