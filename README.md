# Nearapps Contracts

## Execute Contract

- `testnet`: `naps.testnet` https://explorer.testnet.near.org/accounts/naps.testnet
- `mainnet`: `naps.near` https://explorer.mainnet.near.org/accounts/naps.near

### Interface

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

#### Execution of a Proxied Contract Call

method: `execute`

###### Parameters

- `context`: the call context.
    - `contract_call`: the contract call context.
        - `contract_id`: string - the contract's AccountId that is being called.
        - `method_name`: string - the name of the method being called.
        - `args`: string - the arguments for the method that is being called. 
    - `app_id`: optional string.
    - `caller`: optional caller context.
        - `company`: string.
        - `contact`: optional string.
        - `description`: string.
    - `public_key`: string - the public key, in base58 which an optional `{header}:` as prefix. Can be a `Ed25519` or a `Secp256k1` public key. Note: currently disabled as the message still needs to be specified. A placeholder value is being used.
    - `signature`: string - the signature, in base58. Can be a `Ed25519` or a `Secp256k1` signature. Note: currently disabled as the message still needs to be specified. A placeholder value is being used.
    
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
    "public_key": "ed25519:AYWv9RAN1hpSQA4p1DLhCNnpnNXwxhfH9qeHN8B4nJ59",
    "signature": "26gFr4xth7W9K7HPWAxq3BLsua8oTy378mC1MYFiEXHBBpeBjP8WmJEJo8XTBowetvqbRshcQEtBUdwQcAqDyP8T"
  }
}
```

## NFT

Contract Address:

- `testnet`: `nft.naps.testnet` https://explorer.testnet.near.org/accounts/nft.naps.testnet
- `mainnet`: `nft.naps.near` not yet deployed

### Interface

#### Mint NFT

method: `nft_create_series`

###### Parameters

- `creator_id`: string - the creator account of the nft
- `token_metadata`: the tokens metadata
    - `title`: string - ex. `Arch Nemesis: Mail Carrier" or "Parcel #5055`
    - `media`: string - URL to associated media, preferably to decentralized, content-addressed storage
    - `reference`: string - URL to an off-chain JSON file with more info.
    - `copies`: number - number of copies of this set of metadata in existence when token was minted.

###### Returns

- `token_id`: the id of the token created

###### Sample

```json
{
  "creator_id": "my-account.testnet",
  "token_metadata": {
    "title": "Title of my NFT",
    "media": "https://ipfs.io/ipfs/bafybeicvjdjdxhu6oglore3dw26pclogws2adk7gtmsllje6siinqq4uzy",
    "reference": "https://ipfs.io/ipfs/bafybeigo6bjoq6t5dl4fqgvwosplvbkbu5ri6wo3cmkxmypi4sj2j2ae54",
    "copies": 20
  }
}
```

###### Reference Metadata JSON Sample

```json
{
  "creator_id": "my-account.testnet",
  "description": "Title of my NFT that i would like to have a longer description on",
  "custom_field": "I can put what ever i like in this json"
}
```

###### Nearapps API Sample

```bash
curl --location --request POST 'https://api.nearapps.net/testnet/v1/execute' \
--header 'x-api-key: <api key>' \
--header 'Content-Type: application/json' \
--data-raw '{
    "message": "{\"contract_id\":\"nft.naps.testnet\",\"method_name\":\"nft_create_series\",\"args\":{\"creator_id\":\"my-account.testnet\",\"token_metadata\":{\"title\":\"Title of my NFT\",\"media\":\"https: //ipfs.io/ipfs/bafybeicvjdjdxhu6oglore3dw26pclogws2adk7gtmsllje6siinqq4uzy\",\"reference\":\"https://ipfs.io/ipfs/bafybeigo6bjoq6t5dl4fqgvwosplvbkbu5ri6wo3cmkxmypi4sj2j2ae54\",\"copies\":20}}}",
    "sender": "my-account.testnet",
    "signed": {
        "signature": "4FJecZiY22ReWiJHxCSjDw71Jyd8WVgkkeNfH1Zj21uhQEV1c7QQ4bQYc7QMgH3Tcz5LxYJMxPYuHoETN8i4sQNq",
        "publicKey": "ed25519:D5d84XpgHtTUHwg1hbvT3Ljy6LpeLnJhU34scBC1TNKp"
    }
}'
```

#### Claim NFT

##### Approval

method: `nft_approve_from`

###### Parameters

- `token_id`: string - the token id to give allowance on
- `expected_owner_id`: string - the account that is expected to be the owner of the token in question
- `account_id`: string - the account to allow token transfer
- `msg`: optional string.

###### Sample

```json
{
  "token_id": "1",
  "expected_owner_id": "my-account.testnet",
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
    "message": "{\"contract_id\":\"nft.naps.testnet\",\"method_name\":\"nft_approve_from\",\"args\": \"{\"token_id\":\"1\",\"expected_owner_id\": \"my-account.testnet\",\"account_id\":\"my-friend.testnet\"}\"}",
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

method: `nft_revoke_from`

###### Parameters

- `token_id`: string - the token id to revoke allowance on
- `expected_owner_id`: string - the account that is expected to be the owner of the token in question
- `account_id`: string - the account to disallow token transfer

###### Sample

```json
{
  "token_id": "1",
  "expected_owner_id": "my-account.testnet",
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
    "message": "{\"contract_id\":\"nft.naps.testnet\",\"method_name\":\"nft_revoke_from\",\"args\": \"{\"token_id\":\"1\",\"expected_owner_id\": \"my-account.testnet\",\"account_id\":\"my-friend.testnet\"}\"}",
    "sender": "my-account.testnet",
    "signed": {
        "signature": "4FJecZiY22ReWiJHxCSjDw71Jyd8WVgkkeNfH1Zj21uhQEV1c7QQ4bQYc7QMgH3Tcz5LxYJMxPYuHoETN8i4sQNq",
        "publicKey": "ed25519:D5d84XpgHtTUHwg1hbvT3Ljy6LpeLnJhU34scBC1TNKp"
    }
}'
```

##### Revoke All

method: `nft_revoke_from`

###### Parameters

- `token_id`: string - the token id to revoke allowance on
- `expected_owner_id`: string - the account that is expected to be the owner of the token in question

###### Sample

```json
{
  "token_id": "1",
  "expected_owner_id": "my-account.testnet",
}
```
###### Returns

Has no return.

###### Nearapps API Sample

```bash
```

#### Send NFT

##### Transfer

method: `nft_transfer_from`

###### Parameters

- `token_id`: string - the token id to give allowance on.
- `sender_id`: string - the account that is holding the nft.
- `receiver_id`: string - the account to allow token transfer.
- `approval_id`: optional number - the approval id from `nft_approve_from`.
- `memo`: optional string.

###### Sample

```json
{
  "token_id": "1",
  "sender_id": "my-account.testnet",
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
    "sender": "my-account.testnet",
    "signed": {
        "signature": "4FJecZiY22ReWiJHxCSjDw71Jyd8WVgkkeNfH1Zj21uhQEV1c7QQ4bQYc7QMgH3Tcz5LxYJMxPYuHoETN8i4sQNq",
        "publicKey": "ed25519:D5d84XpgHtTUHwg1hbvT3Ljy6LpeLnJhU34scBC1TNKp"
    }
}'
```

##### Transfer Call

method: `nft_transfer_call_from`

###### Parameters

- `token_id`: string - the token id to give allowance on
- `sender_id`: string - the account that is holding the nft
- `receiver_id`: string - the account to allow token transfer

###### Sample

```json
{
  "token_id": "1",
  "sender_id": "my-account.testnet",
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
    "sender": "my-account.testnet",
    "signed": {
        "signature": "4FJecZiY22ReWiJHxCSjDw71Jyd8WVgkkeNfH1Zj21uhQEV1c7QQ4bQYc7QMgH3Tcz5LxYJMxPYuHoETN8i4sQNq",
        "publicKey": "ed25519:D5d84XpgHtTUHwg1hbvT3Ljy6LpeLnJhU34scBC1TNKp"
    }
}'
```
