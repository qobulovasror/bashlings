# 💡 ssh6

## Step 1
`-L LOCAL:DEST_HOST:DEST_PORT` — colon-separated, the order matters:
```
ssh -L 5433:db.internal:5432 user@bastion
       └──┘ └──────────┘ └──┘
       you   inside bastion remote
```

## Step 2
Variable interpolation inside double quotes:
```bash
echo "ssh -L $LOCAL_PORT:$REMOTE_HOST:$REMOTE_PORT $SSH_USER@$BASTION"
```
