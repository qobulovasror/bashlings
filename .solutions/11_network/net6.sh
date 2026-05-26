#!/usr/bin/env bash
# SOLUTION: net6 — URL'lardan hostname ajratish
urls='https://api.github.com/users/octocat
http://example.org:8080/path
https://www.google.com/search?q=bash'

echo "$urls" | sed -E 's|^https?://||; s|[:/].*$||'

# === TEST META ===
# @test:stdout-cmd: printf 'api.github.com\nexample.org\nwww.google.com\n'
# @test:exit: 0
