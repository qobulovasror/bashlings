#!/usr/bin/env bash
#
# MASHQ: Crontab qatoridan BUYRUQNI ajratish
# DARAJA: ★★★☆☆
# MAVZU: part3/04-cron · cut/awk bilan parsing
#
# Quyidagi crontab qatorida birinchi 5 ta maydon — VAQT, qolgani — BUYRUQ:
#
#     0 5 * * * /opt/sync.sh --quiet
#     └─time─┘   └────buyruq───────┘
#
# Faqat buyruq qismini chiqaring:
#
#     /opt/sync.sh --quiet
#
# Maslahat:
#   - `cut -d' ' -f6-` — 6-maydondan keyingi hammasi
#   - Yoki: `awk '{$1=$2=$3=$4=$5=""; print substr($0,6)}'`

# I AM NOT DONE

line="0 5 * * * /opt/sync.sh --quiet"

# TODO: faqat buyruq qismini chiqaring
echo "$line"

# === TEST META ===
# @test:stdout: /opt/sync.sh --quiet
# @test:exit: 0
