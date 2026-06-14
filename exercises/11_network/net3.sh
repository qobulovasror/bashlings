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
#
# --- English ---
# TASK: curl with retry + timeout
# LEVEL: ★★☆☆☆
# TOPIC: part3/01-network · --retry, -m
#
# The network can be unstable. For scripts we add these flags to curl:
#   --retry 3   → retry 3 times
#   -m 10       → max 10 seconds per attempt
#   -fsSL       → the standard script flags
#
# Print exactly this combination for the URL below (flag order matters):
#     curl --retry 3 -m 10 -fsSL https://api.example.com/data
#
# Hint:
#   - Note: -m (--max-time) is for the whole request, while --connect-timeout
#     is only for the connection phase. For scripts, -m is used more often.

# I AM NOT DONE

URL="https://api.example.com/data"

# TODO: yuqoridagi kombinatsiyani chiqaring
echo "curl $URL"

# === TEST META ===
# @test:stdout: curl --retry 3 -m 10 -fsSL https://api.example.com/data
# @test:exit: 0
