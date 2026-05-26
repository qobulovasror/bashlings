#!/usr/bin/env bash
# SOLUTION: net4 — JSON POST so'rovi
URL="https://httpbin.org/post"
BODY='{"city":"Toshkent"}'
echo "curl -X POST -H 'Content-Type: application/json' -d '$BODY' $URL"

# === TEST META ===
# @test:stdout: curl -X POST -H 'Content-Type: application/json' -d '{"city":"Toshkent"}' https://httpbin.org/post
# @test:exit: 0
