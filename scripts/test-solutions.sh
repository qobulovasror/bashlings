#!/usr/bin/env bash
#
# test-solutions.sh — har .solutions/<chapter>/<name>.sh ni mos
# exercises/<chapter>/<name>.sh joyiga vaqtincha qo'yib `bashlings run`
# orqali tekshiradi, keyin exercise faylini joyiga qaytaradi.
#
# CI yoki release-readiness uchun.
#
# Talab: bashlings binary $PATH'da yoki cli/target/release/bashlings da.

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

# Binary topish: avval $PATH, keyin lokal release build
if command -v bashlings >/dev/null 2>&1; then
    BIN="$(command -v bashlings)"
elif [[ -x "$ROOT/cli/target/release/bashlings" ]]; then
    BIN="$ROOT/cli/target/release/bashlings"
else
    echo "❌ bashlings binary topilmadi. Avval 'cargo build --release --manifest-path cli/Cargo.toml' qiling."
    exit 2
fi

echo "Using: $BIN"
echo ""

if [[ ! -d "$ROOT/.solutions" ]]; then
    echo "❌ .solutions/ katalogi mavjud emas."
    exit 2
fi

TOTAL=0
PASS=0
FAIL=0
FAILED_NAMES=()

for sol in "$ROOT"/.solutions/*/*.sh; do
    [[ -f "$sol" ]] || continue
    rel="${sol#$ROOT/.solutions/}"        # e.g. 01_intro/intro1.sh
    name="$(basename "$sol" .sh)"          # intro1
    ex="$ROOT/exercises/$rel"

    if [[ ! -f "$ex" ]]; then
        echo "⚠  $name: exercises/$rel mavjud emas — o'tkazib yuborildi"
        continue
    fi

    TOTAL=$((TOTAL + 1))
    backup="$(mktemp)"
    cp "$ex" "$backup"
    cp "$sol" "$ex"

    if "$BIN" run "$name" >/dev/null 2>&1; then
        printf "  \033[32m✓\033[0m  %s\n" "$name"
        PASS=$((PASS + 1))
    else
        printf "  \033[31m✗\033[0m  %s\n" "$name"
        FAIL=$((FAIL + 1))
        FAILED_NAMES+=("$name")
    fi

    cp "$backup" "$ex"
    rm -f "$backup"
done

echo ""
echo "─────────────────────────────────────"
echo "  Jami:    $TOTAL"
printf "  \033[32mPass:\033[0m    %d\n" "$PASS"
if [[ "$FAIL" -gt 0 ]]; then
    printf "  \033[31mFail:\033[0m    %d  (%s)\n" "$FAIL" "${FAILED_NAMES[*]}"
    exit 1
fi
echo ""
echo "✅ Hamma yechimlar muvaffaqiyatli o'tdi."
