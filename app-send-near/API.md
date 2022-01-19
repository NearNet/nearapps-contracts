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


#### Send Near

##### Send Attached

method: `send_attached_logged`  
description: Sends NEAR tokens to the `receiver` account.  
The sender is the call's predecessor account, and the amount is exactly the call's attached amount.  
Returns `true` on success, and a log is made.  
Returns `false` on failure, and the contract adds the amount that didn't get sent to the sender. Ie. the contract tracks that as that sender's balance.

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
description: Sends `amount` NEAR tokens to the `receiver` account.  
The sender is the call's predecessor account, and any attached amount is considered for the sending operation, as well as any balance that the sender already had (as tracked by the contract).  
Returns `true` on success, and a log is made.  
Returns `false` on failure, and the contract adds the amount that didn't get sent to the sender. Ie. the contract tracks that as that sender's balance.

###### Parameters

- `receiver`: string - the account_id of who will receive the attached amount
- `amount`: string - 128-bit number for the amount in yocto-nears
- `nearapps_tags`: object
    - `app_id`: string
    - `action_id`: string - 64-bits number
    - `user_id`: string - account_id

###### Returns

- `got_sent`: boolean - whether the near tokens were sent successfully

###### Sample

description: Person _X_ wants to send 1 NEAR token to user _Y,_ calling the `send_near` contract directly. 

```json
{
    "receiver": "my.friend",
    "amount": "1000000000000000000000000",
    "nearapps_tags": {
        "app_id": "my nice app",
        "action_id": "2",
        "user_id": "my-account.testnet"
    }
}
```

##### Get Balance

method: `get_balance`  
description: Gets the `user`'s balance, as tracked by this contract.

###### Parameters

- `user`: string - the account_id of whom the balance will be checked

###### Returns

- `amount`: string - 128-bit number of the amount of near tokens that user has, in yocto-near

##### Withdraw

method: `withdraw_logged`  
description: Withdraw all of the balance from the user into themselves, as tracked by this contract. The user is the call's predecessor account.

###### Parameters

- `nearapps_tags`: object
    - `app_id`: string
    - `action_id`: string - 64-bits number
    - `user_id`: string - account_id

###### Returns

- `got_sent`: boolean - whether the near tokens were sent successfully
