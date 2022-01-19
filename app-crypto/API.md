WIP

## Crypto Contract

- `testnet`: ``
- `mainnet`: ``

### Interface

methods:

- `verify_msg`
- `verify_hashed_msg`
- `ecdsa_secp256k1_sign`
- `ecdsa_secp256k1_sign_recoverable`
- `ecdsa_secp256k1_verify_compressed_msg`
- `ecdsa_secp256k1_verify_uncompressed_msg`
- `ecdsa_secp256k1_verify_prehashed_compressed`
- `ecdsa_secp256k1_verify_prehashed_uncompressed`
- `secp256k1_pubkey_compressed`
- `secp256k1_pubkey_uncompressed`
- `eddsa_ed25519_sign`
- `eddsa_ed25519_sign_prehashed`
- `eddsa_ed25519_verify_bytes`
- `eddsa_ed25519_verify_msg`
- `eddsa_ed25519_verify_prehashed`
- `ed25519_pubkey`
- `verify_hashed_msg`
- `verify_msg`
- `hash_sha256`
- `hash_sha256_msg`
- `hash_sha512`
- `hash_sha512_msg`


#### Verification of a Message

method: `verify_msg`

###### Parameters

- `sign`: string - the signature, in base58. Can be a `Ed25519` or a `Secp256k1` signature.
- `pubkey`: string - the public key, in base58 with an optional `{header}:` as prefix. Can be a `Ed25519` or
  a `Secp256k1` public key. On a missing prefix, `ed25519:` is assumed.
- `msg`: string - the message. It will be hashed internally by the contract.

###### Returns

- `is_match`: boolean - whether the sha256 hash of the `msg` matched the `pubkey` on the `sign`.

###### Sample

```json
{
  "sign": "26gFr4xth7W9K7HPWAxq3BLsua8oTy378mC1MYFiEXHBBpeBjP8WmJEJo8XTBowetvqbRshcQEtBUdwQcAqDyP8T",
  "pubkey": "ed25519:AYWv9RAN1hpSQA4p1DLhCNnpnNXwxhfH9qeHN8B4nJ59",
  "msg": "message"
}
```

#### Verification of a Prehashed Message

method: `verify_hashed_msg`

###### Parameters

- `sign`: string - the signature, in base58. Can be a `Ed25519` or a `Secp256k1` signature.
- `pubkey`: string - the public key, in base58 with an optional `{header}:` as prefix. Can be a `Ed25519` or
  a `Secp256k1` public key. On a missing prefix, `ed25519:` is assumed.
- `msg_hash`: number[] - the message hash, in a 32-sized array of bytes, resulted from a sha256 hash of them message.

###### Returns

- `is_match`: boolean - whether the `msg_hash` matched the `pubkey` on the `sign`.

###### Sample

```json
{
  "sign": "26gFr4xth7W9K7HPWAxq3BLsua8oTy378mC1MYFiEXHBBpeBjP8WmJEJo8XTBowetvqbRshcQEtBUdwQcAqDyP8T",
  "pubkey": "ed25519:AYWv9RAN1hpSQA4p1DLhCNnpnNXwxhfH9qeHN8B4nJ59",
  "msg_hash": [171, 83, 10, 19, 228, 89, 20, 152, 43, 121, 249, 183, 227, 251, 169, 148, 207, 209, 243, 251, 34, 247, 28, 234, 26, 251, 240, 43, 70, 12, 109, 29]
}
```
