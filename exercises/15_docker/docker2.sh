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

# I AM NOT DONE

# TODO: Dockerfile mazmunini chiqaring
echo "FROM ..."

# === TEST META ===
# @test:stdout-cmd: printf 'FROM python:3.11-alpine\nWORKDIR /app\nCOPY app.py .\nCMD ["python", "app.py"]\n'
# @test:exit: 0
