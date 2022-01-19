#!/usr/bin/env bash

set -ex

cargo +stable build --target wasm32-unknown-unknown --release

cp ../target/wasm32-unknown-unknown/release/nearapps_send_near.wasm ../res/
