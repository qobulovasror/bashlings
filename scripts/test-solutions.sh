#!/usr/bin/env bash
#
# test-solutions.sh — har mashq uchun ikki narsani tekshiradi:
#   1) tegilmagan exercises/<chapter>/<name>.sh testdan YIQILADI
#      (aks holda o'quvchi javobni tayyor holda oladi);
#   2) .solutions/<chapter>/<name>.sh mashq o'rniga qo'yilganda O'TADI.
# Mashq fayli va .bashlings/state.toml (progress) oxirida joyiga qaytariladi —
# Ctrl+C bosilsa ham.
#
# CI yoki release-readiness uchun.
#
# Talab: cli/target/release/bashlings (lokal build) yoki bashlings $PATH'da.

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

# Binary topish: avval lokal release build (aynan shu kod), keyin $PATH
if [[ -x "$ROOT/cli/target/release/bashlings" ]]; then
    BIN="$ROOT/cli/target/release/bashlings"
elif command -v bashlings >/dev/null 2>&1; then
    BIN="$(command -v bashlings)"
else
    echo "❌ bashlings binary topilmadi. Avval 'cargo build --release --manifest-path cli/Cargo.toml' qiling."
    exit 2
fi

echo "Using: $BIN ($("$BIN" --version))"
echo ""

if [[ ! -d "$ROOT/.solutions" ]]; then
    echo "❌ .solutions/ katalogi mavjud emas."
    exit 2
fi

TMP="$(mktemp -d)"
STATE="$ROOT/.bashlings/state.toml"
[[ -f "$STATE" ]] && cp "$STATE" "$TMP/state.toml"
CURRENT=""   # hozir yechim bilan almashtirilgan mashq fayli

restore() {
    [[ -n "$CURRENT" ]] && cp "$TMP/exercise.sh" "$CURRENT"
    if [[ -f "$TMP/state.toml" ]]; then cp "$TMP/state.toml" "$STATE"; else rm -f "$STATE"; fi
    rm -rf "$TMP"
}
trap restore EXIT
trap 'exit 130' INT TERM

TOTAL=0
PASS=0
FAILED_NAMES=()

for sol in "$ROOT"/.solutions/*/*.sh; do
    [[ -f "$sol" ]] || continue
    rel="${sol#"$ROOT"/.solutions/}"      # e.g. 01_intro/intro1.sh
    name="$(basename "$sol" .sh)"          # intro1
    ex="$ROOT/exercises/$rel"

    if [[ ! -f "$ex" ]]; then
        echo "⚠  $name: exercises/$rel mavjud emas — o'tkazib yuborildi"
        continue
    fi

    TOTAL=$((TOTAL + 1))
    problem=""

    if "$BIN" run "$name" >/dev/null 2>&1; then
        problem="tegilmagan mashq o'tib ketdi"
    fi

    cp "$ex" "$TMP/exercise.sh"
    CURRENT="$ex"
    cp "$sol" "$ex"
    if ! "$BIN" run "$name" >/dev/null 2>&1; then
        problem="${problem:+$problem; }yechim o'tmadi"
    fi
    cp "$TMP/exercise.sh" "$ex"
    CURRENT=""

    if [[ -z "$problem" ]]; then
        printf "  \033[32m✓\033[0m  %s\n" "$name"
        PASS=$((PASS + 1))
    else
        printf "  \033[31m✗\033[0m  %s — %s\n" "$name" "$problem"
        FAILED_NAMES+=("$name")
    fi
done

echo ""
echo "─────────────────────────────────────"
echo "  Jami:    $TOTAL"
printf "  \033[32mPass:\033[0m    %d\n" "$PASS"
if [[ ${#FAILED_NAMES[@]} -gt 0 ]]; then
    printf "  \033[31mFail:\033[0m    %d  (%s)\n" "${#FAILED_NAMES[@]}" "${FAILED_NAMES[*]}"
    exit 1
fi
echo ""
echo "✅ Hamma mashqlar yiqiladi, hamma yechimlar o'tadi."
