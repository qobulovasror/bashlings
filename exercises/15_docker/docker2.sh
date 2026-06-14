#!/usr/bin/env bash
#
# MASHQ: Oddiy Dockerfile
# DARAJA: ★★☆☆☆
# MAVZU: part3/05-docker · Dockerfile asoslari
#
# Quyidagi 4 ta direktivadan iborat Dockerfile'ni stdout'ga chiqaring:
#
#     FROM python:3.11-alpine
#     WORKDIR /app
#     COPY app.py .
#     CMD ["python", "app.py"]
#
# Eslatma: bu Dockerfile ICHKI KONTENT — biz uni faylga yozmaymiz, faqat
# heredoc orqali chiqaramiz.
#
# Maslahat:
#   - `cat <<EOF ... EOF` — heredoc
#   - O'zgaruvchi interpolatsiya kerak emas — literal ko'rsating
#
# --- English ---
# TASK: A simple Dockerfile
# LEVEL: ★★☆☆☆
# TOPIC: part3/05-docker · Dockerfile basics
#
# Print the following Dockerfile (made of 4 directives) to stdout:
#
#     FROM python:3.11-alpine
#     WORKDIR /app
#     COPY app.py .
#     CMD ["python", "app.py"]
#
# Note: this Dockerfile is just INNER CONTENT — we do not write it to a file,
# we only print it via a heredoc.
#
# Hint:
#   - `cat <<EOF ... EOF` — heredoc
#   - No variable interpolation is needed — show it literally

# I AM NOT DONE

# TODO: Dockerfile mazmunini chiqaring
echo "FROM ..."

# === TEST META ===
# @test:stdout-cmd: printf 'FROM python:3.11-alpine\nWORKDIR /app\nCOPY app.py .\nCMD ["python", "app.py"]\n'
# @test:exit: 0
