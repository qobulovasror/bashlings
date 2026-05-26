#!/usr/bin/env bash
#
# MASHQ: Retry + timeout bilan curl
# DARAJA: ★★☆☆☆
# MAVZU: part3/01-network · --retry, -m
#
# Tarmoq beqaror bo'lishi mumkin. Skriptlar uchun curl'ga shu flag'larni qo'shamiz:
#   --retry 3   → 3 marta qayta urin
#   -m 10       → har urinish uchun 10 soniya max
#   -fsSL       → standart skript flaglari
#
# Quyidagi URL uchun aynan shu kombinatsiyani chiqaring (flag tartibi muhim):
#     curl --retry 3 -m 10 -fsSL https://api.example.com/data
#
# Maslahat:
#   - Eslatma: -m (--max-time) butun so'rov uchun, --connect-timeout esa
#     faqat ulanish bosqichi uchun. Skriptlar uchun -m ko'p ishlatiladi.

# I AM NOT DONE

URL="https://api.example.com/data"

# TODO: yuqoridagi kombinatsiyani chiqaring
echo "curl $URL"

# === TEST META ===
# @test:stdout: curl --retry 3 -m 10 -fsSL https://api.example.com/data
# @test:exit: 0
