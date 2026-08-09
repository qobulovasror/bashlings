#!/usr/bin/env bash
# SOLUTION: script8 — read + qolgan stdin

read -r name
echo "Salom, $name!"

qolgan=0
while read -r _; do
    qolgan=$((qolgan + 1))
done
echo "Qolganlari: $qolgan"

# === SETUP (qo'l urmang) ===
# @setup:stdin:
# |Ali
# |Vali
# |Guli

# === TEST META ===
# @test:stdout-cmd: printf 'Salom, Ali!\nQolganlari: 2\n'
# @test:exit: 0
