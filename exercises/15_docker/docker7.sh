#!/usr/bin/env bash
#
# MASHQ: docker-compose.yml minimal
# DARAJA: ★★★★☆
# MAVZU: part3/05-docker · compose services
#
# Quyidagi YAML'ni stdout'ga chiqaring (AYNAN shu chekinish bilan,
# 2 ta bo'sh joy har bosqich):
#
#     services:
#       web:
#         image: nginx:alpine
#         ports:
#           - "8080:80"
#         environment:
#           ENV: production
#
# YAML chekinishi qat'iy: 2-spaces (TAB ishlatib BO'LMAYDI).
#
# Maslahat:
#   - Heredoc orqali: `cat <<EOF ... EOF`
#   - Har bosqich 2 ta bo'sh joy bilan
#   - Quotes literal qoladi (port "8080:80")
#
# --- English ---
# TASK: A minimal docker-compose.yml
# LEVEL: ★★★★☆
# TOPIC: part3/05-docker · compose services
#
# Print the following YAML to stdout (with EXACTLY this indentation,
# 2 spaces per level):
#
#     services:
#       web:
#         image: nginx:alpine
#         ports:
#           - "8080:80"
#         environment:
#           ENV: production
#
# YAML indentation is strict: 2 spaces (TABs are NOT allowed).
#
# Hint:
#   - Via a heredoc: `cat <<EOF ... EOF`
#   - 2 spaces per level
#   - Quotes stay literal (port "8080:80")

# I AM NOT DONE

# TODO: yuqoridagi YAML'ni chiqaring
echo "services:"

# === TEST META ===
# @test:stdout-cmd: printf 'services:\n  web:\n    image: nginx:alpine\n    ports:\n      - "8080:80"\n    environment:\n      ENV: production\n'
# @test:exit: 0
