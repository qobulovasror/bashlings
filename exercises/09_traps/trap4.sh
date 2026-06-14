#!/usr/bin/env bash
#
# MASHQ: Tmp fayl cleanup pattern
# DARAJA: ★★★★☆
# MAVZU: part2/04-traps-signals · production cleanup
#
# Vaqtinchalik fayl yaratiladi va EXIT'da o'chiriladi.
# `cleanup()` funksiya:
#   - $tmpfile ni `rm -f` bilan o'chirsin
#   - "tmp o'chirildi" deb chiqarsin
#
# Asosiy kod "asosiy ish" deb chiqaradi.
#
# Kutilgan:
#     asosiy ish
#     tmp o'chirildi
#
# --- English ---
# TASK: Temp file cleanup pattern
# LEVEL: ★★★★☆
# TOPIC: part2/04-traps-signals · production cleanup
#
# A temporary file is created and removed on EXIT.
# The `cleanup()` function:
#   - should remove $tmpfile with `rm -f`
#   - should print "tmp o'chirildi"
#
# The main code prints "asosiy ish".
#
# Expected:
#     asosiy ish
#     tmp o'chirildi

# I AM NOT DONE

tmpfile=$(mktemp)
echo "data" > "$tmpfile"

cleanup() {
    # TODO: rm -f "$tmpfile"
    # TODO: echo "tmp o'chirildi"
    :
}

# TODO: trap cleanup EXIT


echo "asosiy ish"

# === TEST META ===
# @test:stdout-cmd: printf "asosiy ish\ntmp o'chirildi\n"
# @test:exit: 0
