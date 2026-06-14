# 💡 cron4

## Step 1
day-of-week values:
```
0 = Sunday (or 7)
1 = Monday
2 = Tuesday
3 = Wednesday
4 = Thursday
5 = Friday
6 = Saturday
```

## Step 2
Listing several values: `1,3,5` (no spaces).

Other cron operators:
- `1-5`   → Monday-Friday (a range)
- `1,3,5` → only those days (a list)

## Step 3
Time 08:00 = `0 8`, day-of-week = `1,3,5`:
```
0 8 * * 1,3,5
```
