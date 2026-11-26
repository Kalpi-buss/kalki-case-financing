#!/usr/bin/env bash
# Replays the spike on Stellar testnet with the Stellar CLI only (no REST API).
# Keys live in the CLI's own key store (stellar keys), never in this folder.
# Testnet only. Test token only.
set -euo pipefail
cd "$(dirname "$0")/.."
export STELLAR_NETWORK=testnet
A() { stellar keys address "$1"; }

# 1. Team test keys, funded by friendbot
for k in sk-owner sk-issuer sk-admin sk-a1 sk-a2 sk-a3 sk-provider sk-resolver sk-platform sk-twfee sk-receiver; do
  stellar keys generate "$k" --fund 2>/dev/null || true
done

# 2. Test token TUSDC (a classic asset wrapped in its Stellar Asset Contract, like Circle's USDC)
ISS=$(A sk-issuer)
for k in sk-receiver sk-twfee sk-platform; do stellar tx new change-trust --source "$k" --line "TUSDC:$ISS"; done
TOK=$(stellar contract asset deploy --asset "TUSDC:$ISS" --source sk-issuer --alias tusdc)

# 3. Escrow: Trustless Work multi-release v2 built from source (branch multi-release-develop-v2)
H=$(stellar contract upload --wasm wasm/tw_escrow_multi_release_develop_v2.wasm --source sk-owner)
ESC=$(stellar contract deploy --wasm-hash "$H" --source sk-owner -- --admin sk-admin --approved_wasm_hash "$H")

# 4. Funder spike contract
stellar contract build
SP=$(stellar contract deploy --wasm target/wasm32v1-none/release/funder_spike.wasm --source sk-owner -- --owner sk-owner)

# 5. Initialise the escrow (escrow.json: 2 steps of 40 and 60 TUSDC, each needs 2 of 3 approvers)
#    Regenerate escrow.json with your own addresses before running this on fresh keys.
stellar contract invoke --id "$ESC" --source sk-admin --send=yes -- initialize_escrow --escrow_properties-file-path testnet/escrow.json

# 6. Give the spike 100 TUSDC, then the spike funds the escrow from its own balance
stellar contract invoke --id "$TOK" --source sk-issuer --send=yes -- mint --to "$SP" --amount 1000000000
stellar contract invoke --id "$ESC" --source sk-owner -- get_escrow > testnet/stored_escrow.json
stellar contract invoke --id "$SP" --source sk-owner --send=yes -- fund --escrow "$ESC" --expected-file-path testnet/stored_escrow.json --amount 1000000000

# 7. Step 1 missed: an approver disputes it, the resolver returns it to the spike contract
stellar contract invoke --id "$ESC" --source sk-a1 --send=yes -- dispute_milestones --signer sk-a1 --milestone_indices '[0]' --reason "Step missed its court date"
stellar contract invoke --id "$ESC" --source sk-resolver --send=yes -- resolve_dispute --dispute_resolver sk-resolver \
  --trustless_work_address "$(A sk-twfee)" --milestone_indices '[0]' --distributions "{\"$SP\":\"400000000\"}"

# 8. Step 2: two approvers approve, a third release signer releases
stellar contract invoke --id "$ESC" --source sk-a1 --send=yes -- approve_milestones --milestone_indices '[1]' --approver sk-a1
stellar contract invoke --id "$ESC" --source sk-a2 --send=yes -- approve_milestones --milestone_indices '[1]' --approver sk-a2
stellar contract invoke --id "$ESC" --source sk-a3 --send=yes -- release_funds --release_signer sk-a3 --trustless_work_address "$(A sk-twfee)" --milestone_indices '[1]'

# 9. Balances
for a in "$SP" "$ESC" "$(A sk-receiver)" "$(A sk-twfee)"; do echo "$a $(stellar contract invoke --id "$TOK" --source sk-owner -- balance --id "$a")"; done
