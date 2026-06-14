#!/usr/bin/env bash
#
# record-demo.sh — `bashlings` ning haqiqiy chiqishidan asciinema v2 cast
# fayllarini qayta yaratadi (uz + en). Chiqish real; faqat yozish animatsiyasi
# va vaqtlar sintez qilinadi.
#
# Natija:
#   docs/public/casts/bashlings.cast      (o'zbekcha)
#   docs/public/casts/bashlings.en.cast   (inglizcha)
#
# Talab: python3, va `bashlings` (cli/target/release/bashlings yoki $PATH).

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

if command -v bashlings >/dev/null 2>&1; then
    BIN="$(command -v bashlings)"
elif [[ -x "$ROOT/cli/target/release/bashlings" ]]; then
    BIN="$ROOT/cli/target/release/bashlings"
else
    echo "❌ bashlings topilmadi. Avval: cargo build --release --manifest-path cli/Cargo.toml" >&2
    exit 2
fi

TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT
EX="exercises/01_intro/intro1.sh"
STRIP="s#$ROOT/##g"

capture() {  # <lang_env> <suffix>
    local lang="$1" suf="$2"
    CLICOLOR_FORCE=1 ${lang:+BASHLINGS_LANG=$lang} "$BIN" --version >"$TMP/version$suf.out" 2>&1
    CLICOLOR_FORCE=1 ${lang:+BASHLINGS_LANG=$lang} "$BIN" run intro1 >"$TMP/fail$suf.out" 2>&1 || true
    sed -i.bak "$STRIP" "$TMP/fail$suf.out" && rm -f "$TMP/fail$suf.out.bak"
}

# UZ + EN fail/version
capture "" ""
capture "en" ".en"

# pass output: vaqtincha tuzatib olamiz, so'ng to'liq tiklaymiz
cp "$EX" "$TMP/intro1.bak"
sed -i.bak 's/^eko /echo /' "$EX" && rm -f "$EX.bak"
CLICOLOR_FORCE=1 "$BIN" run intro1 >"$TMP/pass.out" 2>&1 || true
git checkout -- "$EX"
cp "$EX" "$TMP/intro1.bak"
sed -i.bak 's/^eko /echo /' "$EX" && rm -f "$EX.bak"
CLICOLOR_FORCE=1 BASHLINGS_LANG=en "$BIN" run intro1 >"$TMP/pass.en.out" 2>&1 || true
git checkout -- "$EX"

# progress
CLICOLOR_FORCE=1 "$BIN" progress >"$TMP/progress.out" 2>&1
CLICOLOR_FORCE=1 BASHLINGS_LANG=en "$BIN" progress >"$TMP/progress.en.out" 2>&1

CAST_DIR="$TMP" OUT_DIR="$ROOT/docs/public/casts" python3 - <<'PY'
import json, os
CAST_DIR = os.environ["CAST_DIR"]; OUT_DIR = os.environ["OUT_DIR"]
os.makedirs(OUT_DIR, exist_ok=True)
ESC = "\x1b"
PROMPT = ESC + "[32m$" + ESC + "[39m "
GREEN = lambda s: ESC + "[32m" + s + ESC + "[39m"

def read(n):
    with open(os.path.join(CAST_DIR, n), encoding="utf-8") as f:
        return f.read()

def crlf(s):
    return s.rstrip("\n").replace("\n", "\r\n") + "\r\n"

def steps(lang):
    suf = ".en" if lang == "en" else ""
    if lang == "en":
        install = "# Install (no Rust needed):  curl -fsSL .../scripts/install.sh | sh"
        fix = "# Fix it: eko -> echo, then save the file"
    else:
        install = "# O'rnatish (Rust shart emas):  curl -fsSL .../scripts/install.sh | sh"
        fix = "# Tuzatamiz: eko -> echo, so'ng faylni saqlang"
    return [
        (install, None, True),
        ("bashlings --version", read("version%s.out" % suf), False),
        ("bashlings run intro1", read("fail%s.out" % suf), False),
        (fix, None, True),
        ("bashlings run intro1", read("pass%s.out" % suf), False),
        ("bashlings progress", read("progress%s.out" % suf), False),
    ]

def build(lang):
    events = []; t = 0.5
    def emit(d):
        nonlocal t; events.append([round(t, 3), "o", d])
    for cmd, out, is_comment in steps(lang):
        emit(PROMPT)
        for ch in cmd:
            t += 0.045; emit(GREEN(ch) if is_comment else ch)
        t += 0.3; emit("\r\n")
        if out is not None:
            t += 0.5; emit(crlf(out)); t += 2.2
        else:
            t += 1.2
    t += 1.5; emit("\r\n")
    header = {"version": 2, "width": 100, "height": 30, "timestamp": 1718000000,
              "title": "bashlings demo", "env": {"TERM": "xterm-256color", "SHELL": "/bin/bash"}}
    lines = [json.dumps(header)] + [json.dumps(e, ensure_ascii=False) for e in events]
    name = "bashlings.en.cast" if lang == "en" else "bashlings.cast"
    with open(os.path.join(OUT_DIR, name), "w", encoding="utf-8") as f:
        f.write("\n".join(lines) + "\n")
    print("wrote", name, "— events", len(events))

build("uz"); build("en")
PY

echo "✅ Cast'lar yangilandi: docs/public/casts/"
