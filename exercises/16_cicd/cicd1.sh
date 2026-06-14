#!/usr/bin/env bash
#
# MASHQ: Minimal workflow header
# DARAJA: ★☆☆☆☆
# MAVZU: part3/06-cicd · name + on:
#
# GitHub Actions workflow har doim `name:` va `on:` bilan boshlanadi.
# Quyidagi YAML'ni stdout'ga chiqaring:
#
#     name: CI
#     on: push
#
# Eslatma: bu shu workflow har push'da ishga tushadi degani.
#
# Maslahat: heredoc `cat <<EOF ... EOF`.
#
# --- English ---
# TASK: A minimal workflow header
# LEVEL: ★☆☆☆☆
# TOPIC: part3/06-cicd · name + on:
#
# A GitHub Actions workflow always starts with `name:` and `on:`.
# Print the following YAML to stdout:
#
#     name: CI
#     on: push
#
# Note: this means the workflow runs on every push.
#
# Hint: a heredoc `cat <<EOF ... EOF`.

# I AM NOT DONE

# TODO: yuqoridagi YAML'ni chiqaring
echo "name: ..."

# === TEST META ===
# @test:stdout-cmd: printf 'name: CI\non: push\n'
# @test:exit: 0
