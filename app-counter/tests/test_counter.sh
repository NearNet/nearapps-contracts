!/usr/bin/env bash

set -x

rm -rf "neardev/"

# initial exec deploy and init
WASM="../../res/nearapps_exec.wasm"
near dev-deploy \
  --wasmFile $WASM
# load created acc's id as CONTRACT_NAME
source neardev/dev-account.env
EXEC="$CONTRACT_NAME"
# init exec
METHOD="new"
ARGS='{"owner_id": "'"$EXEC"'"}'
near call \
    "$EXEC" \
    "$METHOD" \
    "$ARGS" \
    --accountId "$EXEC"

# contract creation, deploy and init
COUNTER="counter.$EXEC"
near create-account \
    "$COUNTER" \
    --masterAccount "$EXEC"
# deploy & init - owner (predecessor) is EXEC
WASM="../../res/nearapps_counter.wasm"
METHOD="new"
ARGS='{"nearapps_logger": "'"$EXEC"'"}'
near deploy \
  --wasmFile "$WASM" \
  --contractName "$COUNTER" \
  --initFunction "$METHOD" \
  --initArgs "$ARGS"
# ''

# create user acc
USER0="user-0.$EXEC"
near create-account \
    "$USER0" \
    --masterAccount "$EXEC" \
    --initialBalance "1"

