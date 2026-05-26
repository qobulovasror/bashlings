#!/usr/bin/env bash
#
# MASHQ: URL'lardan hostname'ni ajratish
# DARAJA: ★★★☆☆
# MAVZU: part3/01-network · sed bilan URL parsing
#
# Quyidagi URL ro'yxatidan faqat HOSTNAME'larni alohida qatorlarda chiqaring.
#
# Kutilgan:
#     api.github.com
#     example.org
#     www.google.com
#
# Protokol (http:// yoki https://) va undan keyingi yo'l/port'ni olib tashlang.
#
# Maslahat:
#   - `sed -E 's|^https?://||'`  — protokolni olib tashlash
#   - `sed -E 's|[:/].*$||'`     — port yoki yo'lni olib tashlash
#   - Ikkala almashtirish bitta sed buyrug'ida (';' bilan) yoki ketma-ket pipe'da

# I AM NOT DONE

urls='https://api.github.com/users/octocat
http://example.org:8080/path
https://www.google.com/search?q=bash'

# TODO: hostname'larni chiqaring
echo "$urls"

# === TEST META ===
# @test:stdout-cmd: printf 'api.github.com\nexample.org\nwww.google.com\n'
# @test:exit: 0
