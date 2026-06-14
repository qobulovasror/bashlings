#!/usr/bin/env bash
#
# MASHQ: Boolean funksiya — return
# DARAJA: ★★★★☆
# MAVZU: part2/01-functions · return as boolean
#
# `is_even` funksiyasini yozing — bitta sonni qabul qiladi.
# Agar son JUFT bo'lsa — `return 0` (muvaffaqiyat)
# Agar TOQ bo'lsa — `return 1`
#
# Asosiy kod allaqachon yozilgan — har sonni `if is_even` bilan tekshiradi.
#
# Kutilgan natija:
#     4 juft
#     7 toq
#
# Maslahat:
#   - `$(( $1 % 2 ))` — qoldiq (0 yoki 1)
#   - `return $(( ... ))` — qoldiqni return qilish
#   - 0 = juft = return 0 = if true
#   - 1 = toq = return 1 = if false
#
# --- English ---
# TASK: Boolean function — return
# LEVEL: ★★★★☆
# TOPIC: part2/01-functions · return as boolean
#
# Write an `is_even` function — it accepts one number.
# If the number is EVEN — `return 0` (success)
# If it is ODD — `return 1`
#
# The main code is already written — it checks each number with `if is_even`.
#
# Expected result:
#     4 juft
#     7 toq
#
# Hint:
#   - `$(( $1 % 2 ))` — the remainder (0 or 1)
#   - `return $(( ... ))` — return the remainder
#   - 0 = even = return 0 = if true
#   - 1 = odd = return 1 = if false

# I AM NOT DONE

is_even() {
    # TODO: $1 % 2 qoldig'ini return qiling
    return 1
}

# Asosiy kod
for n in 4 7; do
    if is_even "$n"; then
        echo "$n juft"
    else
        echo "$n toq"
    fi
done

# === TEST META ===
# @test:stdout-cmd: printf '4 juft\n7 toq\n'
# @test:exit: 0
