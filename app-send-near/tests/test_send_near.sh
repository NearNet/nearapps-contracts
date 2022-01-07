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
SEND_NEAR="send-near.$EXEC"
near create-account \
    "$SEND_NEAR" \
    --masterAccount "$EXEC"

# deploy & init - owner (predecessor) is EXEC
WASM="../../res/nearapps_send_near.wasm"
METHOD="new"
ARGS='{"owner": "'"$EXEC"'", "nearapps_logger": "'"$EXEC"'"}'
near deploy \
  --wasmFile "$WASM" \
  --contractName "$SEND_NEAR" \
  --initFunction "$METHOD" \
  --initArgs "$ARGS"
# ''

# create acc
USER0="user-0.$EXEC"
near create-account \
    "$USER0" \
    --masterAccount "$EXEC" \
    --initialBalance "20"
# create acc
USER1="user-1.$EXEC"
near create-account \
    "$USER1" \
    --masterAccount "$EXEC" \
    --initialBalance "20"
near state $USER0
# 20N (initial)
near state $USER1
# 20N (initial)


ONE_NEAR_IN_YOCTO="1000000000000000000000000"

# ok: user0 sends 1 Near to user1
METHOD='send_attached_logged'
TAGS='{"app_id": "send_near", "action_id": "0", "user_id": "user0"}'
ARGS='{"receiver": "'"$USER1"'", "nearapps_tags": '"$TAGS"'}'
near call \
    "$SEND_NEAR" \
    "$METHOD" \
    "$ARGS" \
    --accountId "$USER0" \
    --depositYocto "$ONE_NEAR_IN_YOCTO" \
    --gas 300000000000000
# true
near state $USER0
# 20N -0N -> 20N -1N (-1)
near state $USER1
# 20N -> 21N (+1)

# ok: confirm that the user0 balance is 0 N
METHOD=get_balance
ARGS='{"user": "'"$USER0"'"}'
near view \
    "$SEND_NEAR" \
    "$METHOD" \
    "$ARGS"
# error: ERR_SEND_NEAR_MISSING_USER

# fail: user0 sends 1 Near to user11 (inexistent user)
METHOD='send_attached_logged'
TAGS='{"app_id": "send_near", "action_id": "1", "user_id": "user0"}'
ARGS='{"receiver": "'"user-11.$EXEC"'", "nearapps_tags": '"$TAGS"'}'
near call \
    "$SEND_NEAR" \
    "$METHOD" \
    "$ARGS" \
    --accountId "$USER0" \
    --depositYocto "$ONE_NEAR_IN_YOCTO" \
    --gas 300000000000000
# false
near state $USER0
# 20N -1N -> 20N -2N (-1)

# ok: confirm that the user0 balance is 1 N
METHOD=get_balance
ARGS='{"user": "'"$USER0"'"}'
near view \
    "$SEND_NEAR" \
    "$METHOD" \
    "$ARGS"
# '1000000000000000000000000'

# ok: user0 sends 0 Near to user1 but doesn't attach anything
METHOD='send_attached_logged'
TAGS='{"app_id": "send_near", "action_id": "2", "user_id": "user0"}'
ARGS='{"receiver": "'"$USER1"'", "nearapps_tags": '"$TAGS"'}'
near call \
    "$SEND_NEAR" \
    "$METHOD" \
    "$ARGS" \
    --accountId "$USER0" \
    --gas 300000000000000
# true
near state $USER0
# 20N -2N -> 20N -2N (uncahnged)
near state $USER1
# 21N -> 21N (uncahnged)

# ok: confirm that the user0 balance is 1 N
METHOD=get_balance
ARGS='{"user": "'"$USER0"'"}'
near view \
    "$SEND_NEAR" \
    "$METHOD" \
    "$ARGS"
# '1000000000000000000000000'

# ok: user0 sends 1 Near to user1 (using explicit amount)
METHOD='send_logged'
TAGS='{"app_id": "send_near", "action_id": "3", "user_id": "user0"}'
ARGS='{"receiver": "'"$USER1"'", "amount": "'"$ONE_NEAR_IN_YOCTO"'", "nearapps_tags": '"$TAGS"'}'
near call \
    "$SEND_NEAR" \
    "$METHOD" \
    "$ARGS" \
    --accountId "$USER0" \
    --depositYocto "$ONE_NEAR_IN_YOCTO" \
    --gas 300000000000000
# true
near state $USER0
# 20N -2N -> 20N -3N (-1)
near state $USER1
# 21N -> 22N (+1)

# ok: user0 sends 1 Near to user1
# using explicit amount; attaches more than needed (2 extra near)
METHOD='send_logged'
TAGS='{"app_id": "send_near", "action_id": "4", "user_id": "user0"}'
ARGS='{"receiver": "'"$USER1"'", "amount": "'"$ONE_NEAR_IN_YOCTO"'", "nearapps_tags": '"$TAGS"'}'
near call \
    "$SEND_NEAR" \
    "$METHOD" \
    "$ARGS" \
    --accountId "$USER0" \
    --depositYocto "3000000000000000000000000" \
    --gas 300000000000000
# true
near state $USER0
# 20N -3N -> 20N -6N (-3)
near state $USER1
# 22N -> 23N (+1)

# ok: confirm that the user0 balance is 1+2 N
METHOD=get_balance
ARGS='{"user": "'"$USER0"'"}'
near view \
    "$SEND_NEAR" \
    "$METHOD" \
    "$ARGS"
# '3000000000000000000000000'

# fail: user0 sends 4 Near to user1 (not enought balance)
# using explicit amount; uses tracked balance
METHOD='send_logged'
TAGS='{"app_id": "send_near", "action_id": "5", "user_id": "user0"}'
ARGS='{"receiver": "'"$USER1"'", "amount": "4000000000000000000000000", "nearapps_tags": '"$TAGS"'}'
near call \
    "$SEND_NEAR" \
    "$METHOD" \
    "$ARGS" \
    --accountId "$USER0" \
    --gas 300000000000000
# error: ERR_SEND_NEAR_INSUFFICIENT_FUNDS

# ok: user0 sends 1 Near to user1
# using explicit amount; uses tracked balance
METHOD='send_logged'
TAGS='{"app_id": "send_near", "action_id": "6", "user_id": "user0"}'
ARGS='{"receiver": "'"$USER1"'", "amount": "'"$ONE_NEAR_IN_YOCTO"'", "nearapps_tags": '"$TAGS"'}'
near call \
    "$SEND_NEAR" \
    "$METHOD" \
    "$ARGS" \
    --accountId "$USER0" \
    --gas 300000000000000
# true
near state $USER0
# 20N -6N -> 20N -6N (unchanged)
near state $USER1
# 23N -> 24N (+1)

# ok: confirm that the user0 balance is 2 N
METHOD=get_balance
ARGS='{"user": "'"$USER0"'"}'
near view \
    "$SEND_NEAR" \
    "$METHOD" \
    "$ARGS"
# '2000000000000000000000000'

# ok: user0 sends 2 Near to user1
# using explicit amount; uses attached and tracked balance
METHOD='send_logged'
TAGS='{"app_id": "send_near", "action_id": "7", "user_id": "user0"}'
ARGS='{"receiver": "'"$USER1"'", "amount": "2000000000000000000000000", "nearapps_tags": '"$TAGS"'}'
near call \
    "$SEND_NEAR" \
    "$METHOD" \
    "$ARGS" \
    --accountId "$USER0" \
    --depositYocto "1000000000000000000000000" \
    --gas 300000000000000
# true
near state $USER0
# 20N -6N -> 20N -7N (-1)
near state $USER1
# 24N -> 26N (+2)

# ok: confirm that the user0 balance is 1 N
METHOD=get_balance
ARGS='{"user": "'"$USER0"'"}'
near view \
    "$SEND_NEAR" \
    "$METHOD" \
    "$ARGS"
# '1000000000000000000000000'

# ok: user0 sends 1 Near to user1
# using explicit amount; uses all of the tracked balance
METHOD='send_logged'
TAGS='{"app_id": "send_near", "action_id": "8", "user_id": "user0"}'
ARGS='{"receiver": "'"$USER1"'", "amount": "'"$ONE_NEAR_IN_YOCTO"'", "nearapps_tags": '"$TAGS"'}'
near call \
    "$SEND_NEAR" \
    "$METHOD" \
    "$ARGS" \
    --accountId "$USER0" \
    --gas 300000000000000
# true
near state $USER0
# 20N -7N -> 20N -7N (unchanged)
near state $USER1
# 26N -> 27N (+1)

# ok: confirm that the user0 balance is 0 N
METHOD=get_balance
ARGS='{"user": "'"$USER0"'"}'
near view \
    "$SEND_NEAR" \
    "$METHOD" \
    "$ARGS"
# error: ERR_SEND_NEAR_MISSING_USER

# ok: user0 sends 0 Near to itself
# using explicit amount; attaches more than needed
METHOD='send_logged'
TAGS='{"app_id": "send_near", "action_id": "9", "user_id": "user0"}'
ARGS='{"receiver": "'"$USER0"'", "amount": "0", "nearapps_tags": '"$TAGS"'}'
near call \
    "$SEND_NEAR" \
    "$METHOD" \
    "$ARGS" \
    --accountId "$USER0" \
    --depositYocto "1000000000000000000000000" \
    --gas 300000000000000
# true
near state $USER0
# 20N -7N -> 20N -8N (-1)
near state $USER1
# 27N -> 27N (unchanged)

# ok: confirm that the user0 balance is 1 N
METHOD=get_balance
ARGS='{"user": "'"$USER0"'"}'
near view \
    "$SEND_NEAR" \
    "$METHOD" \
    "$ARGS"
# '1000000000000000000000000'

# ok: user0 sends 2 Near to user1
# using explicit amount; uses all of the tracked balance
# and the attached amount
METHOD='send_logged'
TAGS='{"app_id": "send_near", "action_id": "10", "user_id": "user0"}'
ARGS='{"receiver": "'"$USER1"'", "amount": "2000000000000000000000000", "nearapps_tags": '"$TAGS"'}'
near call \
    "$SEND_NEAR" \
    "$METHOD" \
    "$ARGS" \
    --accountId "$USER0" \
    --depositYocto "1000000000000000000000000" \
    --gas 300000000000000
# true
near state $USER0
# 20N -8N -> 20N -9N (-1)
near state $USER1
# 27N -> 29N (+2)

# ok: confirm that the user0 balance is 0 N
METHOD=get_balance
ARGS='{"user": "'"$USER0"'"}'
near view \
    "$SEND_NEAR" \
    "$METHOD" \
    "$ARGS"
# error: ERR_SEND_NEAR_MISSING_USER

# ok: user0 sends 0 Near to itself
# using explicit amount; attaches more than needed
METHOD='send_logged'
TAGS='{"app_id": "send_near", "action_id": "11", "user_id": "user0"}'
ARGS='{"receiver": "'"$USER0"'", "amount": "0", "nearapps_tags": '"$TAGS"'}'
near call \
    "$SEND_NEAR" \
    "$METHOD" \
    "$ARGS" \
    --accountId "$USER0" \
    --depositYocto "1000000000000000000000000" \
    --gas 300000000000000
# true
near state $USER0
# 20N -8N -> 20N -9N (-1)
near state $USER1
# 29N -> 29N (unchanged)

# ok: confirm that the user0 balance is 1 N
METHOD=get_balance
ARGS='{"user": "'"$USER0"'"}'
near view \
    "$SEND_NEAR" \
    "$METHOD" \
    "$ARGS"
# '1000000000000000000000000'

# ok: user0 withdraws
METHOD='withdraw_logged'
TAGS='{"app_id": "send_near", "action_id": "12", "user_id": "user0"}'
ARGS='{"nearapps_tags": '"$TAGS"'}'
near call \
    "$SEND_NEAR" \
    "$METHOD" \
    "$ARGS" \
    --accountId "$USER0" \
    --gas 300000000000000
# true
near state $USER0
# 20N -9N -> 20N -8N (+1)

# ok: confirm that the user0 balance is 0 N
METHOD=get_balance
ARGS='{"user": "'"$USER0"'"}'
near view \
    "$SEND_NEAR" \
    "$METHOD" \
    "$ARGS"
# error: ERR_SEND_NEAR_MISSING_USER
