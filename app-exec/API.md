## Execute Contract

- `testnet`: [`naps.testnet`](https://explorer.testnet.near.org/accounts/naps.testnet)
- `mainnet`: [`v1.nearapps.near`](https://explorer.mainnet.near.org/accounts/v1.nearapps.near)

### Interface

methods:

- `new`
- `execute`
- `execute_then_log` (TODO)
- `log` (TODO)
- `add_owner`
- `remove_owner`
- `is_owner`
- `get_owners`

#### Initialization

method: `new`  
description: Initializes the contract.

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

<!-- TODO: update -->

```json
{
  "context": {
    "contract_call": {
      "contract_id": "nft.naps.testnet",
      "method_name": "nft_transfer_from",
      "args": "\"token_id\": \"1\", \"sender_id\": \"my-account.testnet\", \"receiver_id\": \"my-friend.testnet\", \"approval_id\": \"4711\""
    }
  }
}
```

#### Owners Management

methods:

- `add_owner`
- `remove_owner`
- `is_owner`
- `get_owners`

##### Add Owner

method: `add_owner`

###### Parameters

- `owner_id`: string - the account_id of who will also own the contract

###### Returns

- `added`: boolean - whether the account was newly added as an owner.

###### Sample

<!-- TODO: update -->

```json
{
}
```

##### Remove Owner

method: `remove_owner`

###### Parameters

- `owner_id`: string - the account_id of who will stop owning the contract

###### Returns

- `removed`: boolean - whether the account just removed was as an owner.

###### Sample

<!-- TODO: update -->

```json
{
}
```

##### Check Owner

method: `is_owner`

###### Parameters

- `owner_id`: string - the account_id which the owning status is being checked

###### Returns

- `is_owner`: boolean - whether the account is an owner.

###### Sample

<!-- TODO: update -->

```json
{
}
```

##### Get Owners

method: `get_owner`

###### Parameters

No parameters required.

###### Returns

- `owners`: string[] - list of account_ids of the owners.

###### Sample

<!-- TODO: update -->

```json
{
}
```
