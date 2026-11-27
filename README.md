# Kalki Case Finance

[![CI](https://github.com/Kalpi-buss/kalki-case-financing/actions/workflows/ci.yml/badge.svg)](https://github.com/Kalpi-buss/kalki-case-financing/actions/workflows/ci.yml)

Backers finance one US court case, step by step, on the Stellar test network. This repository is the code for the Stellar Instawards sprint (1 to 30 December 2026). The full design is in [docs/DESIGN.md](docs/DESIGN.md).

**Status: starting skeleton.** Nothing here moves money. Testnet and test USDC only. Nothing built before 1 December 2026 is billed to the sprint.

## What case financing is

- **One case, many backers.** Backers put in test USDC to pay the legal costs of one US court case.
- **Step escrow.** The case budget is split into steps. The money for each step is held in a
  [Trustless Work](https://www.trustlesswork.com/) multi-release escrow (v2) on Stellar testnet.
- **Court entry approval.** A step is released only when its court entry is in the public court record,
  2 of 3 signers approve it, and an outside person (the outside signer or a named outside backup) releases it. The team can approve but never release.
- **Case funding contract.** One new Soroban contract records what each backer put in and repays backers in proportion
  from the backers' part of a recovery (a win or settlement), on terms set before the case is financed. Nothing is issued, traded or transferred.
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

Nothing built before the sprint is part of it, and no other code is copied in. The escrow WASM is already pinned by hash (see [docs/DESIGN.md](docs/DESIGN.md)). The hour log `HOURS.md` starts on 1 December 2026 and is kept daily.

## Tested before the sprint (not billed)

On 26 November 2026 a small contract funded a Trustless Work multi-release v2 escrow on Stellar testnet with a test token we issued (TUSDC), got a disputed step back less exactly 0.3% (Trustless Work's fee), and saw another step released after 2 of 3 approvals. In the order they happened on chain:

- Funding by a contract: https://stellar.expert/explorer/testnet/tx/5d98a6d9f45b5f770fdd464e9d51c566de50e81119681a8dc5290815432d8b10
- Disputed 40 TUSDC step returned as 39.88: https://stellar.expert/explorer/testnet/tx/3ab39d9889d2a9dbb0d423b8fef1ee225e8c80213fc51e911048d12d59729099
- Release after 2 of 3 approvals: https://stellar.expert/explorer/testnet/tx/0fe098d3cf499768d2f73f9b0b2853d26f846c797a5a218fe4a4ef98737b0847

Details, all transactions and saved copies of them: [spike/README.md](spike/README.md).

## What the 30-day sprint adds

1. **Step escrow on Trustless Work.** One multi-release escrow (v2) per case, one milestone per court step, on testnet, used unchanged and pinned by WASM hash. A resolver multisig and an admin multisig, each needing an outside key. Adversarial tests and a key-loss drill.
2. **Case funding contract.** Deposits of test USDC, full refunds if the target is missed, funding of the step escrow, crediting of refunds, and repayment of backers in proportion from the backers' part of a recovery. A 10,000 test USDC cap, a 24-hour wait on results, a pause that never blocks refunds or claims, a solvency check, no token and no upgrade function. 40+ tests, including adversarial ones.
3. **Court-entry approval with an outside signer.** A hand-written rules file, a court reference on each step, an open-source check tool with a signing guide, and a replay of one past, already decided US federal case from CourtListener's RECAP archive. A step is approved by 2 of 3 signers against a named court entry and released only by the outside signer or the named backup, both outside the team.
4. **Web app and test withdrawal.** Three Scaffold Stellar screens (wallet and USDC set-up, case pages to deposit and claim, a follow page), a test withdrawal of 1 to 10 test USDC through Stellar's test anchor (SEP-10, SEP-24), a TypeScript client on npm and a developer guide.

Not in this sprint: real money, mainnet, real cases, payouts to bank accounts and a security audit. Mainnet waits for outside audits and a US anchor, and no date is promised.

## Run it

Needs Rust (stable) and, for contract builds, the [Stellar CLI](https://developers.stellar.org/docs/tools/cli).

```bash
cargo test --workspace
```

## Licence

Apache-2.0. See `LICENSE`.
