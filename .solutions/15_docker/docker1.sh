#!/usr/bin/env bash
# SOLUTION: docker1 — docker run -d -p
IMAGE="nginx:alpine"
HOST_PORT=8080
CTR_PORT=80
echo "docker run -d -p $HOST_PORT:$CTR_PORT $IMAGE"

# === TEST META ===
# @test:stdout: docker run -d -p 8080:80 nginx:alpine
# @test:exit: 0
