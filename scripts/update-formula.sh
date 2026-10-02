#!/usr/bin/env bash
#
# update-formula.sh — homebrew-tap'dagi Formula/bashlings.rb ni berilgan
# release tag bo'yicha yangilaydi (url'lardagi versiya + sha256).
#
# Foydalanish:
#   scripts/update-formula.sh v0.1.2 [path/to/homebrew-tap]
#
# Tap yo'li berilmasa `../homebrew-tap` (bashlings bilan yonma-yon) ishlatiladi.
# Release (binar .tar.gz + .sha256 fayllari bilan) allaqachon mavjud bo'lishi
# kerak. Natijani homebrew-tap repo'sida commit qiling.

set -euo pipefail

REPO="qobulovasror/bashlings"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

TAG="${1:-}"
[ -n "$TAG" ] || {
    echo "Foydalanish: $0 <tag> [homebrew-tap yo'li]   (masalan: v0.1.2)" >&2
    exit 2
}
TAP="${2:-$ROOT/../homebrew-tap}"
FORMULA="$TAP/Formula/bashlings.rb"
[ -f "$FORMULA" ] || {
    echo "Formula topilmadi: $FORMULA" >&2
    exit 1
}

TARGETS="
aarch64-apple-darwin
x86_64-apple-darwin
aarch64-unknown-linux-gnu
x86_64-unknown-linux-gnu
"

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

echo "Tag: $TAG  →  formula: $FORMULA"

for target in $TARGETS; do
    [ -n "$target" ] || continue
    asset="bashlings-${target}.tar.gz"
    url="https://github.com/${REPO}/releases/download/${TAG}/${asset}"

    # sha256'ni release'dagi .sha256 fayldan olamiz (bo'lmasa o'zimiz hisoblaymiz)
    if curl -fsSL "${url}.sha256" -o "$tmp/${asset}.sha256" 2>/dev/null; then
        sha="$(awk '{print $1}' "$tmp/${asset}.sha256")"
    else
        echo "  .sha256 yo'q, $asset yuklab hisoblayapmiz..."
        curl -fsSL "$url" -o "$tmp/$asset"
        if command -v sha256sum >/dev/null 2>&1; then
            sha="$(sha256sum "$tmp/$asset" | awk '{print $1}')"
        else
            sha="$(shasum -a 256 "$tmp/$asset" | awk '{print $1}')"
        fi
    fi
    echo "  $target → $sha"

    # Shu target'ning url'i (versiya) va keyingi qatordagi sha256'ni yangilash.
    URL="$url" SHA="$sha" ASSET="$asset" perl -0pi -e '
        s{url "[^"]*/\Q$ENV{ASSET}\E"\n(\s*)sha256 "[^"]*"}{url "$ENV{URL}"\n${1}sha256 "$ENV{SHA}"}g
    ' "$FORMULA"
    grep -q "$sha" "$FORMULA" || {
        echo "Xato: $target uchun url/sha256 qatori topilmadi" >&2
        exit 1
    }
done

echo "✅ Formula yangilandi. Tekshirish: brew audit --strict qobulovasror/tap/bashlings"
echo "   Keyin homebrew-tap repo'sida commit qiling."
