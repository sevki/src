#!/bin/sh
# FILEPATH: /workspaces/libsrc/packages/app/build.sh
cargo build --release -p src-lsp-browser --target wasm32-unknown-unknown;
wasm-bindgen --out-dir assets/wasm --target web --typescript /scratch/cargo_target/wasm32-unknown-unknown/release/src_lsp_browser.wasm; 
webpack;
mv ../../book/playground/*.wasm ../../book
mv ../../book/playground/*.ttf ../../book
cp ../../book/taocp.png ../../book/playground
