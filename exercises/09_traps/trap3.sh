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
