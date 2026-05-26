#!/usr/bin/env bash
#
# MASHQ: Port skan natijasini sanash
# DARAJA: ★★★☆☆
# MAVZU: part3/01-network · nc -zv output parsing
#
# Quyida `nc -zv` mock chiqishi berilgan. OCHIQ ("succeeded") portlar sonini
# chiqaring.
#
# 5 ta urinish bor — 3 tasi succeeded, 2 tasi failed.
# Kutilgan:
#     3
#
# Maslahat:
#   - `grep -c PATTERN` — moslar SONINI qaytaradi
#   - `succeeded` so'zi faqat OPEN portlar qatorida bor

# I AM NOT DONE

scan_output='Connection to localhost 22 port [tcp/ssh] succeeded!
nc: connectx to localhost port 23 (tcp) failed: Connection refused
Connection to localhost 80 port [tcp/http] succeeded!
nc: connectx to localhost port 81 (tcp) failed: Connection refused
Connection to localhost 443 port [tcp/https] succeeded!'

# TODO: ochiq portlar sonini chiqaring
echo "$scan_output" | wc -l

# === TEST META ===
# @test:stdout: 3
# @test:exit: 0
