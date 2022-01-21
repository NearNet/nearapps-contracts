#!/usr/bin/env bash

set -x

rm -rf "neardev/"

# there will be 2 exec contracts (exec_old and exec_new), 
# and one "old nft" contract that will migrate into
# a newer version of itself.
#
# the "old nft" will only be controlled by the exec_old.
# then the "old nft" will migrate into a newer version of itself,
# and it should be controlled by only exec_new, because if it is 
# kept being controleld by exec_old, the logging will be 
# duplicated (both exec_old and "new nft" will log).
#
# note: the "new nft" will use the exec_new for logging.

# initial exec_new (newer version) deploy and init
WASM="../nearapps_exec.wasm"
near dev-deploy \
  --wasmFile $WASM
# load created acc's id as CONTRACT_NAME
source neardev/dev-account.env
EXEC_NEW="$CONTRACT_NAME"
# init exec
METHOD="new"
ARGS='{"owner_id": "'"$EXEC_NEW"'"}'
near call \
    "$EXEC_NEW" \
    "$METHOD" \
    "$ARGS" \
    --accountId "$EXEC_NEW"

# exec-old (old version) creation, deploy and init
EXEC_OLD="exec-old.$EXEC_NEW"
near create-account \
    "$EXEC_OLD" \
    --initialBalance "5" \
    --masterAccount "$EXEC_NEW"
# exec-old creation, deploy and init
WASM="333a6e7_nearapps_exec.wasm"
METHOD="new"
ARGS='{"owner_id": "'"$EXEC_OLD"'"}'
near deploy \
  --wasmFile "$WASM" \
  --contractName "$EXEC_OLD" \
  --initFunction "$METHOD" \
  --initArgs "$ARGS"
# ''

# old nft creation, deploy and init
NFT="nft.$EXEC_NEW"
near create-account \
    "$NFT" \
    --initialBalance "5" \
    --masterAccount "$EXEC_NEW"
# old nft creation, deploy and init
WASM="924a055_nearapps_nft.wasm"
METHOD="new_default_meta"
ARGS='{"owner_id": "'"$EXEC_OLD"'"}'
near deploy \
  --wasmFile "$WASM" \
  --contractName "$NFT" \
  --initFunction "$METHOD" \
  --initArgs "$ARGS"

# create users acc
USER0="user-0.$EXEC_NEW"
near create-account \
    "$USER0" \
    --masterAccount "$EXEC_NEW" \
    --initialBalance "20"
# create acc
USER1="user-1.$EXEC_NEW"
near create-account \
    "$USER1" \
    --masterAccount "$EXEC_NEW" \
    --initialBalance "20"
near state $USER0
# 20N (initial)
near state $USER1
# 20N (initial)

# creates a series for user0, on nft (old) with exec_old
TAGS='{"app_id": "nft_series", "action_id": "0", "user_id": "user0"}'
METHOD="nft_series_create"
ARGS='{\"name\": \"my-series\", \"capacity\": \"5\", \"creator\": \"'"$USER0"'\"}'
ARGS='{"context": {"contract_call": {"contract_id": "'"$NFT"'", "method_name": "'"$METHOD"'", "args": "'"$ARGS"'"}, "tag_info": '"$TAGS"'}}'
METHOD="execute"
near call \
    "$EXEC_OLD" \
    "$METHOD" \
    "$ARGS" \
    --accountId "$EXEC_OLD" \
    --gas 300000000000000
# '0'

# mints a token from series 0 to user1, 
# on nft (old) with exec_old
TAGS='{"app_id": "nft_series", "action_id": "1", "user_id": "user0"}'
METHOD="nft_series_mint"
ARGS='{\"series_id\": \"0\", \"token_owner_id\": \"'"$USER1"'\"}'
ARGS='{"context": {"contract_call": {"contract_id": "'"$NFT"'", "method_name": "'"$METHOD"'", "args": "'"$ARGS"'"}, "tag_info": '"$TAGS"'}}'
METHOD="execute"
near call \
    "$EXEC_OLD" \
    "$METHOD" \
    "$ARGS" \
    --accountId "$EXEC_OLD" \
    --depositYocto 7130000000000000000000 \
    --gas 300000000000000
# (token info) (token_id: 'my-series:0:0')

# NFT migration
# the new EXEC is added as a the logger,
# but it still needs to be added as an owner as well
WASM="../nearapps_nft_series.wasm"
METHOD="migrate"
ARGS='{"nearapps_logger": "'"$EXEC_NEW"'"}'
near deploy \
  --wasmFile "$WASM" \
  --contractName "$NFT" \
  --initFunction "$METHOD" \
  --initArgs "$ARGS"

# example view method (deserializes the state)
METHOD="get_owners"
ARGS='{}'
near view \
    "$NFT" \
    "$METHOD" \
    "$ARGS"
# [ '$EXEC_OLD' ]

# add the new exec as an additional owner
TAGS='{"app_id": "nft_series", "action_id": "2", "user_id": "exec_old"}'
METHOD="add_owner"
ARGS='{\"owner_id\": \"'"$EXEC_NEW"'\"}'
ARGS='{"context": {"contract_call": {"contract_id": "'"$NFT"'", "method_name": "'"$METHOD"'", "args": "'"$ARGS"'"}, "tag_info": '"$TAGS"'}}'
METHOD="execute"
near call \
    "$EXEC_OLD" \
    "$METHOD" \
    "$ARGS" \
    --accountId "$EXEC_OLD" \
    --gas 300000000000000
# true

# example view method
METHOD="get_owners"
ARGS='{}'
near view \
    "$NFT" \
    "$METHOD" \
    "$ARGS"
# [ '$EXEC_OLD', '$EXEC_NEW' ]

# mints a token from series 0 to user1, using exec-old still.
# (this should be avoided, as the logs will be duplicated)
#
# the tags will need to be included both for the nft
# and for the (old) exec
#
# the tags for the nft need to have it's quotes escaped
TAGS_INNER='{\"app_id\": \"nft_series\", \"action_id\": \"2\", \"user_id\": \"user0\"}'
# the tags for the (old) exec don't have the quotes escaped
TAGS_OUTER='{"app_id": "nft_series", "action_id": "2", "user_id": "user0"}'
# note the function name on the nft has changed
METHOD="nft_series_mint_logged"
# note the nft now requires the tags
ARGS='{\"series_id\": \"0\", \"token_owner_id\": \"'"$USER1"'\", \"nearapps_tags\": '"$TAGS_INNER"'}'
# note the old exec also still requires the tags
ARGS='{"context": {"contract_call": {"contract_id": "'"$NFT"'", "method_name": "'"$METHOD"'", "args": "'"$ARGS"'"}, "tag_info": '"$TAGS_OUTER"'}}'
METHOD="execute"
near call \
    "$EXEC_OLD" \
    "$METHOD" \
    "$ARGS" \
    --accountId "$EXEC_OLD" \
    --depositYocto 7130000000000000000000 \
    --gas 300000000000000
# (token info) (token_id: 'my-series:0:1')
# note that two logs were emited

# mints a token from series 0 to user1, using exec-new.
# this is preferable as the log won't be duplicated
#
# the tags for the nft need to have it's quotes escaped
TAGS_INNER='{\"app_id\": \"nft_series\", \"action_id\": \"3\", \"user_id\": \"user0\"}'
# note the function name on the nft has changed
METHOD="nft_series_mint_logged"
# note the nft now requires the tags
ARGS='{\"series_id\": \"0\", \"token_owner_id\": \"'"$USER1"'\", \"nearapps_tags\": '"$TAGS_INNER"'}'
# note the new exec uses different fields
# and it also won't log by itself
ARGS='{"contract_id": "'"$NFT"'", "method_name": "'"$METHOD"'", "args": "'"$ARGS"'"}'
# note: (to log by itself, check `execute_then_log`)
METHOD="execute"
near call \
    "$EXEC_NEW" \
    "$METHOD" \
    "$ARGS" \
    --accountId "$EXEC_NEW" \
    --depositYocto 7130000000000000000000 \
    --gas 300000000000000
# (token info) (token_id: 'my-series:0:2')
# note that two logs were emited
