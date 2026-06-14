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
#
# --- English ---
# TASK: First trap — EXIT
# LEVEL: ★★★☆☆
# TOPIC: part2/04-traps-signals · trap EXIT
#
# Set up an EXIT trap — it should print "tozalandi" when the script finishes.
# The main code prints "ishlayapti".
#
# Expected order:
#     ishlayapti
#     tozalandi    ← printed by the EXIT trap
#
# Hint:
#   - `trap 'echo "tozalandi"' EXIT`
#   - EXIT — a bash pseudo-signal, called in every case

# I AM NOT DONE

# TODO: trap o'rnating


echo "ishlayapti"

# === TEST META ===
# @test:stdout-cmd: printf 'ishlayapti\ntozalandi\n'
# @test:exit: 0
