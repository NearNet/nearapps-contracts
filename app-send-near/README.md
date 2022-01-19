## Send Near Contract

- [API](./API.md)

Description: This contract can be used to send NEAR tokens to some receiver, logging on success.  
Usually the caller must attach the tokens that are to get sent. If the sending operation fails - for some type of failures - the amount that failed to get sent is deposited for the sender and this information is tracked by the contract. The sender can then try calling send again, or withdraw their balance.

### Test Scripts

- [Overall Test Script](./tests/test_send_near.sh)