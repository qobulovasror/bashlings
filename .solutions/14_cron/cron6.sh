#!/usr/bin/env bash
# SOLUTION: cron6 — buyruqni ajratish
line="0 5 * * * /opt/sync.sh --quiet"
echo "$line" | cut -d' ' -f6-

# === TEST META ===
# @test:stdout: /opt/sync.sh --quiet
# @test:exit: 0
