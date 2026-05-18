#!/usr/bin/env bash
#
# MASHQ: Idempotent cleanup
# DARAJA: ★★★★☆
# MAVZU: part2/04-traps-signals · double-call protection
#
# Cleanup ikki marta chaqirilishidan saqlanish kerak — `__done` flag bilan.
#
# Asosiy kod cleanup'ni QO'LDA bir marta chaqiradi. EXIT trap'da yana
# avtomatik chaqirilishi mumkin edi — lekin flag uni to'xtatadi.
#
# Kutilgan: "tozalandi" FAQAT BIR MARTA chiqishi kerak:
#     main
#     tozalandi

# I AM NOT DONE

__done=0
cleanup() {
    # TODO: agar __done == 1 bo'lsa, darhol return qiling
    # TODO: __done=1 ga o'rnating
    echo "tozalandi"
}

trap cleanup EXIT

echo "main"
cleanup     # qo'lda chaqiramiz — EXIT'da yana chaqirilmasligi kerak

# === TEST META ===
# @test:stdout-cmd: printf 'main\ntozalandi\n'
# @test:exit: 0
