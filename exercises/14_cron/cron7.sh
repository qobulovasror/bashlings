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

# I AM NOT DONE

SCRIPT="/opt/clean.sh"

# TODO: @daily qatorini chiqaring
echo "@daily $SCRIPT"

# === TEST META ===
# @test:stdout: @daily /opt/clean.sh > /dev/null 2>&1
# @test:exit: 0
