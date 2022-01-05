TODO: update.

## NFT

Contract Address:

- `testnet`: [`nft.naps.testnet`](https://explorer.testnet.near.org/accounts/nft.naps.testnet)
- `mainnet`: [`nft.nearapps.near`](https://explorer.mainnet.near.org/accounts/nft.nearapps.near)

### Interface

methods:

- `new`
- `new_default_meta`
- `get_owner` (TODO)
- `change_owner` (TODO)

- `nft_mint_logged` (UPDATE)

- `nft_series_create_logged` (UPDATE)
- `nft_series_mint_logged` (UPDATE)
- `nft_series_supply` (TODO)
- `nft_series_get`
- `nft_series_get_minted_tokens_vec`
- `nft_series_set_mintable`
- `nft_series_set_capacity`

- `nft_transfer_logged` (UPDATE)
- `nft_transfer_call_logged` (UPDATE)
- `nft_token`
- `nft_resolve_transfer` (TODO)
- `nft_approve_logged` (UPDATE)
- `nft_revoke_logged` (UPDATE)
- `nft_revoke_all_logged` (UPDATE)
- `nft_is_approved`
- `nft_total_supply`
- `nft_tokens`
- `nft_supply_for_owner`
- `nft_tokens_for_owner`
- `nft_metadata`


#### Initialization

method: `new`

###### Parameters

- `owner_id`: string - the account_id of who will own the contract
- `metadata`: object - the standard nft metadata
    - `spec`: stirng - eg. "nft-1.0.0"
    - `name`: string - eg. "Mosaics"
    - `symbol`: string - eg. "MOSIAC"
    - `icon`: optional string - data URL
    - `base_uri`: optional string - centralized gateway known to have reliable access to decentralized storage assets
      referenced by `reference` or `media` URLs
    - `reference`: optional string - URL to a JSON file with more info
    - `reference_hash`: optional string - base64-encoded sha256 hash of JSON from reference field. Required
      if `reference` is included.

###### Returns

Has no returns.

###### Sample

<!-- TODO: update -->

```json
{
}
```

###### Reference Metadata JSON Sample

<!-- TODO: update -->

```json
{
}
```

###### Nearapps API Sample

<!-- TODO: update -->

```bash
```

#### Initialization with a default Meta

method: `new_default_meta`

###### Parameters

- `owner_id`: string - the account_id of who will own the contract

###### Returns

Has no returns.

###### Sample

<!-- TODO: update -->

```json
{
}
```

###### Reference Metadata JSON Sample

<!-- TODO: update -->

```json
{
}
```

###### Nearapps API Sample

<!-- TODO: update -->

```bash
```

#### NFT Minting

method: `nft_mint`

###### Parameters

- `token_id`: string - the name of the token. Cannot contain the series delimiter (`:`).
- `token_owner_id`: string - the account_id of who will receive the token.
- `token_metadata`: object - the standard nft token metadata.
    - `title`: optional string - the title of the token, eg. "Arch Nemesis: Mail Carrier" or "Parcel #5055".
    - `description`: optional string - free-form description.
    - `media`: optional string - URL to associated media, preferably to decentralized, content-addressed storage.
    - `media_hash`: optional stirng - Base64-encoded sha256 hash of content referenced by the `media` field. Required
      if `media` is included.
    - `copies`: optional string - number of copies of this set of metadata in existence when token was minted.
    - `issued_at`: optional string - ISO 8601 datetime when token was issued or minted.
    - `expires_at`: optional string - ISO 8601 datetime when token expires.
    - `starts_at`: optional string - ISO 8601 datetime when token starts being valid. -`updated_at`: optional string -
      ISO 8601 datetime when token was last updated.
    - `extra`: optional string - anything extra the NFT wants to store on-chain. Can be stringified JSON.
    - `reference`: optional string - URL to an off-chain JSON file with more info.
    - `reference_hash`: optional string - Base64-encoded sha256 hash of JSON from reference field. Required
      if `reference` is included.

###### Returns

- `token`: object - the standard nft token information.

###### Sample

<!-- TODO: update -->

```json
{
}
```

###### Reference Metadata JSON Sample

<!-- TODO: update -->

```json
{
}
```

###### Nearapps API Sample

<!-- TODO: update -->

```bash
```

#### Standard NFT Operations


TODO: remove

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

<!-- TODO: update -->

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

<!-- TODO: update -->

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

<!-- TODO: update -->

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

<!-- TODO: update -->

```json
{
}
```

###### Returns

- `is_approved`: boolean - whether it is approved.

###### Nearapps API Sample

<!-- TODO: update -->

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

<!-- TODO: update -->

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
  "token_id": "1"
}
```

###### Returns

Has no return.

###### Nearapps API Sample

<!-- TODO: update -->

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

<!-- TODO: update -->

```json
{
}
```

###### Reference Metadata JSON Sample

<!-- TODO: update -->

```json
{
}
```

###### Nearapps API Sample

<!-- TODO: update -->

```bash
```

##### NFT Series Token Minting

method: `nft_series_mint`

###### Parameters

- `series_id`: string - the series id number
- `token_owner_id`: string - the account_id of who will receive the token.
- `token_metadata`: optional object - the standard nft token metadata.

###### Returns

- `token`: object - the standard nft token information.

###### Sample

<!-- TODO: update -->

```json
{
}
```

###### Reference Metadata JSON Sample

<!-- TODO: update -->

```json
{
}
```

###### Nearapps API Sample

<!-- TODO: update -->

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

<!-- TODO: update -->

```json
{
}
```

###### Reference Metadata JSON Sample

<!-- TODO: update -->

```json
{
}
```

###### Nearapps API Sample

<!-- TODO: update -->

```bash
```

##### NFT Series Token List

method: `nft_series_get_minted_tokens_vec`

###### Parameters

- `series_id`: string - the series id number

###### Returns

- `token_ids`: string[] - a list containing the token_id number that were minted under the series.

###### Sample

<!-- TODO: update -->

```json
{
}
```

###### Reference Metadata JSON Sample

<!-- TODO: update -->

```json
{
}
```

###### Nearapps API Sample

<!-- TODO: update -->

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

<!-- TODO: update -->

```json
{
}
```

###### Reference Metadata JSON Sample

<!-- TODO: update -->

```json
{
}
```

###### Nearapps API Sample

<!-- TODO: update -->

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

<!-- TODO: update -->

```json
{
}
```

###### Reference Metadata JSON Sample

<!-- TODO: update -->

```json
{
}
```

###### Nearapps API Sample

<!-- TODO: update -->

```bash
```





