#!/usr/bin/env bash
#
# MASHQ: To'liq crontab qatori — buyruq + log
# DARAJA: ★★★☆☆
# MAVZU: part3/04-cron · buyruq va redirect
#
# Production crontab qatorida — vaqt + buyruq + log redirect bor.
# O'zgaruvchilardan to'liq qatorni chiqaring:
#
#     30 2 * * * /usr/local/bin/backup.sh >> /var/log/backup.log 2>&1
#
# Eslatma:
#   - `>> file`  — append (avvalgi log o'chmaydi)
#   - `2>&1`     — stderr ham log'ga ketsin
#   - cron'da har doim STDERR ni ham faylga yo'naltirish — yaxshi amaliyot

# I AM NOT DONE

SCHEDULE="30 2 * * *"
SCRIPT="/usr/local/bin/backup.sh"
LOG="/var/log/backup.log"

# TODO: to'liq cron qatorini chiqaring
echo "$SCHEDULE $SCRIPT"

# === TEST META ===
# @test:stdout: 30 2 * * * /usr/local/bin/backup.sh >> /var/log/backup.log 2>&1
# @test:exit: 0
