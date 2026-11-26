#![no_std]
//! Backer pool: one pool per financed court case.
//!
//! Starting skeleton only. The sprint adds: backers deposit test USDC and receive
//! shares, the pool funds the case's step escrow, and a recovery is paid out to
//! share holders in a fixed order set before the case is financed.

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
