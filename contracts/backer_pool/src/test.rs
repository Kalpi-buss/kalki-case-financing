#![cfg(test)]
use super::{BackerPool, BackerPoolClient};
use soroban_sdk::Env;

#[test]
fn skeleton_reports_version_zero() {
    let env = Env::default();
    let id = env.register(BackerPool, ());
    let client = BackerPoolClient::new(&env, &id);
    assert_eq!(client.version(), 0);
}
