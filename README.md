# Nearapps Contracts

## NFT

Contract Address:

- `testnet`: `nft.naps.testnet` https://explorer.testnet.near.org/accounts/nft.naps.testnet
- `mainnet`: not yet deployed

### Interface

#### Mint NFT

method: `nft_create_series`

Parameters:

- `creator_id`: string - the creator account of the nft
- `token_metadata`: the tokens metadata
    - `title`: string - ex. `Arch Nemesis: Mail Carrier" or "Parcel #5055`
    - `media`: string - URL to associated media, preferably to decentralized, content-addressed storage
    - `reference`: string - URL to an off-chain JSON file with more info.
    - `copies`: number - number of copies of this set of metadata in existence when token was minted.

Returns:

- `token_id`: the id of the token created

Sample:

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

Reference Metadata JSON Sample:

```json
{
  "creator_id": "my-account.testnet",
  "description": "Title of my NFT that i would like to have a longer description on",
  "custom_field": "I can put what ever i like in this json"
}
```

Nearapps API Sample:

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

Approval:

method: `nft_approve`

Parameters:

- `token_id`: string - the token id to give allowance on
- `account_id`: string - the account to allow token transfer

```json
{
  "token_id": "1",
  "account_id": "my-friend.testnet"
}
```

Returns:

- `approval_id`: the id of the approval

Nearapps API Sample:

```bash
curl --location --request POST 'https://api.nearapps.net/testnet/v1/execute' \
--header 'x-api-key: <api key>' \
--header 'Content-Type: application/json' \
--data-raw '{
    "message": "{\"contract_id\":\"nft.naps.testnet\",\"method_name\":\"nft_approve\",\"args\": \"{\"token_id\":\"1\",\"account_id\":\"my-friend.testnet\"}\"}",
    "sender": "my-account.testnet",
    "signed": {
        "signature": "4FJecZiY22ReWiJHxCSjDw71Jyd8WVgkkeNfH1Zj21uhQEV1c7QQ4bQYc7QMgH3Tcz5LxYJMxPYuHoETN8i4sQNq",
        "publicKey": "ed25519:D5d84XpgHtTUHwg1hbvT3Ljy6LpeLnJhU34scBC1TNKp"
    }
}'
```

Transfer:

method: `nft_transfer`

Parameters:

- `token_id`: string - the token id to give allowance on
- `account_id`: string - the account to allow token transfer

```json
{
  "token_id": "1",
  "receiver_id": "my-friend.testnet",
  "approval_id": "4711"
}
```

Returns:

- `success`: bool - was the transfer was successful or not

Nearapps API Sample:

```bash
curl --location --request POST 'https://api.nearapps.net/testnet/v1/execute' \
--header 'x-api-key: <api key>' \
--header 'Content-Type: application/json' \
--data-raw '{
    "message": "{\"contract_id\":\"nft.naps.testnet\",\"method_name\":\"nft_transfer\",\"args\": \"{\"token_id\":\"1\",\"receiver_id\":\"my-friend.testnet\",\"approval_id\":\"4711\"}\",
    "sender": "my-account.testnet",
    "signed": {
        "signature": "4FJecZiY22ReWiJHxCSjDw71Jyd8WVgkkeNfH1Zj21uhQEV1c7QQ4bQYc7QMgH3Tcz5LxYJMxPYuHoETN8i4sQNq",
        "publicKey": "ed25519:D5d84XpgHtTUHwg1hbvT3Ljy6LpeLnJhU34scBC1TNKp"
    }
}'
```

#### Send NFT
