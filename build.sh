#!/bin/bash
set -ex

REPO=$(git rev-parse --show-toplevel)

# every app's directory name
PROJECT_DIR=( \
  "app-counter" \
  "app-crypto" \
  "app-exec" \
  "app-nft-series" \
  "app-send-near" \
  "app-send-nft" \
  "app-user-factory" \
  "app-wallet" \
)
# every app's binary name
PROJECT_NAME=( \
  "nearapps-counter" \
  "nearapps-crypto" \
  "nearapps-exec" \
  "nearapps-nft-series" \
  "nearapps-send-near" \
  "nearapps-send-nft" \
  "nearapps-user-factory" \
  "nearapps-wallet" \
)

# the target for the binaries and documentation
TARGET="wasm32-unknown-unknown"

# triggers all build.rs steps
for i in "${PROJECT_DIR[@]}"; do touch --no-create "$REPO/$i/build.rs"; done
# in this way the wasm files will have up to date
# versioning information

# build the contract's wasm binaries
cargo +stable build --target $TARGET --release "$@"
# they are stored on $REPO/target/$TARGET/release

# creates the contract's code documentation
eval "cargo doc --release --target $TARGET --document-private-items --no-deps $(printf ' -p %q ' "${PROJECT_NAME[@]}")"
# they are stored on $REPO/target/$TARGET/doc/
# inside of it there are index.html files, such as:
# $REPO/target/$TARGET/doc/nearapps_counter/index.html

# whre the wasm binaries are stored
RELEASE_DIR="$REPO/target/$TARGET/release"
# where we want to copy them into
DEST_DIR="$REPO/res"

# makes the wasm copy into DEST_DIR
for i in "${PROJECT_NAME[@]}"; do cp "$RELEASE_DIR/$(echo $i | tr "-" "_").wasm" "$DEST_DIR/"; done

# reduces the wasm size
WASM_FILES="$DEST_DIR/*.wasm"
for f in $WASM_FILES
do
  wasm-opt -Oz -o "$f" "$f"
done
# note: for more info, check:
# https://github.com/WebAssembly/binaryen
# https://rustwasm.github.io/book/reference/code-size.html#use-the-wasm-opt-tool

# shows the wasm binaries and their size
ls -lah $DEST_DIR/*.wasm | awk '{print $5 " " $9}'
