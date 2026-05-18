#!/usr/bin/env bash
#
# MASHQ: Environment variable + default
# DARAJA: ★★★☆☆
# MAVZU: part2/05-robust-scripting · ${VAR:-default}
#
# Skript LOG_LEVEL environment variable bilan ishlaydi.
# Agar berilmagan bo'lsa — "info" deb hisoblang.
#
# Test runner LOG_LEVEL'ni o'rnatmaydi, shuning uchun default ishlashi kerak.
#
# Kutilgan:
#     Level: info
#
# Maslahat: `${VAR:-default}` — agar VAR bo'sh bo'lsa, default qaytaradi.

# I AM NOT DONE

# TODO: ${LOG_LEVEL:-info} bilan to'ldiring
level="$LOG_LEVEL"

echo "Level: $level"

# === TEST META ===
# @test:stdout: Level: info
# @test:exit: 0
