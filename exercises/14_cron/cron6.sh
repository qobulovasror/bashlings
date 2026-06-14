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
#
# --- English ---
# TASK: Extract the COMMAND from a crontab line
# LEVEL: ★★★☆☆
# TOPIC: part3/04-cron · parsing with cut/awk
#
# In the crontab line below, the first 5 fields are the TIME, the rest is
# the COMMAND:
#
#     0 5 * * * /opt/sync.sh --quiet
#     └─time─┘   └────command──────┘
#
# Print only the command part:
#
#     /opt/sync.sh --quiet
#
# Hint:
#   - `cut -d' ' -f6-` — everything from the 6th field onward
#   - Or: `awk '{$1=$2=$3=$4=$5=""; print substr($0,6)}'`

# I AM NOT DONE

line="0 5 * * * /opt/sync.sh --quiet"

# TODO: faqat buyruq qismini chiqaring
echo "$line"

# === TEST META ===
# @test:stdout: /opt/sync.sh --quiet
# @test:exit: 0
