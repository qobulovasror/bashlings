#!/usr/bin/env bash
#
# MASHQ: Image nomini qurish
# DARAJA: ★★★☆☆
# MAVZU: part3/05-docker · registry/owner/name:tag
#
# Docker image to'liq nomi formati:
#
#     <registry>/<owner>/<name>:<tag>
#
# O'zgaruvchilardan to'liq nomni chiqaring:
#
#     ghcr.io/myorg/api:v1.2.3
#
# Bu nom `docker pull` yoki `docker run` da ishlatiladi.
#
# Maslahat: oddiy string konkatenatsiya `echo "..."` ichida.
#
# --- English ---
# TASK: Building an image name
# LEVEL: ★★★☆☆
# TOPIC: part3/05-docker · registry/owner/name:tag
#
# The full format of a Docker image name:
#
#     <registry>/<owner>/<name>:<tag>
#
# Print the full name from the variables:
#
#     ghcr.io/myorg/api:v1.2.3
#
# This name is used in `docker pull` or `docker run`.
#
# Hint: simple string concatenation inside `echo "..."`.

# I AM NOT DONE

REGISTRY="ghcr.io"
OWNER="myorg"
NAME="api"
TAG="v1.2.3"

# TODO: to'liq image nomini chiqaring
echo "$NAME"

# === TEST META ===
# @test:stdout: ghcr.io/myorg/api:v1.2.3
# @test:exit: 0
