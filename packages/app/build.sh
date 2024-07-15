#!/bin/sh
# FILEPATH: /workspaces/libsrc/packages/app/build.sh
cargo build --release -p src-lsp-browser --target wasm32-unknown-unknown;

# if we'are in a docker container the output directory is /scratch/cargo_target
# if we're not in a docker container the output directory is ../../target
OUT_DIR="/scratch/cargo_target"
if [ ! -d "$OUT_DIR" ]; then
  OUT_DIR="../../target"
fi

wasm-bindgen --out-dir assets/wasm --target web --typescript ${OUT_DIR}/wasm32-unknown-unknown/release/src_lsp_browser.wasm;
webpack;
mv ../../book/playground/*.wasm ../../book
mv ../../book/playground/*.ttf ../../book
cp ../../book/taocp.png ../../book/playground
