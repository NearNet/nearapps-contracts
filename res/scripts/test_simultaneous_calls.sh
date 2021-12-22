#!/usr/bin/env bash

set -x

rm -rf "neardev/"

# initial exec deploy and init
WASM="../nearapps_exec.wasm"
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

# nft creation, deploy and init
NFT="nft.$EXEC"
near create-account \
    "$NFT" \
    --masterAccount "$EXEC"
# deploy & init - owner (predecessor) is EXEC
WASM="../nearapps_nft.wasm"
METHOD="new_default_meta"
ARGS='{"owner_id": "'"$EXEC"'"}'
near deploy \
  --wasmFile "$WASM" \
  --contractName "$NFT" \
  --initFunction "$METHOD" \
  --initArgs "$ARGS"
# ''

# create various accounts that will also be able to call EXEC
OWNER_LEN=5
COUNTER=0
while [ $COUNTER -lt $OWNER_LEN ]
do
OWNER_ACC="owner-$COUNTER.$EXEC"
# create acc
near create-account \
    "$OWNER_ACC" \
    --masterAccount "$EXEC" \
    --initialBalance "5"
# add acc as another owner
METHOD="add_owner"
ARGS='{"owner_id": "'"$OWNER_ACC"'"}'
near call \
    "$EXEC" \
    "$METHOD" \
    "$ARGS" \
    --accountId "$EXEC"
((COUNTER++))
done

SERIES_LEN=10
# creates 10 series with a single account (EXEC), timing it
INIT_MILLI1=$(date +%s%3N)
INNER_METHOD="nft_series_create"
METHOD="execute"
COUNTER=0
while [ $COUNTER -lt $SERIES_LEN ]
do
OWNER_ACC="$EXEC"
USER_ACC="some-user-through-$OWNER_ACC"
INNER_ARGS='{\"name\": \"series of '"$USER_ACC"'\", \"capacity\": \"2\", \"creator\": \"'"$USER_ACC"'\"}'
ARGS='{"context": {"contract_call": {"contract_id": "'"$NFT"'", "method_name": "'"$INNER_METHOD"'", "args": "'"$INNER_ARGS"'"}, "tag_info": {"app_id": "nft-id", "action_id": "'"$COUNTER"'", "user_id": "'"$USER_ACC"'"}}}'
near call \
    "$EXEC" \
    "$METHOD" \
    "$ARGS" \
    --accountId "$OWNER_ACC" \
    --gas 300000000000000
((COUNTER++))
done
END_MILLI1=$(date +%s%3N)
echo "took $(($END_MILLI1 - $INIT_MILLI1))ms"

# show how many series are created
METHOD="nft_series_supply"
ARGS='{}'
near call \
    "$NFT" \
    "$METHOD" \
    "$ARGS" \
    --accountId "$EXEC"


# creates 10 series with multiple accounts, timing it
INIT_MILLI2=$(date +%s%3N)
INNER_METHOD="nft_series_create"
METHOD="execute"
COUNTER=0
while [ $COUNTER -lt $SERIES_LEN ]
do
OWNER_ACC="owner-$(($COUNTER % $OWNER_LEN)).$EXEC"
USER_ACC="some-user-through-$OWNER_ACC"
INNER_ARGS='{\"name\": \"series of '"$USER_ACC"'\", \"capacity\": \"2\", \"creator\": \"'"$USER_ACC"'\"}'
ARGS='{"context": {"contract_call": {"contract_id": "'"$NFT"'", "method_name": "'"$INNER_METHOD"'", "args": "'"$INNER_ARGS"'"}, "tag_info": {"app_id": "nft-id", "action_id": "'"$COUNTER"'", "user_id": "'"$USER_ACC"'"}}}'
# executes in the background
near call \
    "$EXEC" \
    "$METHOD" \
    "$ARGS" \
    --accountId "$OWNER_ACC" \
    --gas 300000000000000 \
    &
PIDS[${i}]=$!
((COUNTER++))
done
# wait for all subprocesses 
for PID in ${PIDS[*]}; do
    wait $PID
done
END_MILLI2=$(date +%s%3N)
echo "took $(($END_MILLI2 - $INIT_MILLI2))ms"


# show how many series are created
METHOD="nft_series_supply"
ARGS='{}'
near call \
    "$NFT" \
    "$METHOD" \
    "$ARGS" \
    --accountId "$EXEC"

echo "serial took $(($END_MILLI1 - $INIT_MILLI1))ms"
echo "concurrent took $(($END_MILLI2 - $INIT_MILLI2))ms"