#!/usr/bin/env sh
#
# bashlings install skripti — GitHub Release'lardan mos binarni yuklab oladi.
#
# Foydalanish:
#   curl -fsSL https://raw.githubusercontent.com/qobulovasror/bashlings/main/scripts/install.sh | sh
#
# Sozlamalar (env):
#   BASHLINGS_BIN_DIR  — o'rnatish katalogi (default: $HOME/.local/bin)
#   BASHLINGS_VERSION  — aniq tag (default: oxirgi release)
#
# Eslatma: bu faqat `bashlings` binarini o'rnatadi. Mashqlar (exercises/)
# uchun reponi clone qiling va `bashlings` ni repo ichidan ishga tushiring.

set -eu

REPO="qobulovasror/bashlings"
BINDIR="${BASHLINGS_BIN_DIR:-$HOME/.local/bin}"

err() {
    echo "❌ $*" >&2
    exit 1
}

need() {
    command -v "$1" >/dev/null 2>&1 || err "kerakli vosita topilmadi: $1"
}

need curl
need tar

# ── Platforma aniqlash ────────────────────────────────────────────────
os="$(uname -s)"
arch="$(uname -m)"

case "$os" in
    Linux) os_part="unknown-linux-gnu" ;;
    Darwin) os_part="apple-darwin" ;;
    *) err "qo'llab-quvvatlanmaydigan OS: $os (Windows uchun Release sahifasidan qo'lda yuklab oling)" ;;
esac

case "$arch" in
    x86_64 | amd64) arch_part="x86_64" ;;
    aarch64 | arm64) arch_part="aarch64" ;;
    *) err "qo'llab-quvvatlanmaydigan arxitektura: $arch" ;;
esac

target="${arch_part}-${os_part}"
asset="bashlings-${target}.tar.gz"

# ── Tag aniqlash ──────────────────────────────────────────────────────
if [ -n "${BASHLINGS_VERSION:-}" ]; then
    tag="$BASHLINGS_VERSION"
else
    tag="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
        | grep '"tag_name"' | head -1 | cut -d'"' -f4)"
fi
[ -n "$tag" ] || err "release tag topilmadi (internet yoki repo nomini tekshiring)"

url="https://github.com/${REPO}/releases/download/${tag}/${asset}"

# ── Yuklab olish + checksum ───────────────────────────────────────────
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

echo "⬇  Yuklanmoqda: $url"
curl -fSL "$url" -o "$tmp/$asset" || err "yuklab bo'lmadi: $asset ($tag) — bu platforma uchun binar bo'lmasligi mumkin"

if curl -fsSL "$url.sha256" -o "$tmp/$asset.sha256" 2>/dev/null; then
    echo "🔒 Checksum tekshirilmoqda..."
    (
        cd "$tmp"
        if command -v sha256sum >/dev/null 2>&1; then
            sha256sum -c "$asset.sha256"
        else
            shasum -a 256 -c "$asset.sha256"
        fi
    ) || err "checksum mos kelmadi"
fi

# ── O'rnatish ─────────────────────────────────────────────────────────
tar -C "$tmp" -xzf "$tmp/$asset"
mkdir -p "$BINDIR"
install -m 0755 "$tmp/bashlings" "$BINDIR/bashlings"

echo "✅ O'rnatildi: $BINDIR/bashlings ($tag)"

case ":$PATH:" in
    *":$BINDIR:"*) ;;
    *) echo "ℹ  PATH'ga qo'shing:  export PATH=\"$BINDIR:\$PATH\"" ;;
esac
