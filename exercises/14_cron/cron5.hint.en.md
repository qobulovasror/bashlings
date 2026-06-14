# 💡 cron5

## Step 1
The full crontab line format:
```
<TIME>  <COMMAND>  >> <LOG>  2>&1
```

## Step 2
Variable interpolation inside `echo "..."` — quoting:
```bash
echo "$SCHEDULE $SCRIPT >> $LOG 2>&1"
```

Note: `>>` and `2>&1` — here these are LITERAL text (written into the cron file).
They are not bash redirect operators (because they are used as arguments to echo).

## Step 3
Note: in cron it is always good practice to redirect BOTH stdout AND stderr to
the log — otherwise error messages get lost.
