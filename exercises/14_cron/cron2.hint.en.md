# 💡 cron2

## Step 1
Time 03:30 — minute = 30, hour = 3:
```
30 3 * * *
│  │ └──┴──┴── day/month/dow = *
│  └────────── hour
└───────────── minute
```

## Step 2
The remaining 3 fields are `*` — because it is "every day" (it doesn't matter
which month or which day of the week it is).
