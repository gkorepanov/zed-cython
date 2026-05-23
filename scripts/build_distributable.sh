#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CYRIGHT_DIR="${CYRIGHT_DIR:-"$ROOT/../vs-code-cython/cyright"}"
OUT_DIR="${OUT_DIR:-"$ROOT/dist"}"

VERSION="$(awk -F'"' '/^version = / { print $2; exit }' "$ROOT/extension.toml")"
BUNDLE_NAME="zed-cython-cyright-$VERSION"
BUNDLE_DIR="$OUT_DIR/$BUNDLE_NAME"
ZIP_PATH="$OUT_DIR/$BUNDLE_NAME.zip"
CYRIGHT_PYRIGHT="$CYRIGHT_DIR/packages/pyright"

if [[ ! -f "$CYRIGHT_PYRIGHT/langserver.index.js" || ! -f "$CYRIGHT_PYRIGHT/dist/pyright-langserver.js" ]]; then
  cat >&2 <<EOF
Cyright is not built at:
  $CYRIGHT_PYRIGHT

Build it first:
  git clone https://github.com/ktnrg45/vs-code-cython.git
  cd vs-code-cython
  git submodule update --init --recursive cyright
  cd cyright
  npm install
  npm run build:cli:dev

Or set CYRIGHT_DIR=/path/to/vs-code-cython/cyright.
EOF
  exit 1
fi

mkdir -p "$OUT_DIR"
rm -rf "$BUNDLE_DIR" "$ZIP_PATH"

cargo build --release --target wasm32-wasip1 --locked

mkdir -p "$BUNDLE_DIR/cyright/packages/pyright"

cp "$ROOT/extension.toml" "$BUNDLE_DIR/"
cp "$ROOT/README.md" "$BUNDLE_DIR/"
cp "$ROOT/LICENSE" "$BUNDLE_DIR/"
cp -R "$ROOT/languages" "$BUNDLE_DIR/"
cp "$ROOT/target/wasm32-wasip1/release/zed_cython.wasm" "$BUNDLE_DIR/extension.wasm"

cp "$CYRIGHT_PYRIGHT/index.js" "$BUNDLE_DIR/cyright/packages/pyright/"
cp "$CYRIGHT_PYRIGHT/langserver.index.js" "$BUNDLE_DIR/cyright/packages/pyright/"
cp -R "$CYRIGHT_PYRIGHT/dist" "$BUNDLE_DIR/cyright/packages/pyright/"

cat > "$BUNDLE_DIR/install_macos.sh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

SRC_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
EXTENSIONS_DIR="${ZED_EXTENSIONS_DIR:-"$HOME/Library/Application Support/Zed/extensions"}"
DEST_DIR="$EXTENSIONS_DIR/installed/cython"

mkdir -p "$(dirname "$DEST_DIR")"
rm -rf "$DEST_DIR"
mkdir -p "$DEST_DIR"

if command -v ditto >/dev/null 2>&1; then
  ditto "$SRC_DIR" "$DEST_DIR"
else
  cp -R "$SRC_DIR"/. "$DEST_DIR"/
fi

echo "Installed zed-cython to:"
echo "  $DEST_DIR"
echo
echo "Restart Zed."
EOF
chmod +x "$BUNDLE_DIR/install_macos.sh"

(
  cd "$OUT_DIR"
  if command -v zip >/dev/null 2>&1; then
    zip -qr "$ZIP_PATH" "$BUNDLE_NAME"
  else
    python3 -m zipfile -c "$ZIP_PATH" "$BUNDLE_NAME"
  fi
)

echo "$ZIP_PATH"
