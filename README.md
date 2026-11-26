# Kalki case financing

[![CI](https://github.com/Kalpi-buss/kalki-case-financing/actions/workflows/ci.yml/badge.svg)](https://github.com/Kalpi-buss/kalki-case-financing/actions/workflows/ci.yml)

Backers finance one US court case, step by step, on the Stellar test network.

**Status: starting skeleton.** Nothing here moves money. Testnet and test USDC only. Nothing built before 1 December 2026 is billed to the sprint.

## What case financing is

- **One case, many backers.** Backers put in test USDC to pay the legal costs of one US court case.
- **Step escrow.** The case budget is split into steps. The money for each step is held in a
  [Trustless Work](https://www.trustlesswork.com/) multi-release escrow (v2) on Stellar testnet.
- **Court entry approval.** A step is released only when its court entry is in the public court record
  and 2 of 3 signers approve it. One of the three signers is always outside the team.
- **Case funding contract.** One new Soroban contract records what each backer put in and pays a recovery
  (a win or settlement) in a fixed order, set before the case is financed. Nothing is issued, traded or transferred.
- **Testnet only.** Test USDC on Stellar testnet. No real money, no mainnet deployment.

## What exists today

A starting skeleton in the Scaffold Stellar layout:

| Path | What it holds |
|---|---|
| `contracts/case_funding` | The case funding contract: an empty interface (`version()` returns 0) and one test |
| `environments.toml` | Local and testnet network settings. No mainnet |
| `packages/client` | Placeholder: the TypeScript client is built and published to npm during the sprint |
| `src/` | Empty: the web app goes here |
| `spike/` | The escrow funding test, done before the sprint and not billed. See [spike/README.md](spike/README.md) |
| `.github/workflows/ci.yml` | Format check and `cargo test` on every push and pull request |

The case funding contract is a stub. Its one test passes in CI on every push (format check and `cargo test --workspace`, soroban-sdk 29.0.1).

Nothing built before the sprint is part of it, and no other code is copied in. The sprint's pinned escrow WASM hash and the hour log `HOURS.md` are added on 1 December 2026.

## Tested before the sprint (not billed)

On 26 November 2026 a small contract funded a Trustless Work multi-release v2 escrow on Stellar testnet and got a disputed step back, less exactly 0.3%:

- Funding by a contract: https://stellar.expert/explorer/testnet/tx/5d98a6d9f45b5f770fdd464e9d51c566de50e81119681a8dc5290815432d8b10
- Release after 2 of 3 approvals: https://stellar.expert/explorer/testnet/tx/0fe098d3cf499768d2f73f9b0b2853d26f846c797a5a218fe4a4ef98737b0847
- Disputed 40 TUSDC step returned as 39.88: https://stellar.expert/explorer/testnet/tx/3ab39d9889d2a9dbb0d423b8fef1ee225e8c80213fc51e911048d12d59729099

Details, all transactions and saved copies of them: [spike/README.md](spike/README.md).

## What the 30-day sprint adds

1. **Step escrow on Trustless Work.** One multi-release escrow (v2) per case, one release per step, on testnet.
2. **Case funding contract.** Deposits of test USDC, each backer's balance, funding of the step escrow, and payout of a recovery in a fixed order. Unit tests for every path.
3. **Court-entry approval with an outside signer.** A step is approved by 2 of 3 signers against a named court entry; one signer is always outside the team.
4. **Web app and test withdrawal.** A backer's view of the case, its steps and their approvals, and a test withdrawal through Stellar's test anchor.

## Run it

Needs Rust (stable) and, for contract builds, the [Stellar CLI](https://developers.stellar.org/docs/tools/cli).

```bash
cargo test --workspace
```

## Licence

Apache-2.0. See `LICENSE`.
