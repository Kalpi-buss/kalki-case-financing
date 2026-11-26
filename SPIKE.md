# Spike: a contract as funder of a Trustless Work multi-release v2 escrow

Work done before the sprint, on 26 Nov 2026. Not billed. Stellar testnet only, test token only.

## Question
Can a Soroban contract (the future backer pool) be the funder of a Trustless Work multi-release v2 escrow, and can a dispute resolution pay a missed step back to that contract's address?

## Answer
**Yes to both, on public testnet transactions.**

- The spike contract funded the escrow with 100 TUSDC from its own balance. The escrow's `tw_fund` event records the contract address as `funder`.
- Step 1 (40 TUSDC) was disputed and resolved with the whole step paid to the spike contract. The contract's balance went from 0 to **39.88 TUSDC (398,800,000 base units)**: exactly 40 less 0.3% (0.12 to the Trustless Work fee address). Platform fee was set to 0.
- Step 2 (60 TUSDC) was released the normal way after 2 of 3 approvers approved; the receiver got 59.82 (60 less 0.3%). A release tried before the approvals was refused (`ReleaseError::EscrowNotCompleted`, #7).
- Everything was done with the **Stellar CLI alone**, no REST API and no API key.

## How the contract funds the escrow
`fund_escrow(signer, expected_escrow, amount)` checks the stored escrow equals `expected_escrow`, calls `signer.require_auth()`, then calls `token.transfer(signer, escrow, amount)`.

- `signer.require_auth()` passes because the spike contract is the direct caller of the escrow.
- `token.transfer(spike, ...)` is one call deeper (escrow calls token), so the contract must authorise it itself, before the call, with `env.authorize_as_current_contract` naming the token, `transfer`, and the exact `(spike, escrow, amount)` arguments. See `spike/src/lib.rs`. Checked: with that call removed, the local test fails with `Error(Auth, InvalidAction)`.
- The owner key must sign the spike's `fund` call (`owner.require_auth()`), so nobody else can move the pool's money.

The refund needs nothing special: the resolver's distribution map names the contract address and the token transfers to it. Contract addresses hold Stellar Asset Contract balances without a trustline.

## Route chosen
Self-deploy from source. The Trustless Work REST API needs an API key only the owner can create, and it builds transactions for a wallet signer; a contract funder calls the escrow from its own code, so the API is not needed for funding.

- Source: `github.com/Trustless-Work/trustlesswork-smart-contract-stellar`
- Built from branch **`multi-release-develop-v2`**, commit `3c52edde586f561b77221ef715ae1f7a41f3f090` (24 Nov 2026), with `stellar contract build` (Stellar CLI 30.0.1, Rust 1.100.0, soroban-sdk 29.0.1). Unmodified.
- **Escrow WASM hash: `e04b78f5bde774b228f8590a9bac95ed469c8d99d70185cb8201a379adb77b1a`** (also the SHA-256 of `spike/wasm/tw_escrow_multi_release_develop_v2.wasm`).

### Why the develop branch, not `multi-release-main-v2`
`multi-release-main-v2` (commit `d35c03a`) hardcodes the mainnet fee address `GBWWSOATPLIC72ZBOIM7WJCT7VCAHNWW4QUBZ2H4FORMCCIUM5ZVKSZN` in `modules/fee/calculator.rs`. That account does not exist on testnet (Horizon 404), so any release or resolution there would fail on the 0.3% fee transfer of a classic-asset token like USDC. The develop branch has the same logic but takes `trustless_work_address` as an argument on `release_funds`, `approve_and_release_milestones`, `resolve_dispute` and `withdraw_remaining_funds`. We passed a team test account. On mainnet the main branch's hardcoded address applies and that argument does not exist.

## Token
`TUSDC`, a classic asset we issued (issuer `GDAE2BENO3QDQZHJUSI6OZM2TE3VPF422RJOFXSIJ26F6OOO4J4FA5KG`), wrapped in its Stellar Asset Contract `CAPXXPGX5C63TEAVTGFOJPQ6ZTU2GEPFWKMNBNKXUVDBDNKAXDSW6NGN`, 7 decimals. Circle's testnet USDC is the same kind of token (classic asset plus Stellar Asset Contract), so the auth and transfer path is identical. We did not use Circle's testnet USDC because getting it needs Circle's web faucet; it can be swapped in by changing `trustline.address`.

## Contracts on testnet
| What | ID |
|---|---|
| Trustless Work escrow (from source) | [`CCFN7SLXWITEMK7VBY2DIB6UHMEOJFEOCGT42SLFR4L46NRI5PDAKWSH`](https://stellar.expert/explorer/testnet/contract/CCFN7SLXWITEMK7VBY2DIB6UHMEOJFEOCGT42SLFR4L46NRI5PDAKWSH) |
| Funder spike contract | [`CCFMJWVEDTQNHS3GJUZ4LCO77BROBHNKWJ3L7XZPBKRTOJWPUADFQZLR`](https://stellar.expert/explorer/testnet/contract/CCFMJWVEDTQNHS3GJUZ4LCO77BROBHNKWJ3L7XZPBKRTOJWPUADFQZLR) |
| TUSDC token contract | [`CAPXXPGX5C63TEAVTGFOJPQ6ZTU2GEPFWKMNBNKXUVDBDNKAXDSW6NGN`](https://stellar.expert/explorer/testnet/contract/CAPXXPGX5C63TEAVTGFOJPQ6ZTU2GEPFWKMNBNKXUVDBDNKAXDSW6NGN) |

## Roles (team test keys, public addresses)
| Role | Address |
|---|---|
| Spike owner / deployer | `GDATBOVJ7IWHQ7TJHAPCYSRTI65ASVUIQNP3BRWUKJ7EASSRMTSQCXRU` |
| Escrow admin (separate) | `GAXDLP4OHNKBACOTZ55UPN2NGZOMCR4JT6WJDMVXJXZIHKBIITRS3GJH` |
| Approvers and release signers (3) | `GAB7VO3JO5MMATURNVJSUDS74SL65Q4KBZPDP7UQO6PRUDA7BUV4SBW6`, `GALYX4EJN5SHS3AIQI34IPMA4MJ74NL4A2LBGL6ZVW3ZT7J4HWGA5RM4`, `GDPGF7MJAQQOPYQPRQC2BD4OFV7O4TJYXXP6US6BVXXDG6HD2MGQU4B4` |
| Service provider | `GB7WMUVGSDTJNFMMTNW2X5DAPGE5BXNFEJ35ZTCBAV3XUNGKQF56E54Z` |
| Dispute resolver | `GBHCQWEOR7QYVWOKZLUMFCIJVT2IF7YHKZWS2EAWBCY7U5BCXRIUBGOW` |
| Platform (fee 0) | `GCXCAB27FWQDLUY5HMHN4TVG5WK66M42ZZBND2R2ZGPZ4XERVEWY6PS5` |
| Trustless Work fee (stand-in) | `GDQ6JFDOEVQUEERMWRF5XVPBEFWKHSWPNENTFLOKKRHDYELYG3LIRILH` |
| Step receiver | `GAP7IR5SONG4FEPVAX5OQOZ3AMQ5DOWDPJILR7Z4ORXQ7WXIQBCI6VAC` |

Each step needs 2 of 3 approvals (`approvals.target = 2`). Secret keys stay in the local `stellar keys` store and are not in this repository.

## Transactions (stellar.expert, testnet)
| # | Step | Transaction |
|---|---|---|
| 1 | Trustline: receiver | [541f60c3…](https://stellar.expert/explorer/testnet/tx/541f60c32d72261ccfd16f40b6befbf037e3609895943d450dccf4839f970c83) |
| 2 | Trustline: fee account | [0a94c83e…](https://stellar.expert/explorer/testnet/tx/0a94c83e249f92865cdfab732e8e4ecc01513d44ef043b749ea3cd23e1e29d04) |
| 3 | Trustline: platform | [99fa3ede…](https://stellar.expert/explorer/testnet/tx/99fa3ede0ebcda3f3f8fe7b1bf4447cfd4b904cc5db360cda84d5829bd84c6a7) |
| 4 | Deploy TUSDC token contract | [e5b8b586…](https://stellar.expert/explorer/testnet/tx/e5b8b586633256f86f26ba9c60cd0d0acbae420848a9ad98d814425357349868) |
| 5 | Upload escrow WASM | [b898082d…](https://stellar.expert/explorer/testnet/tx/b898082d4d686e6e4e1aab094315be4a2de7ff5d942826a1bb958aaa6806fa15) |
| 6 | Deploy escrow | [bfc1c6e4…](https://stellar.expert/explorer/testnet/tx/bfc1c6e4f68f2474a59b8ea435bb60aa086924d3bac932a6fbfaeecfb48658ed) |
| 7 | Upload spike WASM | [17b268a5…](https://stellar.expert/explorer/testnet/tx/17b268a507ad5f641d24fcd4248a820307c2797eb63a88470d886b84654b33a9) |
| 8 | Deploy spike | [24ea92fe…](https://stellar.expert/explorer/testnet/tx/24ea92fe9cba5924f4c2bc9930f17a7338b299957e6afab8baa95dd3be5aa694) |
| 9 | `initialize_escrow` (admin) | [c201e71e…](https://stellar.expert/explorer/testnet/tx/c201e71ed5c7b8ff911ef8afd56b19319326577f03abca7aa04e200e2d22a7c9) |
| 10 | Mint 100 TUSDC to the spike | [8c138d03…](https://stellar.expert/explorer/testnet/tx/8c138d03203ea692d08fa73a677879df02f4481db256c47b506464096bba0d84) |
| 11 | **Spike funds escrow (100 TUSDC)** | [5d98a6d9…](https://stellar.expert/explorer/testnet/tx/5d98a6d9f45b5f770fdd464e9d51c566de50e81119681a8dc5290815432d8b10) |
| 12 | Dispute step 1 (approver) | [8f41f32a…](https://stellar.expert/explorer/testnet/tx/8f41f32aca741f3d897cab2c4d16b1a9647f7dc04f5a1a5cff26907c38820daa) |
| 13 | **Resolve step 1 to the spike (39.88 back)** | [3ab39d98…](https://stellar.expert/explorer/testnet/tx/3ab39d9889d2a9dbb0d423b8fef1ee225e8c80213fc51e911048d12d59729099) |
| 14 | Approve step 2 (approver 1) | [6ca98c8d…](https://stellar.expert/explorer/testnet/tx/6ca98c8d17e35c5c3b6b635c620b097a143e8fd674cbd9a9120fb144bd8cd5a8) |
| 15 | Approve step 2 (approver 2) | [f51c0e02…](https://stellar.expert/explorer/testnet/tx/f51c0e02d514a1227feb6c89d1f6c65eddfe6e41bab50f2c5a03cded472032c3) |
| 16 | Release step 2 (signer 3) | [0fe098d3…](https://stellar.expert/explorer/testnet/tx/0fe098d3cf499768d2f73f9b0b2853d26f846c797a5a218fe4a4ef98737b0847) |

## Balances after the run (TUSDC base units, 7 decimals)
| Address | Before | After |
|---|---|---|
| Spike contract | 1,000,000,000 (after mint) | **398,800,000** (39.88) |
| Escrow | 0 | 0 |
| Receiver | 0 | 598,200,000 (59.82) |
| Fee account | 0 | 3,000,000 (0.30 = 0.12 + 0.18) |

## Stellar CLI only?
**Yes**, for an escrow deployed from Trustless Work's source. Commands that worked (full list in `spike/testnet/run.sh`):

```
stellar contract build                                   # in the Trustless Work repo, branch multi-release-develop-v2
stellar contract upload --wasm escrow.wasm --source <deployer> --network testnet
stellar contract deploy --wasm-hash <hash> --source <deployer> --network testnet -- --admin <admin> --approved_wasm_hash <hash>
stellar contract invoke --id <escrow> --source <admin> --network testnet --send=yes -- initialize_escrow --escrow_properties-file-path escrow.json
stellar contract invoke --id <spike> --source <owner> --network testnet --send=yes -- fund --escrow <escrow> --expected-file-path stored_escrow.json --amount 1000000000
stellar contract invoke --id <escrow> --source <approver> --network testnet --send=yes -- dispute_milestones --signer <approver> --milestone_indices '[0]' --reason "..."
stellar contract invoke --id <escrow> --source <resolver> --network testnet --send=yes -- resolve_dispute --dispute_resolver <resolver> --trustless_work_address <fee account> --milestone_indices '[0]' --distributions '{"<spike>":"400000000"}'
stellar contract invoke --id <escrow> --source <approver> --network testnet --send=yes -- approve_milestones --milestone_indices '[1]' --approver <approver>
stellar contract invoke --id <escrow> --source <signer> --network testnet --send=yes -- release_funds --release_signer <signer> --trustless_work_address <fee account> --milestone_indices '[1]'
```

Not tested: creating the escrow through Trustless Work's own deployed v2 factory or REST API (needs an API key from the owner), and mainnet.

## What this means for the SOW wording
1. "The escrow is set up with the Stellar CLI" holds, but say exactly what: the escrow contract is built from Trustless Work's open-source v2 code and deployed and run with the Stellar CLI; the REST API is optional.
2. Name the branch. On testnet only `multi-release-develop-v2` works with a classic-asset token like USDC, because `multi-release-main-v2` pays its fee to a mainnet-only address. On mainnet the main branch applies, and its call signatures differ (no `trustless_work_address` argument), so the backer pool's calls must be built per network.
3. The backer pool must use `authorize_as_current_contract` for the token transfer inside `fund_escrow`. Worth one line in the technical plan, since it is the reason the "riskiest call" works.
4. The refund to the pool is the step amount less 0.3% (plus the platform fee, if one is set). Confirmed to the base unit.
5. Still beta and not externally audited (as the SOW already says). On the develop branch the resolver or release signer chooses the fee address, which is a testnet convenience only.

## Files
- `spike/src/lib.rs`: the funder contract (owner-gated `fund`, `balance`).
- `spike/src/test.rs`: the same flow run locally against the real escrow WASM (`cargo test`), with only the owner's signature mocked on `fund`.
- `spike/wasm/tw_escrow_multi_release_develop_v2.wasm`: the escrow build used.
- `spike/testnet/`: `run.sh` (the CLI run), `escrow.json` (escrow properties), `stored_escrow.json` and `final_escrow.json` (escrow state before funding and at the end).
