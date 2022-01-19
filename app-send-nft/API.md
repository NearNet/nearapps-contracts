WIP

## Send Near Contract

- `testnet`: `` 
- `mainnet`: `` 

### Interface

methods:

- `new`
- `get_owner`
- `change_owner`
- `add_nft_protocol`
- `change_nft_protocol`
- `remove_nft_protocol`
- `get_nft_protocols`
- `add_user`
- `remove_user`
- `enable_nft_for_user`
- `disable_nft_for_user`
- `get_tokens_owned_by_users`
- `get_tokens_for_user`
- `get_registered_nfts_for_user`
- `user_send_logged`
- `send_logged`
- `send_call_logged`


#### Initialization

method: `new`  
description: Initializes the contract.

###### Parameters

- `owner`: string - the account_id of who will own the contract
- `nearapps_logger`: string - the account_id of nearapps logging contract


###### Returns

Has no returns.

#### Owner Management

##### Get Owner

method: `get_owner`  
description: Get the contract's owner.

###### Parameters

Has no parameters.

###### Returns

- `account_id`: string - The owner's account_id.

##### Change Owner

method: `change_owner`  
description: Changes the contract's owner.

###### Parameters

- `new_owner`: string - the account_id of the next owner who will own the contract

###### Returns

Has no returns.


#### NFT Protocols Management

##### Add NFT Protocol

method: `add_nft_protocol`  
description: Adds a new Nft contract and registers it's protocol.

###### Parameters

- `nft`: string - the account_id of the nft contract that is being added.
- `protocol`: string - the protocol for that nft contract. Either `"Unknown"` | `"Standard"` | `"NearApps"`.

###### Returns

Has no returns.

##### Change NFT Protocol

method: `change_nft_protocol`  
description: Edits a Nft contract's protocol.


###### Parameters

- `nft`: string - the account_id of the nft contract that is being modified.
- `protocol`: string - the new protocol for that nft contract. Either `"Unknown"` | `"Standard"` | `"NearApps"`.

###### Returns

Has no returns.

##### Remove NFT Protocol

method: `remove_nft_protocol`  
description: Removes a nft protocol.  
Note: No user can be owning any token on this nft contract.  
Note: Actual removal is not yet implemented.

###### Parameters

- `nft`: string - the account_id of the nft contract that is being removed.

###### Returns

Has no returns.

##### Get NFT Protocols

method: `get_nft_protocols`  
description: Shows registered nft contracts and their protocols.

###### Parameters

- `from_index`: optional string - the number of how many nft contracts to skip.
- `limit`: optional number - 16-bits number to limit how many nft contracts to show.

###### Returns

- nft_protocols: string[][] - 2D array for the nft contracts information.
    - string[] - each nft contract element is an array of two inner values.
        - [0]: string - the account_id of the nft contract.
        - [1]: string - the protocol name of the nft contract. Either `"Unknown"` | `"Standard"` | `"NearApps"`.


#### User Management


- `add_user`
- `remove_user`
- `enable_nft_for_user`
- `disable_nft_for_user`
- `get_tokens_owned_by_users`
- `get_tokens_for_user`
- `get_registered_nfts_for_user`


TODO

