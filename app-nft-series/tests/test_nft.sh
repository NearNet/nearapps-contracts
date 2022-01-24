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
NFT_SERIES="nft-series.$EXEC"
near create-account \
    "$NFT_SERIES" \
    --masterAccount "$EXEC"
# deploy & init - owner (predecessor) is EXEC
WASM="../../res/nearapps_nft_series.wasm"
METHOD="new_default_meta"
ARGS='{"owner_id": "'"$EXEC"'", "nearapps_logger": "'"$EXEC"'"}'
near deploy \
  --wasmFile "$WASM" \
  --contractName "$NFT_SERIES" \
  --initFunction "$METHOD" \
  --initArgs "$ARGS"
# ''

# create user acc
USER0="user-0.$EXEC"
near create-account \
    "$USER0" \
    --masterAccount "$EXEC" \
    --initialBalance "1"

# exec creates a series for user0
TAGS='{\"app_id\": \"nft_series\", \"action_id\": \"0\", \"user_id\": \"user0\"}'
METHOD="nft_series_create"
ARGS='{\"name\": \"my-series\", \"capacity\": \"5\", \"creator\": \"'"$USER0"'\"}'
ARGS='{"contract_id": "'"$NFT_SERIES"'", "method_name": "'"$METHOD"'", "args": "'"$ARGS"'", "nearapps_tags": '"$TAGS"'}'
METHOD="execute_then_log"
near call \
    "$EXEC" \
    "$METHOD" \
    "$ARGS" \
    --accountId "$EXEC" \
    --gas 300000000000000 

eval SERIES_ID=`near call \
    "$EXEC" \
    "$METHOD" \
    "$ARGS" \
    --accountId "$EXEC" \
    --gas 300000000000000 | tail -n 1`
# 0
#
# note: this value result has already stripped the single quotes 
# '' from it - otherwise it would have single quotes, like '0'

# exec mints a token from a series for user0
METHOD="nft_series_mint"
# note: for series id, we are using a value that's already a 
# number (without single quotes).
ARGS='{\"series_id\": \"'"$SERIES_ID"'\", \"token_owner_id\": \"'"$USER0"'\", \"token_metadata\": {\"title\": \"default-title\", \"description\": null, \"media\": null, \"media_hash\": null, \"copies\": null, \"issued_at\": null, \"expires_at\": null, \"starts_at\": null, \"updated_at\": null, \"extra\": null, \"reference\": null, \"reference_hash\": null}}'
ARGS='{"context": {"contract_call": {"contract_id": "'"$NFT_SERIES"'", "method_name": "'"$METHOD"'", "args": "'"$ARGS"'"}, "tag_info": {"app_id": "some-app", "action_id": "1", "user_id": "'"$USER0"'"}}}'
METHOD="execute"
near call \
    "$EXEC" \
    "$METHOD" \
    "$ARGS" \
    --accountId "$EXEC" \
    --depositYocto 6560000000000000000000 \
    --gas 300000000000000

# verify that the token has been minted
METHOD=nft_tokens_for_owner
ARGS='{"account_id": "'"$USER0"'"}'
near view \
    "$NFT_SERIES" \
    "$METHOD" \
    "$ARGS"
