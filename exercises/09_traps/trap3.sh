#!/usr/bin/env bash
#
# MASHQ: Cleanup'da exit code
# DARAJA: ★★★★☆
# MAVZU: part2/04-traps-signals · $? trap ichida
#
# `cleanup()` funksiyasi:
#   - eng birinchi qatorida `$?` ni `rc` o'zgaruvchiga saqlab oling
#   - "Exit: N" formatida chiqarsin (N — rc qiymati)
#
# Asosiy kod: "ish boshlandi" chiqarib `exit 7` qiladi.
#
# Kutilgan:
#     ish boshlandi
#     Exit: 7
# Skript exit code: 7
#
# --- English ---
# TASK: Exit code in cleanup
# LEVEL: ★★★★☆
# TOPIC: part2/04-traps-signals · $? inside trap
#
# The `cleanup()` function:
#   - on its very first line, save `$?` into the `rc` variable
#   - print it in the format "Exit: N" (N — the value of rc)
#
# Main code: prints "ish boshlandi" and does `exit 7`.
#
# Expected:
#     ish boshlandi
#     Exit: 7
# Script exit code: 7

# I AM NOT DONE

cleanup() {
    # TODO: local rc=$?  (eng birinchi qator bo'lishi kerak!)
    # TODO: "Exit: $rc" chiqaring
    :
}

# TODO: trap cleanup EXIT


echo "ish boshlandi"
exit 7

# === TEST META ===
# @test:stdout-cmd: printf 'ish boshlandi\nExit: 7\n'
# @test:exit: 7
