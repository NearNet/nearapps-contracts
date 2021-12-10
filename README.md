# Nearapps Contracts


## Execute Contract

- `testnet`: `naps.testnet` https://explorer.testnet.near.org/accounts/naps.testnet
- `mainnet`: `naps.near` https://explorer.mainnet.near.org/accounts/naps.near

### Interface

methods:

- `init`
- `execute`
- `verify_msg`
- `verify_hashed_msg`

#### Initialization

method: `init`

###### Parameters

- `owner_id`: string - the account_id of who will own the contract
    
###### Returns

Has no returns.

###### Sample

```json
{
}
```

#### Execution of a Proxied Contract Call

method: `execute`

###### Parameters

- `context`: the call context.
    - `contract_call`: the contract call context.
        - `contract_id`: string - the contract's AccountId that is being called.
        - `method_name`: string - the name of the method being called.
        - `args`: string - the arguments for the method that is being called. 
    - `tag_info`: the tags information.
        - `app_id`: string - app tag.
        - `action_id`: string - action number.
        - `user_id`: string - user account_id tag.
    <!-- - `public_key`: string - the public key, in base58 which an optional `{header}:` as prefix. Can be a `Ed25519` or a `Secp256k1` public key. Note: currently disabled as the message still needs to be specified. A placeholder value is being used. -->
    <!-- - `signature`: string - the signature, in base58. Can be a `Ed25519` or a `Secp256k1` signature. Note: currently disabled as the message still needs to be specified. A placeholder value is being used. -->
    
###### Returns

- `result` - the same return that `contract_id`'s method `method_name` with `args` would return.


###### Sample

```json
{
  "context": {
    "contract_call": {
        "contract_id": "nft.naps.testnet",
        "method_name": "nft_transfer_from",
        "args": "\"token_id\": \"1\", \"sender_id\": \"my-account.testnet\", \"receiver_id\": \"my-friend.testnet\", \"approval_id\": \"4711\""
    },
  }
}
```


#### Verification of a Message

method: `verify_msg`

###### Parameters

- `sign`: string - the signature, in base58. Can be a `Ed25519` or a `Secp256k1` signature.
- `pubkey`: string - the public key, in base58 with an optional `{header}:` as prefix. Can be a `Ed25519` or a `Secp256k1` public key. On a missing prefix, `ed25519:` is assumed.
- `msg`: string - the message. It will be hashed internally by the contract.

###### Returns

- `is_match`: boolean - whether the sha256 hash of the `msg` matched the `pubkey` on the `sign`.

###### Sample

```json
{
  "sign": "26gFr4xth7W9K7HPWAxq3BLsua8oTy378mC1MYFiEXHBBpeBjP8WmJEJo8XTBowetvqbRshcQEtBUdwQcAqDyP8T",
  "pubkey": "ed25519:AYWv9RAN1hpSQA4p1DLhCNnpnNXwxhfH9qeHN8B4nJ59",
  "msg": "message"
}
```

#### Verification of a Prehashed Message

method: `verify_hashed_msg`

###### Parameters

- `sign`: string - the signature, in base58. Can be a `Ed25519` or a `Secp256k1` signature.
- `pubkey`: string - the public key, in base58 with an optional `{header}:` as prefix. Can be a `Ed25519` or a `Secp256k1` public key. On a missing prefix, `ed25519:` is assumed.
- `msg_hash`: number[] - the message hash, in a 32-sized array of bytes, resulted from a sha256 hash of them message.

###### Returns

- `is_match`: boolean - whether the `msg_hash` matched the `pubkey` on the `sign`.

###### Sample

```json
{
  "sign": "26gFr4xth7W9K7HPWAxq3BLsua8oTy378mC1MYFiEXHBBpeBjP8WmJEJo8XTBowetvqbRshcQEtBUdwQcAqDyP8T",
  "pubkey": "ed25519:AYWv9RAN1hpSQA4p1DLhCNnpnNXwxhfH9qeHN8B4nJ59",
  "msg_hash": [171, 83, 10, 19, 228, 89, 20, 152, 43, 121, 249, 183, 227, 251, 169, 148, 207, 209, 243, 251, 34, 247, 28, 234, 26, 251, 240, 43, 70, 12, 109, 29]
}
```


## Wallet Creation

- `testnet`: `` 
- `mainnet`: `` 

### Interface

methods:

- `new`
- `create_account`
- `create_subaccount`


#### Initialization

method: `new`

###### Parameters

- `owner`: string - owner account id that will be allowed to make other calls into this contract
- `defaults`: Object - the default parameters to be used during account creation.
    - `initial_amount`: string - the default initial amount to attach to created accounts, in yoctoNear.
    - `allowance`: string - the default allowance to attach to allowed calls on created accounts, in yoctoNear.
    - `allowed_calls`: Object[] - the default allowed calls that new accounts are able to make.
        - `allowance`: optional string - the user's allowance for when calling a contract. If missing, defaults to the `defaults.allowance`.
        - `receiver_id`: string - the contract address that the user is allowed to call into.
        - `method_names`: string[] - list of method names (eg. `["method_a", "method_b"]`) that the user is allowed to call on `receiver_id` contract. An empty list means all methods.

###### Returns

Has no returns.

###### Sample

```json
{
}
```

#### Account Creation

note: not tested.

method: `create_account`

###### Parameters

- `config`: Object - account configuration for the user that is being created.
    - `account_id`: string - the sub-account that is being created. Expected to be a sub-account on `.testnet` or `.near`.
    - `user_public_key`: string - the user/sub-account public key, in base58 with an optional `{header}:` as prefix. Can be a `Ed25519` or a `Secp256k1` public key. On a missing prefix, `ed25519:` is assumed. This value may be generated by the user.
    - `initial_amount`: optional string - the initial  amount of deposit that the user should receive. If missing, defaults to `defaults.initial_amount`.
- `allowed_calls`: optional Object[] - call information that the user is allowed to make. If missing, defaults to `defaults.allowed_calls`. If is an empty list, the user will not be allowed to call any contract.
    - `allowance`: optional string - the user's allowance for when calling a contract. If missing, defaults to the `defaults.allowance`.
    - `receiver_id`: string - the contract address that the user is allowed to call into.
    - `method_names`: string[] - list of method names (eg. `["method_a", "method_b"]`) that the user is allowed to call on `receiver_id` contract. An empty list means all methods.


###### Returns

- `account_created`: boolean - whether the accoutn was successfully created.

###### Sample

```json
{
}
```

#### Sub-Account Creation

note: not tested.

method: `create_subaccount`

###### Parameters

- `config`: Object - account configuration for the user that is being created.
    - `account_id`: string - the sub-account that is being created. It will be postfixed with the wallet's account automatically.
    - `user_public_key`: string - the user/sub-account public key, in base58 with an optional `{header}:` as prefix. Can be a `Ed25519` or a `Secp256k1` public key. On a missing prefix, `ed25519:` is assumed. This value may be generated by the user.
    - `initial_amount`: optional string - the initial  amount of deposit that the user should receive. If missing, defaults to `defaults.initial_amount`.
- `allowed_calls`: optional Object[] - call information that the user is allowed to make. If missing, defaults to `defaults.allowed_calls`. If is an empty list, the user will not be allowed to call any contract.
    - `allowance`: optional string - the user's allowance for when calling a contract. If missing, defaults to the `defaults.allowance`.
    - `receiver_id`: string - the contract address that the user is allowed to call into.
    - `method_names`: string[] - list of method names (eg. `["method_a", "method_b"]`) that the user is allowed to call on `receiver_id` contract. An empty list means all methods.


###### Returns

- `account_created`: boolean - whether the accoutn was successfully created.

###### Sample

```json
{
}
```


## NFT

Contract Address:

- `testnet`: `nft.naps.testnet` https://explorer.testnet.near.org/accounts/nft.naps.testnet
- `mainnet`: `nft.naps.near` not yet deployed

### Interface

methods:

- `new`
- `new_default_meta`
- `nft_mint`
- `nft_transfer`
- `nft_transfer_call`
- `nft_token`
- `nft_approve`
- `nft_revoke`
- `nft_revoke_all`
- `nft_is_approved`
- `nft_total_supply`
- `nft_tokens`
- `nft_supply_for_owner`
- `nft_tokens_for_owner`
- `nft_metadata`
- `nft_series_create`
- `nft_series_mint`
- `nft_series_get`
- `nft_series_get_minted_tokens_vec`
- `nft_series_set_mintable`
- `nft_series_set_capacity`

#### Initialization

method: `new`

###### Parameters

- `owner_id`: string - the account_id of who will own the contract
- `metadata`: object - the standard nft metadata
  - `spec`: stirng - eg. "nft-1.0.0"
  - `name`: string - eg. "Mosaics"
  - `symbol`: string - eg. "MOSIAC"
  - `icon`: optional string - data URL
  - `base_uri`: optional string - centralized gateway known to have reliable access to decentralized storage assets referenced by `reference` or `media` URLs
  - `reference`: optional string - URL to a JSON file with more info
  - `reference_hash`: optional string - base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.

###### Returns

Has no returns.

###### Sample

```json
{
}
```

###### Reference Metadata JSON Sample

```json
{
}
```

###### Nearapps API Sample

```bash
```

#### Initialization with a default Meta

method: `new_default_meta`

###### Parameters

- `owner_id`: string - the account_id of who will own the contract

###### Returns

Has no returns.

###### Sample

```json
{
}
```

###### Reference Metadata JSON Sample

```json
{
}
```

###### Nearapps API Sample

```bash
```

#### NFT Minting


method: `nft_mint`

###### Parameters

- `token_id`: string - the name of the token. Cannot contain the series delimiter (`:`).
- `token_owner_id`: string - the account_id of who will receive the token.
- `token_metadata`: object - the standard nft metadata.

###### Returns

- `token`: object - the standard nft token information.

###### Sample

```json
{
}
```

###### Reference Metadata JSON Sample

```json
{
}
```

###### Nearapps API Sample

```bash
```

#### Standard NFT Operations


methods:

- `nft_transfer`
- `nft_transfer_call`
- `nft_token`
- `nft_resolve_transfer`
- `nft_approve`
- `nft_revoke`
- `nft_revoke_all`
- `nft_is_approved`
- `nft_total_supply`
- `nft_tokens`
- `nft_supply_for_owner`
- `nft_tokens_for_owner`
- `nft_metadata`





##### Transfer

method: `nft_transfer`

###### Parameters

- `token_id`: string - the token id to give allowance on.
- `receiver_id`: string - the account to allow token transfer.
- `approval_id`: optional number - the approval id from `nft_approve_from`.
- `memo`: optional string.

###### Sample

```json
{
  "token_id": "1",
  "receiver_id": "my-friend.testnet"
}
```

###### Returns

- `success`: bool - was the transfer successful or not

###### Nearapps API Sample

```bash
curl --location --request POST 'https://api.nearapps.net/testnet/v1/execute' \
--header 'x-api-key: <api key>' \
--header 'Content-Type: application/json' \
--data-raw '{
    "message": "{\"contract_id\":\"nft.naps.testnet\",\"method_name\":\"nft_transfer_from\",\"args\": \"{\"token_id\":\"1\",\"sender_id\":\"my-account.testnet\", \"receiver_id\":\"my-friend.testnet\"}\",
    "signed": {
        "signature": "4FJecZiY22ReWiJHxCSjDw71Jyd8WVgkkeNfH1Zj21uhQEV1c7QQ4bQYc7QMgH3Tcz5LxYJMxPYuHoETN8i4sQNq",
        "publicKey": "ed25519:D5d84XpgHtTUHwg1hbvT3Ljy6LpeLnJhU34scBC1TNKp"
    }
}'
```

##### Transfer Call

method: `nft_transfer_call`

###### Parameters

- `token_id`: string - the token id to give allowance on
- `receiver_id`: string - the account to allow token transfer

###### Sample

```json
{
  "token_id": "1",
  "receiver_id": "my-friend.testnet"
}
```

###### Returns

- `success`: bool - was the transfer successful or not

###### Nearapps API Sample

```bash
curl --location --request POST 'https://api.nearapps.net/testnet/v1/execute' \
--header 'x-api-key: <api key>' \
--header 'Content-Type: application/json' \
--data-raw '{
    "message": "{\"contract_id\":\"nft.naps.testnet\",\"method_name\":\"nft_transfer_from\",\"args\": \"{\"token_id\":\"1\",\"sender_id\":\"my-account.testnet\", \"receiver_id\":\"my-friend.testnet\"}\",
    "signed": {
        "signature": "4FJecZiY22ReWiJHxCSjDw71Jyd8WVgkkeNfH1Zj21uhQEV1c7QQ4bQYc7QMgH3Tcz5LxYJMxPYuHoETN8i4sQNq",
        "publicKey": "ed25519:D5d84XpgHtTUHwg1hbvT3Ljy6LpeLnJhU34scBC1TNKp"
    }
}'
```

##### Approval

method: `nft_approve`

###### Parameters

- `token_id`: string - the token id to give allowance on
- `account_id`: string - the account to allow token transfer
- `msg`: optional string.

###### Sample

```json
{
  "token_id": "1",
  "account_id": "my-friend.testnet"
}
```

###### Returns

- `approval_id`: the id of the approval

###### Nearapps API Sample

```bash
curl --location --request POST 'https://api.nearapps.net/testnet/v1/execute' \
--header 'x-api-key: <api key>' \
--header 'Content-Type: application/json' \
--data-raw '{
    "message": "{\"contract_id\":\"nft.naps.testnet\",\"method_name\":\"nft_approve_from\",\"args\": \"{\"token_id\":\"1\",\"account_id\":\"my-friend.testnet\"}\"}",
    "sender": "my-account.testnet",
    "signed": {
        "signature": "4FJecZiY22ReWiJHxCSjDw71Jyd8WVgkkeNfH1Zj21uhQEV1c7QQ4bQYc7QMgH3Tcz5LxYJMxPYuHoETN8i4sQNq",
        "publicKey": "ed25519:D5d84XpgHtTUHwg1hbvT3Ljy6LpeLnJhU34scBC1TNKp"
    }
}'
```

##### Check Approval

method: `nft_is_approved`

###### Parameters

- `token_id`: string - the token id to check allowance on
- `approved_account_id`: string.
- `approval_id`: optional number.

###### Sample

```json
{
}
```

###### Returns

- `is_approved`: boolean - whether it is approved.

###### Nearapps API Sample

```bash

```

##### Revoke

method: `nft_revoke`

###### Parameters

- `token_id`: string - the token id to revoke allowance on
- `account_id`: string - the account to disallow token transfer

###### Sample

```json
{
  "token_id": "1",
  "account_id": "my-friend.testnet"
}
```

###### Returns

Has no returns.

###### Nearapps API Sample

```bash
curl --location --request POST 'https://api.nearapps.net/testnet/v1/execute' \
--header 'x-api-key: <api key>' \
--header 'Content-Type: application/json' \
--data-raw '{
    "message": "{\"contract_id\":\"nft.naps.testnet\",\"method_name\":\"nft_revoke_from\",\"args\": \"{\"token_id\":\"1\",\"account_id\":\"my-friend.testnet\"}\"}",
    "sender": "my-account.testnet",
    "signed": {
        "signature": "4FJecZiY22ReWiJHxCSjDw71Jyd8WVgkkeNfH1Zj21uhQEV1c7QQ4bQYc7QMgH3Tcz5LxYJMxPYuHoETN8i4sQNq",
        "publicKey": "ed25519:D5d84XpgHtTUHwg1hbvT3Ljy6LpeLnJhU34scBC1TNKp"
    }
}'
```

##### Revoke All

method: `nft_revoke`

###### Parameters

- `token_id`: string - the token id to revoke allowance on

###### Sample

```json
{
  "token_id": "1",
}
```
###### Returns

Has no return.

###### Nearapps API Sample

```bash
```



#### NFT Series Operations

methods:

- `nft_series_create`
- `nft_series_mint`
- `nft_series_get`
- `nft_series_get_minted_tokens_vec`
- `nft_series_set_mintable`
- `nft_series_set_capacity`


##### NFT Series Creation


method: `nft_series_create`

###### Parameters

- `name`: string - the name of the token series
- `capacity`: string - the maximum number of the of tokens that can be minted
- `creator`: string - the account_id of the creator, used for informing

###### Returns

- `series_id`: string - a number representing the id of the created series.

###### Sample

```json
{
}
```

###### Reference Metadata JSON Sample

```json
{
}
```

###### Nearapps API Sample

```bash
```

##### NFT Series Token Minting


method: `nft_series_mint`

###### Parameters

- `series_id`: string - the series id number
- `token_owner_id`: string - the account_id of who will receive the token.
- `token_metadata`: object - the standard nft metadata.

###### Returns

- `token`: object - the standard nft token information.

###### Sample

```json
{
}
```

###### Reference Metadata JSON Sample

```json
{
}
```

###### Nearapps API Sample

```bash
```

##### NFT Series Query


method: `nft_series_get`

###### Parameters

- `series_id`: string - the series id number

###### Returns

- `series`: object - nft series information.
  - `id`: string - the series id number,
  - `name`: string
  - `creator`: string - the account_id of the creator
  - `len`: string - the number of minted tokens
  - `capacity`: string - the number of how many tokens can be minted
  - `is_mintable`: boolean - whether the series can be minted

###### Sample

```json
{
}
```

###### Reference Metadata JSON Sample

```json
{
}
```

###### Nearapps API Sample

```bash
```

##### NFT Series Token List


method: `nft_series_get_minted_tokens_vec`

###### Parameters

- `series_id`: string - the series id number

###### Returns

- `token_ids`: string[] - a list containing the token_id number that were minted under the series.

###### Sample

```json
{
}
```

###### Reference Metadata JSON Sample

```json
{
}
```

###### Nearapps API Sample

```bash
```

##### NFT Series Set Mintable


method: `nft_series_set_mintable`

###### Parameters

- `series_id`: string - the series id number.
- `is_mintable`: boolean - choose whether it will be mintable or not.

###### Returns

Has no returns.

###### Sample

```json
{
}
```

###### Reference Metadata JSON Sample

```json
{
}
```

###### Nearapps API Sample

```bash
```

##### NFT Series Set Capacity


method: `nft_series_set_capacity`

###### Parameters

- `series_id`: string - the series id number.
- `capacity`: string - choose the number of what the capacity will be.

###### Returns

Has no returns.

###### Sample

```json
{
}
```

###### Reference Metadata JSON Sample

```json
{
}
```

###### Nearapps API Sample

```bash
```





