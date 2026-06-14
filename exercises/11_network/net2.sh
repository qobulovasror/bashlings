#!/usr/bin/env bash
#
# MASHQ: HTTP status code'ni ajratib olish
# DARAJA: ★★☆☆☆
# MAVZU: part3/01-network · response parsing
#
# Quyidagi mock HTTP javobining BIRINCHI qatorida HTTP/2 va status code bor:
#     HTTP/2 200
#     content-type: text/html
#     cache-control: max-age=3600
#
# Faqat status code raqamini chiqaring:
#     200
#
# Maslahat:
#   - `head -1` — faqat birinchi qator
#   - `awk '{print $2}'` — bo'sh joy bilan ajratilgan 2-ustun
#
# --- English ---
# TASK: Extracting the HTTP status code
# LEVEL: ★★☆☆☆
# TOPIC: part3/01-network · response parsing
#
# The FIRST line of the mock HTTP response below has HTTP/2 and the status code:
#     HTTP/2 200
#     content-type: text/html
#     cache-control: max-age=3600
#
# Print only the status code number:
#     200
#
# Hint:
#   - `head -1` — only the first line
#   - `awk '{print $2}'` — the 2nd column separated by whitespace

# I AM NOT DONE

response='HTTP/2 200
content-type: text/html
cache-control: max-age=3600

<html>...</html>'

# TODO: response'ning birinchi qatoridan status code'ni chiqaring
echo "$response"

# === TEST META ===
# @test:stdout: 200
# @test:exit: 0
