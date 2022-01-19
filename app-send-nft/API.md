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
- `get_enabled_nfts_for_user`
- `get_tokens_owned_by_users`
- `get_tokens_for_user`
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

- `nft_protocols`: `string[][]` - 2D array for the nft contracts information.
    - `string[]` - each nft contract element is an array of two inner values.
        - `[0]`: string - the account_id of the nft contract.
        - `[1]`: string - the protocol name of the nft contract. Either `"Unknown"` | `"Standard"` | `"NearApps"`.


#### User Management

- `add_user`
- `remove_user`
- `enable_nft_for_user`
- `disable_nft_for_user`
- `get_enabled_nfts_for_user`
- `get_tokens_owned_by_users`
- `get_tokens_for_user`

##### Add User

method: `add_user`  
description: Register a user on this contract.  
The user must also have specific nft contracts enabled for them.

###### Parameters

- `user`: string - The account_id of the user that is being added.

###### Returns

Has no returns.

##### Remove User

method: `remove_user`  
description: Un-register a user from this contract.  
The user must not be owning any token.  
Note: Actual removal is not yet implemented.


###### Parameters

- `user`: string - The account_id of the user that is being removed.

###### Returns

Has no returns.

##### Enable NFT For User

method: `enable_nft_for_user`  
description: Enables a user to be able to interact with a given nft contract. Otherwise, the user cannot register a token from that nft contract.  
The user must also be registered themselves, See `add_user`.


###### Parameters

- `user`: string - The account_id of the user that will have a NFT contract enabled for them.
- `nft`: string - The account_id of the nft contract that will be enabled for the user.

###### Returns

Has no returns.

##### Disable NFT For User

method: `disable_nft_for_user`  
description: Makes a user to be unable to interact with a given nft contract.  
The user must not be owning any token on a given nft contract.  
Note: Actual disabling is not yet implemented.

###### Parameters

- `user`: string - The account_id of the user that will have a NFT contract disabled for them.
- `nft`: string - The account_id of the nft contract that will be disabled for the user.

###### Returns

Has no returns.

##### Get Enabled Nfts For User

method: `get_enabled_nfts_for_user`  
description: Get nft contracts that a user has enabled for their usage.  
The nft account_id elements are ordered by their name, so enabling/disabling nfts can change the elements position.

###### Parameters

- `user`: string - The account_id of the user from whom information will be shown.
- `from_index`: optional string - the number of how many nft account_id's to skip.
- `limit`: optional number - 16-bits number to limit how many nft account_id's to show.

###### Returns

- `nfts`: `string[]` - An array of nft account_id's enabled by that user.

##### Get Tokens Owned By Users

method: `get_tokens_owned_by_users`  
description: Get token_id's of tokens that are owned by users for a given nft contract.  
Note: The token_id elements are ordered by their name, so additions and removals of tokens can change the elements position.

###### Parameters

- `nft`: string - The account_id of the nft contract that will show it's token_id's and their owners.
- `from_index`: optional string - the number of how many token_id's and owners to skip.
- `limit`: optional number - 16-bits number to limit how many token_id's and owners to show.

###### Returns

- `token_ids`: `string[][]` - 2D array for the token_ids and owners.
    - `string[]` - each element is an array of two inner values.
        - `[0]`: string - the token_id.
        - `[1]`: string - the account_id of the owner.

##### Get Tokens For User

method: `get_tokens_for_user`  
description: Get token_id's of tokens that are owned by a user in a given nft contract.  
Note: The token_id elements are ordered by their name, so additions and removals of tokens can change the elements position.

###### Parameters

- `user`: string - The account_id of the user from whom information will be shown.
- `nft`: string - The account_id of the nft contract that will show it's token_id's owned by the user.
- `from_index`: optional string - the number of how many token_id's to skip.
- `limit`: optional number - 16-bits number to limit how many token_id's to show.

###### Returns

- `token_ids`: `string[]` - An array of token_id owned by that user for that nft contract.


#### Sending Token

methods:

- `send_logged`
- `send_call_logged`
- `user_send_logged`
- `user_send_call_logged`


##### Send

method: `send_logged`  
description: Sends the `token_id` to `receiver`.  
This will de-register the token from the current user.
In case of an (external contract call) transfer failure, an internall callback will re-register the token for the previous user.

###### Parameters

- `nft_contract`: string - The account_id of the nft contract where the token resides.
- `sender`: string - The account_id of the indirect owner of the token,.
- `receiver`: string - The account_id of who will receive the token on the nft_contract.
- `token_id`: string - The token_id of the token that is being transferred.
- `approval_id`: optional number - Expected approval ID. A number smaller than 2^53, and therefore representable as JSON. 
- `memo`: optional string - free-form information.
- `nearapps_tags`: object - the tags information.
    - `app_id`: string - app tag.
    - `action_id`: string - action number.
    - `user_id`: string - user account_id tag.

###### Returns

- `transfer_success`: bool - Whether the transfer was successful.

##### Send call

method: `send_call_logged`  
description: Sends the `token_id` to `receiver`, making the nft
call a function on the receiver.  
This will de-register the token from the current user.
In case of an (external contract call) transfer failure, an internall callback will re-register the token for the previous user.

###### Parameters

- `nft_contract`: string - The account_id of the nft contract where the token resides.
- `sender`: string - The account_id of the indirect owner of the token,.
- `receiver`: string - The account_id of who will receive the token on the nft_contract.
- `token_id`: string - The token_id of the token that is being transferred.
- `approval_id`: optional number - Expected approval ID. A number smaller than 2^53, and therefore representable as JSON. 
- `memo`: optional string - free-form information.
- `memo`: optional string - free-form information that will be send to the receiver.
- `nearapps_tags`: object - the tags information.
    - `app_id`: string - app tag.
    - `action_id`: string - action number.
    - `user_id`: string - user account_id tag.

###### Returns

- `transfer_success`: bool - Whether the transfer was successful.

##### Send (by user)

method: `user_send_logged`  
description: Sends the `token_id` to `receiver`.  
This function is intended to be called by users, not by the send-nft owner. The `sender` is implicitly the predecessor. The send-nft owner should use `send_logged` instead.  
This will de-register the token from the current user.
In case of an (external contract call) transfer failure, an internall callback will re-register the token for the previous user.

###### Parameters

- `nft_contract`: string - The account_id of the nft contract where the token resides.
- `receiver`: string - The account_id of who will receive the token on the nft_contract.
- `token_id`: string - The token_id of the token that is being transferred.
- `approval_id`: optional number - Expected approval ID. A number smaller than 2^53, and therefore representable as JSON. 
- `memo`: optional string - free-form information.
- `nearapps_tags`: object - the tags information.
    - `app_id`: string - app tag.
    - `action_id`: string - action number.
    - `user_id`: string - user account_id tag.

###### Returns

- `transfer_success`: bool - Whether the transfer was successful.

##### Send call (by user)

method: `user_send_call_logged`  
description: Sends the `token_id` to `receiver`, making the nft call a function on the receiver.  
This function is intended to be called by users, not by the send-nft contract owner. The `sender` is implicitly the predecessor. The send-nft owner should use `send_call_logged` instead.  
This will de-register the token from the current user.
In case of an (external contract call) transfer failure, an internall callback will re-register the token for the previous user.

###### Parameters

- `nft_contract`: string - The account_id of the nft contract where the token resides.
- `receiver`: string - The account_id of who will receive the token on the nft_contract.
- `token_id`: string - The token_id of the token that is being transferred.
- `approval_id`: optional number - Expected approval ID. A number smaller than 2^53, and therefore representable as JSON. 
- `memo`: optional string - free-form information.
- `memo`: optional string - free-form information that will be send to the receiver.
- `nearapps_tags`: object - the tags information.
    - `app_id`: string - app tag.
    - `action_id`: string - action number.
    - `user_id`: string - user account_id tag.

###### Returns

- `transfer_success`: bool - Whether the transfer was successful.