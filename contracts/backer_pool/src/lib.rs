#![no_std]
//! Case funding contract: one per financed court case.
//!
//! Starting skeleton only. The sprint adds: backers deposit test USDC and their
//! balances are recorded, the contract funds the case's step escrow, and a recovery
//! is paid out to backers in a fixed order set before the case is financed.

use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct BackerPool;

#[contractimpl]
impl BackerPool {
    /// Version of the pool interface. 0 means skeleton: no deposits, no payouts.
    pub fn version(_env: Env) -> u32 {
        0
    }
}

#[cfg(test)]
mod test;
