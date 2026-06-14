#!/usr/bin/env bash
#
# MASHQ: Cron — Du/Chor/Ju 08:00 da
# DARAJA: ★★★☆☆
# MAVZU: part3/04-cron · ro'yxat (a,b,c)
#
# Faqat Dushanba (1), Chorshanba (3), Juma (5) kunlari soat 08:00 da:
#
#     0 8 * * 1,3,5
#
# Maydon: day-of-week — 0=Yakshanba ... 6=Shanba (yoki 7=Yakshanba ham).
#
# Maslahat:
#   - Ro'yxat: `1,3,5` — bo'sh joy YO'Q
#   - hour = 8, minute = 0
#
# --- English ---
# TASK: Cron — Mon/Wed/Fri at 08:00
# LEVEL: ★★★☆☆
# TOPIC: part3/04-cron · list (a,b,c)
#
# Only on Monday (1), Wednesday (3), Friday (5) at 08:00:
#
#     0 8 * * 1,3,5
#
# Field: day-of-week — 0=Sunday ... 6=Saturday (or 7=Sunday as well).
#
# Hint:
#   - List: `1,3,5` — NO spaces
#   - hour = 8, minute = 0

# I AM NOT DONE

# TODO: ifodani chiqaring
echo "TODO"

# === TEST META ===
# @test:stdout: 0 8 * * 1,3,5
# @test:exit: 0
