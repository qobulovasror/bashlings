#!/usr/bin/env bash
# SOLUTION: net5 — port skan natijasini sanash
scan_output='Connection to localhost 22 port [tcp/ssh] succeeded!
nc: connectx to localhost port 23 (tcp) failed: Connection refused
Connection to localhost 80 port [tcp/http] succeeded!
nc: connectx to localhost port 81 (tcp) failed: Connection refused
Connection to localhost 443 port [tcp/https] succeeded!'

echo "$scan_output" | grep -c "succeeded"

# === TEST META ===
# @test:stdout: 3
# @test:exit: 0
