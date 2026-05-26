#!/usr/bin/env bash
# SOLUTION: docker3 — docker run + volume + env + workdir
HOST_DIR="/data"
CTR_DIR="/app"
IMAGE="node:20"
CMD="npm test"
echo "docker run --rm -v $HOST_DIR:$CTR_DIR -w $CTR_DIR -e ENV=dev $IMAGE $CMD"

# === TEST META ===
# @test:stdout: docker run --rm -v /data:/app -w /app -e ENV=dev node:20 npm test
# @test:exit: 0
