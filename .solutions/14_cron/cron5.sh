#!/usr/bin/env bash
# SOLUTION: cron5 — to'liq crontab qatori
SCHEDULE="30 2 * * *"
SCRIPT="/usr/local/bin/backup.sh"
LOG="/var/log/backup.log"
echo "$SCHEDULE $SCRIPT >> $LOG 2>&1"

# === TEST META ===
# @test:stdout: 30 2 * * * /usr/local/bin/backup.sh >> /var/log/backup.log 2>&1
# @test:exit: 0
