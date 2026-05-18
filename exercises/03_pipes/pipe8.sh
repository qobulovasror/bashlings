#!/usr/bin/env bash
#
# MASHQ: Here-string bilan input
# DARAJA: ★★★☆☆
# MAVZU: part1/03-pipes-redirection · <<<
#
# `bc` — bu komandalar qatori orqali ishlovchi kalkulyator.
# Unga "5+7" ifodasini HERE-STRING (`<<<`) orqali uzating.
# Natija (12) chiqishi kerak.
#
# Maslahat:
#   - `<<<` — bitta qator matnni stdin sifatida buyruqqa beradi
#   - Misol: `cat <<< "salom"` → "salom" chiqaradi
#
# Muqobil (lekin bu mashqda biz <<< ni o'rganamiz):
#   echo "5+7" | bc

# I AM NOT DONE

# TODO: bc ga "5+7" ni <<< orqali uzating

# === TEST META ===
# @test:stdout: 12
# @test:exit: 0
