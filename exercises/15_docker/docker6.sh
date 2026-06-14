#!/usr/bin/env bash
#
# MASHQ: Multi-stage Dockerfile
# DARAJA: ★★★☆☆
# MAVZU: part3/05-docker · FROM ... AS, COPY --from
#
# Multi-stage build — Dockerfile'ni KICHIK final image bilan tayyorlash usuli.
# Go misol uchun quyidagi Dockerfile'ni chiqaring:
#
#     FROM golang:1.22 AS build
#     WORKDIR /src
#     COPY . .
#     RUN go build -o app
#
#     FROM alpine:3.20
#     COPY --from=build /src/app /app
#     CMD ["/app"]
#
# Diqqat: ikki blok orasida BO'SH qator bor.
#
# Maslahat: heredoc — `cat <<EOF ... EOF` butun matnni stdout'ga chiqaradi.
#
# --- English ---
# TASK: A multi-stage Dockerfile
# LEVEL: ★★★☆☆
# TOPIC: part3/05-docker · FROM ... AS, COPY --from
#
# A multi-stage build — a way to produce a SMALL final image from a Dockerfile.
# For a Go example, print the following Dockerfile:
#
#     FROM golang:1.22 AS build
#     WORKDIR /src
#     COPY . .
#     RUN go build -o app
#
#     FROM alpine:3.20
#     COPY --from=build /src/app /app
#     CMD ["/app"]
#
# Attention: there is a BLANK line between the two blocks.
#
# Hint: a heredoc — `cat <<EOF ... EOF` prints the whole text to stdout.

# I AM NOT DONE

# TODO: yuqoridagi Dockerfile'ni chiqaring
echo "TODO"

# === TEST META ===
# @test:stdout-cmd: printf 'FROM golang:1.22 AS build\nWORKDIR /src\nCOPY . .\nRUN go build -o app\n\nFROM alpine:3.20\nCOPY --from=build /src/app /app\nCMD ["/app"]\n'
# @test:exit: 0
