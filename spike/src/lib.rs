//! Funder spike: the smallest Soroban contract that funds a Trustless Work
//! multi-release v2 escrow with its own tokens, and can receive a dispute
//! refund back. Testnet only. Not the backer pool.
#![no_std]

use soroban_sdk::{
    auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation},
    contract, contractimpl, contracttype, token, vec, Address, Env, IntoVal, Symbol,
};

mod tw {
    soroban_sdk::contractimport!(file = "wasm/tw_escrow_multi_release_develop_v2.wasm");
}

#[contracttype]
enum Key {
    Owner,
}

#[contract]
pub struct FunderSpike;

#[contractimpl]
impl FunderSpike {
    pub fn __constructor(e: Env, owner: Address) {
        e.storage().instance().set(&Key::Owner, &owner);
    }

    /// Fund `escrow` with `amount` of the escrow's token, paid from this
    /// contract's own balance. `expected` is the escrow the owner agreed to;
    /// the escrow refuses the call if its stored state differs.
    pub fn fund(e: Env, escrow: Address, expected: tw::Escrow, amount: i128) {
        let owner: Address = e.storage().instance().get(&Key::Owner).unwrap();
        owner.require_auth();

        let me = e.current_contract_address();
        // fund_escrow calls signer.require_auth() (satisfied: this contract is
        // the direct invoker) and then token.transfer(signer, escrow, amount),
        // which is one call deeper. A contract authorises a deeper call on its
        // own behalf with authorize_as_current_contract.
        e.authorize_as_current_contract(vec![
            &e,
            InvokerContractAuthEntry::Contract(SubContractInvocation {
                context: ContractContext {
                    contract: expected.trustline.address.clone(),
                    fn_name: Symbol::new(&e, "transfer"),
                    args: (me.clone(), escrow.clone(), amount).into_val(&e),
                },
                sub_invocations: vec![&e],
            }),
        ]);
        tw::Client::new(&e, &escrow).fund_escrow(&me, &expected, &amount);
    }

    /// This contract's balance of `token`.
    pub fn balance(e: Env, token: Address) -> i128 {
        token::Client::new(&e, &token).balance(&e.current_contract_address())
    }
}

#[cfg(test)]
mod test;
