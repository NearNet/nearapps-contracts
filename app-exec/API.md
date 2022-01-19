## Execute Contract

- `testnet`: [`naps.testnet`](https://explorer.testnet.near.org/accounts/naps.testnet)
- `mainnet`: [`v1.nearapps.near`](https://explorer.mainnet.near.org/accounts/v1.nearapps.near)

### Interface

methods:

- `new`
- `execute`
- `execute_then_log`
- `log`
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

```json
{
  "owner_id": "owner-account.near"
}
```

#### Execution of a Proxied Contract Call

method: `execute`  
description: Executes an external contract's function, where all of the logging should be proactively made by the contract that is to be called.  
The logging information must be contained in `args`, under a field named `nearapps_tags`.

###### Parameters

- `contract_id`: string - the contract's AccountId that is being called.
- `method_name`: string - the name of the method being called.
- `args`: string - the arguments for the method that is being called, plus the `nearapps_tags` information.
    - `nearapps_tags`: object - the tags information. Must be contained inside of `args`.
        - `app_id`: string - app tag.
        - `action_id`: string - action number.
        - `user_id`: string - user account_id tag.


###### Returns

- `result` - the same return that `contract_id`'s method `method_name` with `args` would return.

###### Sample


```json
{
  "contract_id": "nft.naps.testnet",
  "method_name": "ft_transfer_logged",
  "args": "\"receiver_id\": \"my-friend.testnet\", \"token_id\": \"1\",  \"msg\": \"\", \"nearapps_tags\": {\"app_id\": \"the-app\", \"action_id\": \"0\", \"user_id\": \"the-user\"}"
}
```

#### Execution of a Proxied Contract Call (2)


method: `execute_then_log`  
description: Similar to `execute`, executes an external contract's function, but the logging is made on the callback, when forwarding the result back to the caller.  
This should be used for methods that don't log for themselves.

###### Parameters

- `contract_id`: string - the contract's AccountId that is being called.
- `method_name`: string - the name of the method being called.
- `args`: string - the arguments for the method that is being called.
- `nearapps_tags`: object - the tags information. Note that this is not contained inside of `args`.
    - `app_id`: string - app tag.
    - `action_id`: string - action number.
    - `user_id`: string - user account_id tag.


###### Returns

- `result` - the same return that `contract_id`'s method `method_name` with `args` would return.

###### Sample


```json
{
  "contract_id": "nft.naps.testnet",
  "method_name": "change_owner",
  "args": "{\"new_owner\": \"my-friend.testnet\"}",
  "nearapps_tags": {"app_id": "the-app", "action_id": "0", "user_id": "the-user"}
}
```

#### Logging

method: `log`  
description: Emits a nearapps log.

###### Parameters

- `nearapps_tags`: object - the tags information.
    - `app_id`: string - app tag.
    - `action_id`: string - action number.
    - `user_id`: string - user account_id tag.


###### Returns

Has no returns.


###### Sample


```json
{
  "nearapps_tags": {"app_id": "the-app", "action_id": "0", "user_id": "the-user"}
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
description: Adds a new owner.  

###### Parameters

- `owner_id`: string - the account_id of who will also own the contract

###### Returns

- `added`: boolean - whether the added account was a newly added one.

###### Sample

```json
{
  "owner_id": "new-owner.testnet"
}
```

##### Remove Owner

method: `remove_owner`  
description: Removes a owner.  

###### Parameters

- `owner_id`: string - the account_id of who will stop owning the contract

###### Returns

- `removed`: boolean - whether the removed account was just removed.

###### Sample

```json
{
  "owner_id": "no-longer-owner.testnet"
}
```

##### Check Owner

method: `is_owner`  
description: Checks if the given account is an owner.  

###### Parameters

- `owner_id`: string - the account_id which the owning status is being checked

###### Returns

- `is_owner`: boolean - whether the account is an owner.

###### Sample

```json
{
  "owner_id": "are-you-an-owner.testnet"
}
```

##### Get Owners

method: `get_owners`  
description: Show owners.

###### Parameters

- `from_index`: optional string - the number of how many owners to skip.
- `limit`: optional number - 16-bits number to limit how many owners to show.

###### Returns

- `owners`: string[] - list of account_ids of the owners.

###### Sample


```json
{
  "from_index": "0",
  "limit": 1
}
```
