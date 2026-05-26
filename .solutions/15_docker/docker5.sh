#!/usr/bin/env bash
# SOLUTION: docker5 — image to'liq nomi
REGISTRY="ghcr.io"
OWNER="myorg"
NAME="api"
TAG="v1.2.3"
echo "$REGISTRY/$OWNER/$NAME:$TAG"

# === TEST META ===
# @test:stdout: ghcr.io/myorg/api:v1.2.3
# @test:exit: 0
