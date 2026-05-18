#!/usr/bin/env bash
# SOLUTION: rob5
lock=/tmp/bashlings-rob5.lock
echo 12345 > "$lock"

if [[ -f "$lock" ]]; then
    echo "busy"
    exit 1
fi
echo "ish bajarilmoqda"

# === TEST META ===
# @test:stdout: busy
# @test:exit: 1
