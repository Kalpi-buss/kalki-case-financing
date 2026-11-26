Short testnet experiments, kept apart from the product code. Not part of the case funding contract.

## Funder test (26 November 2026, done before the sprint, not billed)

A small Soroban contract funded a Trustless Work multi-release v2 escrow on Stellar testnet and received a disputed step back. The three transactions that matter:

1. The contract funds the escrow with 100 test tokens: [https://stellar.expert/explorer/testnet/tx/5d98a6d9f45b5f770fdd464e9d51c566de50e81119681a8dc5290815432d8b10](https://stellar.expert/explorer/testnet/tx/5d98a6d9f45b5f770fdd464e9d51c566de50e81119681a8dc5290815432d8b10)
2. A step is released after 2 of 3 approvals: [https://stellar.expert/explorer/testnet/tx/0fe098d3cf499768d2f73f9b0b2853d26f846c797a5a218fe4a4ef98737b0847](https://stellar.expert/explorer/testnet/tx/0fe098d3cf499768d2f73f9b0b2853d26f846c797a5a218fe4a4ef98737b0847)
3. A disputed 40-token step comes back to the contract as 39.88, exactly 0.3% less: [https://stellar.expert/explorer/testnet/tx/3ab39d9889d2a9dbb0d423b8fef1ee225e8c80213fc51e911048d12d59729099](https://stellar.expert/explorer/testnet/tx/3ab39d9889d2a9dbb0d423b8fef1ee225e8c80213fc51e911048d12d59729099)

Every transaction, the contract IDs and the escrow's WASM hash are in [../SPIKE.md](../SPIKE.md). Run the local test with `cargo test` in this folder.
