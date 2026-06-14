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
#
# --- English ---
# TASK: Full crontab line — command + log
# LEVEL: ★★★☆☆
# TOPIC: part3/04-cron · command and redirect
#
# A production crontab line has — time + command + log redirect.
# From the variables, print the full line:
#
#     30 2 * * * /usr/local/bin/backup.sh >> /var/log/backup.log 2>&1
#
# Note:
#   - `>> file`  — append (the previous log is not erased)
#   - `2>&1`     — send stderr to the log too
#   - in cron, always redirecting STDERR to a file too — good practice

# I AM NOT DONE

SCHEDULE="30 2 * * *"
SCRIPT="/usr/local/bin/backup.sh"
LOG="/var/log/backup.log"

# TODO: to'liq cron qatorini chiqaring
echo "$SCHEDULE $SCRIPT"

# === TEST META ===
# @test:stdout: 30 2 * * * /usr/local/bin/backup.sh >> /var/log/backup.log 2>&1
# @test:exit: 0
