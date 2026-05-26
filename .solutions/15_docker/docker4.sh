#!/usr/bin/env bash
# SOLUTION: docker4 — docker ps natijasidan ID
ps_output='CONTAINER ID   IMAGE          STATUS    NAMES
a1b2c3d4e5f6   nginx:alpine   Up 5min   web
b2c3d4e5f6a7   redis:7        Up 1h     cache
c3d4e5f6a7b8   postgres:16    Up 2d     db'

echo "$ps_output" | tail -n +2 | awk '{print $1}'

# === TEST META ===
# @test:stdout-cmd: printf 'a1b2c3d4e5f6\nb2c3d4e5f6a7\nc3d4e5f6a7b8\n'
# @test:exit: 0
