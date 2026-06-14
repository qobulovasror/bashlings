#!/usr/bin/env bash
#
# MASHQ: @daily shortcut + silent redirect
# DARAJA: ★★★★☆
# MAVZU: part3/04-cron · maxsus stringlar (@)
#
# Cron'da maxsus stringlar bor:
#   @reboot, @yearly, @monthly, @weekly, @daily, @hourly
#
# Quyidagi qatorni chiqaring (har kuni yarim tunda + chiqishni butunlay
# yo'qotish):
#
#     @daily /opt/clean.sh > /dev/null 2>&1
#
# Maslahat:
#   - `> /dev/null` — stdout'ni yo'qot
#   - `2>&1`        — stderr ham xuddi shu joyga
#   - Tartib MUHIM: avval `> /dev/null`, keyin `2>&1`
#
# --- English ---
# TASK: @daily shortcut + silent redirect
# LEVEL: ★★★★☆
# TOPIC: part3/04-cron · special strings (@)
#
# Cron has special strings:
#   @reboot, @yearly, @monthly, @weekly, @daily, @hourly
#
# Print the following line (every day at midnight + discard the output
# entirely):
#
#     @daily /opt/clean.sh > /dev/null 2>&1
#
# Hint:
#   - `> /dev/null` — discard stdout
#   - `2>&1`        — stderr to the same place too
#   - Order MATTERS: first `> /dev/null`, then `2>&1`

# I AM NOT DONE

SCRIPT="/opt/clean.sh"

# TODO: @daily qatorini chiqaring
echo "@daily $SCRIPT"

# === TEST META ===
# @test:stdout: @daily /opt/clean.sh > /dev/null 2>&1
# @test:exit: 0
