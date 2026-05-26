#!/usr/bin/env bash
#
# MASHQ: docker ps natijasidan ID'larni ajratish
# DARAJA: ★★★☆☆
# MAVZU: part3/05-docker · ps output parsing
#
# Quyida mock `docker ps` chiqishi berilgan (header + 3 ta konteyner).
# Faqat CONTAINER ID'larni alohida qatorlarda chiqaring.
#
# Kutilgan:
#     a1b2c3d4e5f6
#     b2c3d4e5f6a7
#     c3d4e5f6a7b8
#
# Maslahat:
#   - `tail -n +2` — header'ni o'tkazib yuborish (2-qatordan boshlab)
#   - `awk '{print $1}'` — birinchi ustun (ID)

# I AM NOT DONE

ps_output='CONTAINER ID   IMAGE          STATUS    NAMES
a1b2c3d4e5f6   nginx:alpine   Up 5min   web
b2c3d4e5f6a7   redis:7        Up 1h     cache
c3d4e5f6a7b8   postgres:16    Up 2d     db'

# TODO: ID'larni alohida qatorlarda chiqaring
echo "$ps_output"

# === TEST META ===
# @test:stdout-cmd: printf 'a1b2c3d4e5f6\nb2c3d4e5f6a7\nc3d4e5f6a7b8\n'
# @test:exit: 0
