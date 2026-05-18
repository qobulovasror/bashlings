#!/usr/bin/env bash
#
# MASHQ: Birinchi trap — EXIT
# DARAJA: ★★★☆☆
# MAVZU: part2/04-traps-signals · trap EXIT
#
# EXIT trap o'rnating — skript tugaganda "tozalandi" deb chiqarsin.
# Asosiy kod "ishlayapti" deb chiqaradi.
#
# Kutilgan tartib:
#     ishlayapti
#     tozalandi    ← EXIT trap'da chiqadi
#
# Maslahat:
#   - `trap 'echo "tozalandi"' EXIT`
#   - EXIT — bash pseudo-signali, har holatda chaqiriladi

# I AM NOT DONE

# TODO: trap o'rnating


echo "ishlayapti"

# === TEST META ===
# @test:stdout-cmd: printf 'ishlayapti\ntozalandi\n'
# @test:exit: 0
