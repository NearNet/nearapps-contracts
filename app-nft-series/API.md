## NFT

Contract Address:

- `testnet`: [`nft.naps.testnet`](https://explorer.testnet.near.org/accounts/nft.naps.testnet)
- `mainnet`: [`nft.nearapps.near`](https://explorer.mainnet.near.org/accounts/nft.nearapps.near)

### Interface

methods:

- `new`
- `new_default_meta`
- `get_owner`
- `change_owner`
- `nft_mint_logged`
- `nft_series_create_logged`
- `nft_series_mint_logged`
- `nft_series_supply`
- `nft_series_get`
- `nft_series_get_minted_tokens_vec`
- `nft_series_set_mintable_logged`
- `nft_series_set_capacity_logged`
- `nft_transfer_logged`
- `nft_transfer_call_logged`
- `nft_token`
- `nft_approve_logged`
- `nft_revoke_logged`
- `nft_revoke_all_logged`
- `nft_is_approved`
- `nft_total_supply`
- `nft_tokens`
- `nft_supply_for_owner`
- `nft_tokens_for_owner`
- `nft_metadata`


#### Initialization

method: `new`  
description: Initializes the NftSeries contract.

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
- `nearapps_logger`: string - the account_id of which contract will log the `nearapps_tags`.

###### Returns

Has no returns.

#### Initialization with a default Meta

method: `new_default_meta`  
description: Initializes the NftSeries contract, using a dummy nft contract metadata.

###### Parameters

- `owner_id`: string - the account_id of who will own the contract
- `nearapps_logger`: string - the account_id of which contract will log the `nearapps_tags`.

###### Returns

Has no returns.


#### Owners Management

methods:

- `get_owner`
- `change_owner`

##### Get Owner

method: `get_owner`  
description: Gets the contract's owner.

###### Parameters

Has no parameters.

###### Returns

- `owner`: string - the account_id of the owner.

##### Change Owner

method: `change_owner`  
description: Changes the contract's owner.

###### Parameters

`new_owner`: string - the account_id of the new owner.

###### Returns

Has no returns.


#### NFT Minting

method: `nft_mint_logged`  
description: Creates a new nft token.
The `token_id` cannot contain the series delimiter character, which is `:`.

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
- `nearapps_tags`: object - the tags information.
    - `app_id`: string - app tag.
    - `action_id`: string - action number.
    - `user_id`: string - user account_id tag.

###### Returns

- `token`: object - the standard nft token information.


#### Standard NFT Operations

methods:

- `nft_transfer_logged`
- `nft_transfer_call_logged`
- `nft_token`
- `nft_approve_logged`
- `nft_revoke_logged`
- `nft_revoke_all`
- `nft_is_approved`
- `nft_total_supply`
- `nft_tokens`
- `nft_supply_for_owner`
- `nft_tokens_for_owner`
- `nft_metadata`

##### Transfer

method: `nft_transfer_logged`  
description: Simple transfer. Transfer a given `token_id` from current owner to `receiver_id`.

###### Parameters

- `token_id`: string - the token id to transfer.
- `receiver_id`: string - the account to receive the token.
- `approval_id`: optional number - expected approval ID. A number smaller than 2^53, and therefore representable as JSON.
- `memo`: optional string - free-form information.
- `nearapps_tags`: object - the tags information. 
    - `app_id`: string - app tag.
    - `action_id`: string - action number.
    - `user_id`: string - user account_id tag.

###### Returns

Has no return.

##### Transfer Call

method: `nft_transfer_call_logged`  
description: Transfer token and call a method on a receiver contract. A successful workflow will end in a success execution outcome to the callback on the NFT contract at the method `nft_resolve_transfer`.

###### Parameters

- `token_id`: string - the token id to transfer.
- `receiver_id`: string - the account to receive the token.
- `approval_id`: optional number - expected approval ID. A number smaller than 2^53, and therefore representable as JSON.
- `memo`: optional string - free-form information.
- `msg`: String - free-form information that can help the receiver to make a decision to accept or deny the token.
- `nearapps_tags`: object - the tags information. 
    - `app_id`: string - app tag.
    - `action_id`: string - action number.
    - `user_id`: string - user account_id tag.

###### Returns

- `success`: bool - whether the transfer was successful or not.

##### Approval

method: `nft_approve_logged`  
description: Add an approved account for a specific token.

###### Parameters

- `token_id`: string - the token id to give allowance on
- `account_id`: string - the account to allow token transfer
- `msg`: optional string.
- `nearapps_tags`: object - the tags information.
    - `app_id`: string - app tag.
    - `action_id`: string - action number.
    - `user_id`: string - user account_id tag.

###### Returns

- `approval_id`: the id of the approval


##### Check Approval

method: `nft_is_approved`  
description: Check if a token is approved for transfer by a given account, optionally checking an approval_id

###### Parameters

- `token_id`: string - the token id to check allowance on
- `approved_account_id`: string.
- `approval_id`: optional number.


###### Returns

- `is_approved`: boolean - whether it is approved.


##### Revoke

method: `nft_revoke_logged`  
description: Revoke an approved account for a specific token.

###### Parameters

- `token_id`: string - the token id to revoke allowance on
- `account_id`: string - the account to disallow token transfer
- `nearapps_tags`: object - the tags information.
    - `app_id`: string - app tag.
    - `action_id`: string - action number.
    - `user_id`: string - user account_id tag.

###### Returns

Has no returns.


##### Revoke All

method: `nft_revoke_all_logged`  
description: Revoke all approved accounts for a specific token.

###### Parameters

- `token_id`: string - the token id to revoke allowance on
- `nearapps_tags`: object - the tags information.
    - `app_id`: string - app tag.
    - `action_id`: string - action number.
    - `user_id`: string - user account_id tag.


###### Returns

Has no return.


#### NFT Series Operations

methods:

- `nft_series_create_logged`
- `nft_series_supply`
- `nft_series_mint_logged`
- `nft_series_get`
- `nft_series_get_minted_tokens_vec`
- `nft_series_set_mintable_logged`
- `nft_series_set_capacity_logged`

##### NFT Series Creation

method: `nft_series_create_logged`  
description: Creates a new NFT series.

###### Parameters

- `name`: string - the name of the token series
- `capacity`: string - the maximum number of the of tokens that can be minted
- `creator`: string - the account_id of the creator, used for informing
- `nearapps_tags`: object - the tags information. 
    - `app_id`: string - app tag.
    - `action_id`: string - action number.
    - `user_id`: string - user account_id tag.

###### Returns

- `series_id`: string - a number representing the id of the created series.

##### NFT Series Supply

method: `nft_series_supply`  
description: Shows how many series were created.

###### Parameters

Has no parameters.

###### Returns

- `series_quantity`: string - a number representing the number of created series.

##### NFT Series Token Minting

method: `nft_series_mint_logged`  
description: Creates a new nft token from a created token series.

###### Parameters

- `series_id`: string - the series id number
- `token_owner_id`: string - the account_id of who will receive the token.
- `token_metadata`: optional object - the standard nft token metadata.
- `nearapps_tags`: object - the tags information. 
    - `app_id`: string - app tag.
    - `action_id`: string - action number.
    - `user_id`: string - user account_id tag.

###### Returns

- `token`: object - the standard nft token information.


##### NFT Series Query

method: `nft_series_get`  
description: Gets information on a series.

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

##### NFT Series Token List

method: `nft_series_get_minted_tokens_vec`  
description: Get minted tokens from a series.

###### Parameters

- `series_id`: string - the series id number
- `from_index`: optional string - the number of how many tokens to skip.
- `limit`: optional number - 16-bits number to limit how many tokens to show.

###### Returns

- `token_ids`: `string[]` - a list containing the token_id number that were minted under the series.


##### NFT Series Set Mintable

method: `nft_series_set_mintable_logged`  
description: Sets whether a series is mintable or not.

###### Parameters

- `series_id`: string - the series id number.
- `is_mintable`: boolean - choose whether it will be mintable or not.
- `nearapps_tags`: object - the tags information. 
    - `app_id`: string - app tag.
    - `action_id`: string - action number.
    - `user_id`: string - user account_id tag.

###### Returns

Has no returns.

##### NFT Series Set Capacity

method: `nft_series_set_capacity_logged`  
description: Sets the token capacity (the token max length) of a series.

###### Parameters

- `series_id`: string - the series id number.
- `capacity`: string - choose the number of what the capacity will be.
- `nearapps_tags`: object - the tags information. 
    - `app_id`: string - app tag.
    - `action_id`: string - action number.
    - `user_id`: string - user account_id tag.

###### Returns

Has no returns.

