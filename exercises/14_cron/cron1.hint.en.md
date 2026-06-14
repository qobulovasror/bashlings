# 💡 cron1

## Step 1
Cron — 5 fields, in each `*` means "at every value":
```
* * * * *
│ │ │ │ │
│ │ │ │ └── day-of-week (0-6)
│ │ │ └──── month (1-12)
│ │ └────── day-of-month (1-31)
│ └──────── hour (0-23)
└────────── minute (0-59)
```

## Step 2
Every minute = all fields `*` = let everything match.
