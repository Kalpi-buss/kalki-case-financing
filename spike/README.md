# Funder test (26 November 2026, done before the sprint, not billed)

A small Soroban contract (`src/`) funded a Trustless Work multi-release v2 escrow on Stellar testnet and received a disputed step back. Test token: TUSDC, issued for this test.

| What happened | Transaction |
|---|---|
| The contract funds the escrow with 100 TUSDC | [5d98a6d9f45b…](https://stellar.expert/explorer/testnet/tx/5d98a6d9f45b5f770fdd464e9d51c566de50e81119681a8dc5290815432d8b10) |
| Step 2, approval 1 of 2 | [6ca98c8d17e3…](https://stellar.expert/explorer/testnet/tx/6ca98c8d17e35c5c3b6b635c620b097a143e8fd674cbd9a9120fb144bd8cd5a8) |
| Step 2, approval 2 of 2 | [f51c0e02d514…](https://stellar.expert/explorer/testnet/tx/f51c0e02d514a1227feb6c89d1f6c65eddfe6e41bab50f2c5a03cded472032c3) |
| Step 2 released after the two approvals | [0fe098d3cf49…](https://stellar.expert/explorer/testnet/tx/0fe098d3cf499768d2f73f9b0b2853d26f846c797a5a218fe4a4ef98737b0847) |
| Disputed step 1 (40 TUSDC) comes back to the contract as 39.88, exactly 0.3% less | [3ab39d9889d2…](https://stellar.expert/explorer/testnet/tx/3ab39d9889d2a9dbb0d423b8fef1ee225e8c80213fc51e911048d12d59729099) |

- Full write-up, all 16 transactions, contract IDs and the escrow's WASM hash: [SPIKE.md](SPIKE.md).
- Stellar testnet is reset from time to time, which clears its history. A saved copy of every transaction (hash, ledger, envelope, result and result meta) is in [testnet/transactions/](testnet/transactions/), with an index in `index.json`.
- `src/`: the funder contract and its local test (`cargo test` in this folder, against the real escrow WASM in `wasm/`).
- `wasm/tw_escrow_multi_release_develop_v2.wasm`: Trustless Work's own escrow, built unmodified from their open-source code (branch `multi-release-develop-v2`, commit `3c52edde586f561b77221ef715ae1f7a41f3f090`) with `stellar contract build`. Its SHA-256, `e04b78f5bde774b228f8590a9bac95ed469c8d99d70185cb8201a379adb77b1a`, is the WASM hash deployed on testnet. It is kept so the test runs against the exact escrow used.
- `testnet/run.sh`: the Stellar CLI commands used. Keys are local aliases only; no key is in this repository.
