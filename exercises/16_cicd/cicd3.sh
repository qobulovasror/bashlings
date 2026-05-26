#!/usr/bin/env bash
#
# MASHQ: actions/checkout — kodni klon qilish
# DARAJA: ★★☆☆☆
# MAVZU: part3/06-cicd · uses: action
#
# Har GitHub Actions workflow'i kodni klon qilish bilan boshlanadi.
# Quyidagi step bloki MUHIM — uses + name:
#
#     - name: Checkout code
#       uses: actions/checkout@v4
#
# Eslatma: `uses:` — marketplace'dan action ishlatish.
# `@v4` — pinned versiya (mutable tag emas, releasega bog'langan).
#
# Maslahat: 2 ta qator, ikkinchisi 2 ta bo'sh joy bilan chekinadi (parent
# list element ostida).

# I AM NOT DONE

# TODO: yuqoridagi step blokini chiqaring
echo "- name: ..."

# === TEST META ===
# @test:stdout-cmd: printf -- '- name: Checkout code\n  uses: actions/checkout@v4\n'
# @test:exit: 0
