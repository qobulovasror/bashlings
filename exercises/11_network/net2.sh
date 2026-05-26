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
