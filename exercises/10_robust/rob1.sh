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
#
# --- English ---
# TASK: pipefail
# LEVEL: ★★★★☆
# TOPIC: part2/05-robust-scripting · set -o pipefail
#
# In the pipeline below the `false` command fails (exit 1).
# But by default Bash takes the exit code of the LAST command in the pipeline.
# Because of that the error is hidden and the script returns exit 0.
#
# Add `set -o pipefail` — so any error inside the pipeline surfaces.
#
# Expected: stdout = "ish\nyashirildi", exit = 1
#
# Hint: at the top of the script, `set -o pipefail`

# I AM NOT DONE

# TODO: set -o pipefail qo'shing


echo "ish"
false | echo "yashirildi"

# === TEST META ===
# @test:stdout-cmd: printf 'ish\nyashirildi\n'
# @test:exit: 1
