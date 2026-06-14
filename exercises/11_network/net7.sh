#!/usr/bin/env bash
#
# MASHQ: Health-check loop (mock curl bilan)
# DARAJA: ★★★★☆
# MAVZU: part3/01-network · URL monitor pattern
#
# `curl` ni offline'da test qila olmaymiz, shuning uchun uni FUNKSIYA bilan
# o'rab oldik — har URL uchun fixed status qaytaradi.
#
# Sizning vazifangiz: URLS massivi ustidan aylanib, har bir URL uchun
# "URL=STATUS" formatida chiqarish.
#
# Kutilgan:
#     https://github.com=200
#     https://example.com/notfound=404
#     https://bad.invalid=500
#
# Maslahat:
#   - Massiv ustidan loop: `for url in "${URLS[@]}"; do ...; done`
#   - Funksiya chiqishini olish: `status=$(curl "$url")`
#   - Format: `echo "$url=$status"`
#
# --- English ---
# TASK: Health-check loop (with mock curl)
# LEVEL: ★★★★☆
# TOPIC: part3/01-network · URL monitor pattern
#
# We cannot test `curl` offline, so we wrapped it in a FUNCTION —
# it returns a fixed status for each URL.
#
# Your task: loop over the URLS array and, for each URL, print it
# in the "URL=STATUS" format.
#
# Expected:
#     https://github.com=200
#     https://example.com/notfound=404
#     https://bad.invalid=500
#
# Hint:
#   - Loop over the array: `for url in "${URLS[@]}"; do ...; done`
#   - Capture the function output: `status=$(curl "$url")`
#   - Format: `echo "$url=$status"`

# I AM NOT DONE

# Mock curl — offline test uchun fixed javoblar
curl() {
    case "$1" in
        *github*)  echo 200 ;;
        *example*) echo 404 ;;
        *bad*)     echo 500 ;;
        *)         echo 000 ;;
    esac
}

URLS=(https://github.com https://example.com/notfound https://bad.invalid)

# TODO: har URL uchun "URL=STATUS" chiqaring

# === TEST META ===
# @test:stdout-cmd: printf 'https://github.com=200\nhttps://example.com/notfound=404\nhttps://bad.invalid=500\n'
# @test:exit: 0
