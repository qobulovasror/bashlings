#!/usr/bin/env bash
#
# MASHQ: Lock fayl pattern
# DARAJA: ★★★★☆
# MAVZU: part2/05-robust-scripting · lock file
#
# Skript boshida — boshqa instance allaqachon ishlamoqda (simulyatsiya):
# `/tmp/bashlings-rob5.lock` faylida PID 12345 saqlanadi.
#
# Sizning vazifa: agar lock fayl mavjud bo'lsa, "busy" deb chiqib `exit 1`.
# Aks holda — "ish bajarilmoqda" deb chiqing.
#
# Kutilgan: stdout = "busy", exit = 1
#
# Maslahat:
#   - `[[ -f "$lock" ]]` — fayl mavjudligini tekshirish
#   - if/then/exit pattern

# I AM NOT DONE

lock=/tmp/bashlings-rob5.lock
echo 12345 > "$lock"   # boshqa instance simulyatsiyasi

# TODO: lock fayl mavjud bo'lsa, "busy" chiqarib exit 1


echo "ish bajarilmoqda"

# === TEST META ===
# @test:stdout: busy
# @test:exit: 1
