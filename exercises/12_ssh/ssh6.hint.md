# 💡 ssh6

## 1-bosqich
`-L LOCAL:DEST_HOST:DEST_PORT` — kolon bilan ajratilgan, tartib muhim:
```
ssh -L 5433:db.internal:5432 user@bastion
       └──┘ └──────────┘ └──┘
       siz   bastion ich.  remote
```

## 2-bosqich
O'zgaruvchi interpolatsiyasi qo'shtirnoq ichida:
```bash
echo "ssh -L $LOCAL_PORT:$REMOTE_HOST:$REMOTE_PORT $SSH_USER@$BASTION"
```
