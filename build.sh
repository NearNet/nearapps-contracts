#!/bin/bash
set -ex

cargo +stable build --target wasm32-unknown-unknown --release

RELEASE_DIR="./target/wasm32-unknown-unknown/release"
DEST_DIR="./res"

cp "$RELEASE_DIR/nearapps_exec.wasm" "$DEST_DIR/"
cp "$RELEASE_DIR/nearapps_counter.wasm" "$DEST_DIR/"
cp "$RELEASE_DIR/nearapps_wallet.wasm" "$DEST_DIR/"
cp "$RELEASE_DIR/nearapps_nft.wasm" "$DEST_DIR/"

# reduces wasm size
# https://github.com/WebAssembly/binaryen
# https://rustwasm.github.io/book/reference/code-size.html#use-the-wasm-opt-tool
WASM_FILES="$DEST_DIR/*.wasm"
for f in $WASM_FILES
do
  wasm-opt -Oz -o "$f" "$f"
done

ls -lah res/*.wasm | awk '{print $5 " " $9}'
