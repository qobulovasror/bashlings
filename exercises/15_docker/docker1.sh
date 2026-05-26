#!/usr/bin/env bash
#
# MASHQ: docker run komandasi
# DARAJA: ★☆☆☆☆
# MAVZU: part3/05-docker · docker run -d -p
#
# O'zgaruvchilardan to'liq `docker run` komandasini chiqaring:
#
#     docker run -d -p 8080:80 nginx:alpine
#
# Flag'lar:
#   -d           → detached (background)
#   -p HOST:CTR  → port mapping
#
# Maslahat:
#   - Format: `docker run -d -p $HOST_PORT:$CTR_PORT $IMAGE`

# I AM NOT DONE

IMAGE="nginx:alpine"
HOST_PORT=8080
CTR_PORT=80

# TODO: docker run komandasini chiqaring
echo "docker run $IMAGE"

# === TEST META ===
# @test:stdout: docker run -d -p 8080:80 nginx:alpine
# @test:exit: 0
