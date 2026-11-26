#![cfg(test)]
use super::{CaseFunding, CaseFundingClient};
use soroban_sdk::Env;

#[test]
fn skeleton_reports_version_zero() {
    let env = Env::default();
    let id = env.register(CaseFunding, ());
    let client = CaseFundingClient::new(&env, &id);
    assert_eq!(client.version(), 0);
}
