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

# I AM NOT DONE

# TODO: yuqoridagi YAML'ni chiqaring
echo "name: ..."

# === TEST META ===
# @test:stdout-cmd: printf 'name: CI\non: push\n'
# @test:exit: 0
