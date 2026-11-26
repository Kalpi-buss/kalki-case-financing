#![no_std]
//! Case funding contract: one per financed court case.
//!
//! Starting skeleton only. The sprint adds: backers deposit test USDC and their
//! balances are recorded, the contract funds the case's step escrow, and a recovery
//! is paid out to backers in proportion, on terms set before the case is financed.

use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct CaseFunding;

#[contractimpl]
impl CaseFunding {
    /// Version of the contract interface. 0 means skeleton: no deposits, no payouts.
    pub fn version(_env: Env) -> u32 {
        0
    }
}

#[cfg(test)]
mod test;
