#!/usr/bin/env bash
# SOLUTION: net2 — HTTP status code'ni ajratish
response='HTTP/2 200
content-type: text/html
cache-control: max-age=3600

<html>...</html>'

echo "$response" | head -1 | awk '{print $2}'

# === TEST META ===
# @test:stdout: 200
# @test:exit: 0
