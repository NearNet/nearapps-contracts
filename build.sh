#!/bin/bash
set -ex

cargo +stable build --target wasm32-unknown-unknown --release

cp target/wasm32-unknown-unknown/release/nearapps_exec.wasm ./res/
cp target/wasm32-unknown-unknown/release/nearapps_counter.wasm ./res/
cp target/wasm32-unknown-unknown/release/nearapps_wallet.wasm ./res/
cp target/wasm32-unknown-unknown/release/nearapps_nft.wasm ./res/
