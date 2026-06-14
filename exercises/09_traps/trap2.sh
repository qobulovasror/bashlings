#!/usr/bin/env bash
#
# MASHQ: Cleanup funksiya
# DARAJA: ★★★☆☆
# MAVZU: part2/04-traps-signals · trap + funksiya
#
# `cleanup()` funksiyasini yozing — "tozalandi" deb chiqarsin.
# Uni EXIT'ga trap qiling.
#
# Kutilgan tartib:
#     asosiy ish
#     tozalandi
#
# --- English ---
# TASK: Cleanup function
# LEVEL: ★★★☆☆
# TOPIC: part2/04-traps-signals · trap + function
#
# Write a `cleanup()` function — it should print "tozalandi".
# Trap it on EXIT.
#
# Expected order:
#     asosiy ish
#     tozalandi

# I AM NOT DONE

# TODO: cleanup funksiyasini yozing
cleanup() {
    :
}

# TODO: trap cleanup EXIT


echo "asosiy ish"

# === TEST META ===
# @test:stdout-cmd: printf 'asosiy ish\ntozalandi\n'
# @test:exit: 0
