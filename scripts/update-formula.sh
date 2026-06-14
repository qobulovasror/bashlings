#!/usr/bin/env bash
#
# update-formula.sh — Homebrew formuladagi url/sha256/version qiymatlarini
# berilgan release tag bo'yicha avtomatik to'ldiradi.
#
# Foydalanish:
#   scripts/update-formula.sh v0.1.0
#
# Release (binar .tar.gz + .sha256 fayllari bilan) allaqachon mavjud bo'lishi
# kerak. Natijani `homebrew-bashlings` tap repo'siga commit qiling.

set -euo pipefail

REPO="qobulovasror/bashlings"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FORMULA="$ROOT/cli/Formula/bashlings.rb"

TAG="${1:-}"
[ -n "$TAG" ] || {
    echo "Foydalanish: $0 <tag>   (masalan: v0.1.0)" >&2
    exit 2
}
VERSION="${TAG#v}"

TARGETS="
aarch64-apple-darwin
x86_64-apple-darwin
aarch64-unknown-linux-gnu
x86_64-unknown-linux-gnu
"

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

echo "Tag: $TAG  →  formula: $FORMULA"

# version qatorini yangilash
sed -i.bak "s/^  version \".*\"/  version \"$VERSION\"/" "$FORMULA"

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
    # Shu target uchun url va sha256 qatorlarini yangilash
    sed -i.bak \
        -e "s#releases/download/[^/]*/bashlings-${target}.tar.gz#releases/download/${TAG}/bashlings-${target}.tar.gz#" \
        -e "s/REPLACE_WITH_SHA256_${target}/${sha}/" \
        "$FORMULA"
    # Allaqachon to'ldirilgan bo'lsa (qayta ishga tushirishda) ham yangilash:
    perl -0pi -e "s{(bashlings-\Q${target}\E\.tar\.gz\"\n      sha256 \")[0-9a-f]{64}(\")}{\${1}${sha}\${2}}g" "$FORMULA"
done

rm -f "$FORMULA.bak"
echo "✅ Formula yangilandi. Endi homebrew-bashlings tap repo'siga commit qiling."
