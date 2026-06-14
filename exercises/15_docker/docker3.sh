#!/usr/bin/env bash
#
# MASHQ: docker run — volume + env + workdir
# DARAJA: ★★☆☆☆
# MAVZU: part3/05-docker · -v, -w, -e, --rm
#
# Dev workflow uchun klassik docker run kombinatsiyasi:
#
#     docker run --rm -v /data:/app -w /app -e ENV=dev node:20 npm test
#
# Flag'lar:
#   --rm   → konteyner tugagach o'chir (tmp test'lar uchun)
#   -v     → volume mount (lokal:konteyner)
#   -w     → ish katalogi konteyner ichida
#   -e K=V → environment variable
#
# --- English ---
# TASK: docker run — volume + env + workdir
# LEVEL: ★★☆☆☆
# TOPIC: part3/05-docker · -v, -w, -e, --rm
#
# A classic docker run combination for a dev workflow:
#
#     docker run --rm -v /data:/app -w /app -e ENV=dev node:20 npm test
#
# Flags:
#   --rm   → remove the container once it exits (for temporary tests)
#   -v     → volume mount (local:container)
#   -w     → working directory inside the container
#   -e K=V → environment variable

# I AM NOT DONE

HOST_DIR="/data"
CTR_DIR="/app"
IMAGE="node:20"
CMD="npm test"

# TODO: yuqoridagi komandani chiqaring
echo "docker run"

# === TEST META ===
# @test:stdout: docker run --rm -v /data:/app -w /app -e ENV=dev node:20 npm test
# @test:exit: 0
