# Case financing: design

The detail behind the Instawards SOW for Kalki Case Finance. Testnet and Circle's testnet USDC only (the asset Stellar's test anchor accepts). Nothing here moves real money.

## 1. Step escrow (Trustless Work multi-release v2)

- **Version and pin.** Built unchanged from Trustless Work's open-source code, branch `multi-release-develop-v2`, commit `3c52edde586f561b77221ef715ae1f7a41f3f090`, with the Stellar CLI. WASM hash `e04b78f5bde774b228f8590a9bac95ed469c8d99d70185cb8201a379adb77b1a` (see [spike/SPIKE.md](../spike/SPIKE.md)). Version 2 is in Beta, testnet only, and has no outside audit; Trustless Work's audited code is version 1.
- **Fee.** The escrow code deducts 0.3% on every release and every refund. On this branch the fee goes to an address passed in each call; in the sprint that is a team test account. The escrow's own platform fee is 0.
- **Approvals.** Three approvers: the outside signer and exactly two team signers. `approvals.target = 2`. Two release signers: the outside signer and the named backup. The team can approve but cannot release.
- **Outside people.** Two outside people cover three outside roles: the outside signer (Bhawna Singh, an independent practising lawyer in India) and one person who is both the named backup and the outside keyholder (named in Week 1). Neither backs the case. Each is not employed by, paid by or invested in Kalki, is not the Ambassador Chapter Lead or a chapter member, and is not related to a team member. If either cannot sign by 18 December, a replacement meeting the same test is named and its key published.
- **Refund path.** A milestone has no deadline field, so each step's deadline is in the rules file. After it, a signer opens a dispute and the dispute resolver returns the step to the case funding contract, less the fee. The resolver is chosen by the caller, so the outside keyholder and one team key could send an unreleased step anywhere. The outside keyholder confirms in writing they sign only refunds to the case funding contract, and every dispute and resolution is in the public evidence table. The resolver resolves one step per call. The same resolver can call `withdraw_remaining_funds` once every step is released or resolved; the same written confirmation covers it.
- **Multisig accounts.** The dispute resolver and the admin are two separate classic accounts. On each: master key weight 0, the outside keyholder's key weight 2, each of exactly two team keys weight 1, every threshold 3. So the outside key plus any team key passes; two team keys, or the outside key alone, fail. After funding, the admin can only add milestones and extend storage life; roles, amounts and existing milestones are fixed. At funding the admin extends the escrow's storage past the case end date. The admin multisig also holds the case funding contract's admin and pauser roles.
- **Evidence field.** Before approval it holds the court, case number, entry number, entry date and a SHA-256 hash of the entry: the entry's text as shown on the court record, with whitespace normalised, plus the SHA-256 of the RECAP document when one exists. Trustless Work lets the field change after approval, so the release signer checks it again at release.
- **Signing dates.** Outside people confirm availability by 4 December; a replacement is named by 18 December if needed; replay signing is 20 to 23 December, with the backup signing by 23 December at the latest.

## 2. Case funding contract

- **Funding the escrow.** One call, sent by a signer after the target is reached, authorises the token transfer inside the call (`authorize_as_current_contract`, see [spike/src/lib.rs](../spike/src/lib.rs)) and passes Trustless Work's `expected_escrow`, so it fails if the escrow's roles, receivers or amounts changed.
- **Listing terms**, fixed once: target, deadline to reach it, end date, approved backer addresses, the backers' part of any recovery, the platform fee (0 in the sprint), the signers and the named backup, and a cap of 10,000 test USDC.
- **Result.** Won, Settled or Lost needs 2 of 3 approvals, one from the outside signer (or the backup if that key is lost), each checked with `require_auth`. It waits 24 hours; the outside signer or the backup can cancel it in that time.
- **Repayment.** The law firm pays in only the backers' part of a recovery. Backers are repaid first, in proportion. In the replay the recovery paid in is no more than the deposits, so no test wallet receives more than it put in; larger payouts exist only in unit tests. On a loss or a missed end date, backers claim back what is left.
- **Crediting refunds.** Deposits and recovery pay-ins enter only through the contract's own functions, which add to a running total. When a refund arrives, a signer records it for its step: the contract reads the escrow (`get_escrow`), checks the step is resolved and not released, and credits the smaller of the step amount less 0.3% and the balance rise it has not yet counted. A transfer that matches no resolved step is never credited, and no step is recorded twice. Money from `withdraw_remaining_funds` is recorded the same way, once per case, and is capped at the escrow's unreleased, unresolved balance read from `get_escrow`.
- **Solvency.** A read-only call returns what the contract holds and owes. Money still in the escrow is shown on its own line.
- **Storage and pause.** Each deposit, record and claim extends the storage life of the entries it touches; a public extend call renews the case's storage up to the network maximum. The pause lapses 14 days after it is set and never blocks refunds or claims.
- **No upgrade function.** A bug in the claim math cannot be fixed after deployment; the cap limits the loss.

## 3. Known risks (STRIDE list)

1. The escrow is Beta and not externally audited.
2. The refund recipient is enforced by a written promise, not by the escrow.
3. One outside person holds both the backup and keyholder roles in the sprint; a third outside person is required before mainnet.
4. If the outside keyholder's key is lost after funding, unreleased steps cannot be refunded.
5. In the replay all three signers read one archived CourtListener source.
6. No upgrade function.
7. Testnet resets clear history; every cited transaction is saved in the repository as it is made, and the scripts redeploy within a day.
8. A recovery is paid in only if the law firm sends it; a real case needs a signed funding agreement off chain before any real money.

Each threat gets one test, listed here as it is written during the sprint.
