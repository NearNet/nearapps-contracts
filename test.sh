#!/bin/bash
set -ex

# the test should not run on the wasm target (although they 
# require that the wasm artifacts have been built), but on the 
# rustc default one instead
#
# eg. x86_64-unknown-linux-gnu, x86_64-apple-darwin

NATIVE_TARGET=$(rustc -vV | sed -n 's|host: ||p')
cargo test --target="$NATIVE_TARGET" "$@"

# note: if a specific target is desired, there are custom
# aliases on .cargo/config.toml, such as test-linux, test-apple
# and test-windows

# note: if RAM usage is too high, you can try having less 
# simultaneous compilations, such as appending --jobs=2 when 
# calling this script