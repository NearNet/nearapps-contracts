## Send Near Contract

- `testnet`: `` 
- `mainnet`: `` 

### Interface

methods:

- `new`
- `get_owner`
- `change_owner`

- `send_attached_logged`
- `send_logged`
- `get_balance`
- `withdraw_logged`


#### Initialization

method: `new`

###### Parameters

- `owner`: string - the account_id of who will own the contract
- `nearapps_logger`: string - the account_id of nearapps logging contract

###### Returns

Has no returns.

#### Owner Management

##### Get Owner

method: `get_owner`

###### Parameters

Has no parameters.

###### Returns

- `account_id`: string - The owner's account_id.

##### Change Owner

method: `change_owner`

###### Parameters

- `new_owner`: string - the account_id of the next owner who will own the contract

###### Returns

Has no returns.


#### Send Near

##### Send Attached

method: `send_attached_logged`

###### Parameters

- `receiver`: string - the account_id of who will receive the attached amount
- `nearapps_tags`: object
    - `app_id`: string
    - `action_id`: string - 64-bits number
    - `user_id`: string - account_id

###### Returns

- `got_sent`: boolean - whether the near tokens were sent successfully

##### Send

method: `send_logged`

###### Parameters

- `receiver`: string - the account_id of who will receive the attached amount
- `amount`: string - 128-bit number for the amount in yocto-nears
- `nearapps_tags`: object
    - `app_id`: string
    - `action_id`: string - 64-bits number
    - `user_id`: string - account_id

###### Returns

- `got_sent`: boolean - whether the near tokens were sent successfully

##### Get Balance

method: `get_balance`

###### Parameters

- `user`: string - the account_id of whom the balance will be checked

###### Returns

- `amount`: string - 128-bit number of the amount of near tokens that user has, in yocto-near

##### Withdraw

method: `withdraw_logged`

###### Parameters

- `nearapps_tags`: object
    - `app_id`: string
    - `action_id`: string - 64-bits number
    - `user_id`: string - account_id

###### Returns

- `got_sent`: boolean - whether the near tokens were sent successfully
