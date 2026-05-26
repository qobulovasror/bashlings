#!/usr/bin/env bash
# SOLUTION: net7 — health-check loop (mock curl bilan)
curl() {
    case "$1" in
        *github*)  echo 200 ;;
        *example*) echo 404 ;;
        *bad*)     echo 500 ;;
        *)         echo 000 ;;
    esac
}

URLS=(https://github.com https://example.com/notfound https://bad.invalid)

for url in "${URLS[@]}"; do
    status=$(curl "$url")
    echo "$url=$status"
done

# === TEST META ===
# @test:stdout-cmd: printf 'https://github.com=200\nhttps://example.com/notfound=404\nhttps://bad.invalid=500\n'
# @test:exit: 0
