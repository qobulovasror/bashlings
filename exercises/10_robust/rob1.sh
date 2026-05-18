#!/usr/bin/env bash
#
# MASHQ: pipefail
# DARAJA: ★★★★☆
# MAVZU: part2/05-robust-scripting · set -o pipefail
#
# Quyidagi pipeline'da `false` buyrug'i xato (exit 1) qaytaradi.
# Lekin Bash default — pipeline'ning OXIRGI buyrug'i exit code'ini olamiz.
# Shu sababli xato yashirinadi va skript exit 0 deydi.
#
# `set -o pipefail` qo'shing — pipeline ichidagi har qanday xato chiqsin.
#
# Kutilgan: stdout = "ish\nyashirildi", exit = 1
#
# Maslahat: skript boshida `set -o pipefail`

# I AM NOT DONE

# TODO: set -o pipefail qo'shing


echo "ish"
false | echo "yashirildi"

# === TEST META ===
# @test:stdout-cmd: printf 'ish\nyashirildi\n'
# @test:exit: 1
