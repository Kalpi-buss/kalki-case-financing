//! Local run of the same flow the testnet script runs. The fund call is made
//! with only the owner's signature mocked: the funder contract's own
//! authority must come from authorize_as_current_contract.
extern crate std;

use super::*;
use soroban_sdk::{
    testutils::{Address as _, MockAuth, MockAuthInvoke},
    map, String, Vec,
};

fn milestone(e: &Env, receiver: &Address, amount: i128) -> tw::Milestone {
    tw::Milestone {
        description: String::from_str(e, "step"),
        status: String::from_str(e, "pending"),
        evidence: String::from_str(e, ""),
        approvals: tw::MilestoneApprovals {
            target: 2,
            approval_count: 0,
            approved_by: Vec::new(e),
        },
        amount,
        dispute: tw::Dispute {
            is_disputed: false,
            reason: String::from_str(e, ""),
            resolved: false,
        },
        released: false,
        receiver: receiver.clone(),
    }
}

#[test]
fn contract_funds_escrow_and_gets_dispute_refund() {
    let e = Env::default();
    e.mock_all_auths();

    let issuer = Address::generate(&e);
    let sac = e.register_stellar_asset_contract_v2(issuer);
    let usdc = sac.address();
    let mint = token::StellarAssetClient::new(&e, &usdc);
    let tok = token::Client::new(&e, &usdc);

    let owner = Address::generate(&e);
    let admin = Address::generate(&e);
    let platform = Address::generate(&e);
    let tw_fee = Address::generate(&e);
    let receiver = Address::generate(&e);
    let a1 = Address::generate(&e);
    let a2 = Address::generate(&e);
    let a3 = Address::generate(&e);
    let provider = Address::generate(&e);
    let resolver = Address::generate(&e);

    let hash = e.deployer().upload_contract_wasm(tw::WASM);
    let escrow_id = e.register(tw::WASM, (admin.clone(), hash));
    let esc = tw::Client::new(&e, &escrow_id);

    let props = tw::Escrow {
        engagement_id: String::from_str(&e, "spike-1"),
        title: String::from_str(&e, "funder spike"),
        description: String::from_str(&e, "testnet spike"),
        platform_fee: 0,
        receiver_memo: 0,
        trustline: tw::Trustline { address: usdc.clone() },
        roles: tw::Roles {
            approvers: soroban_sdk::vec![&e, a1.clone(), a2.clone(), a3.clone()],
            service_providers: soroban_sdk::vec![&e, provider.clone()],
            platform: platform.clone(),
            release_signers: soroban_sdk::vec![&e, a1.clone(), a2.clone(), a3.clone()],
            dispute_resolvers: soroban_sdk::vec![&e, resolver.clone()],
            admin: admin.clone(),
            observers: Vec::new(&e),
        },
        milestones: soroban_sdk::vec![
            &e,
            milestone(&e, &receiver, 40_000_000),
            milestone(&e, &receiver, 60_000_000)
        ],
    };
    esc.initialize_escrow(&props);

    let spike = e.register(FunderSpike, (owner.clone(),));
    mint.mint(&spike, &100_000_000);

    // Only the owner is mocked here.
    e.set_auths(&[]);
    e.mock_auths(&[MockAuth {
        address: &owner,
        invoke: &MockAuthInvoke {
            contract: &spike,
            fn_name: "fund",
            args: (escrow_id.clone(), props.clone(), 100_000_000i128).into_val(&e),
            sub_invokes: &[],
        },
    }]);
    FunderSpike::fund_client(&e, &spike).fund(&escrow_id, &props, &100_000_000);
    assert_eq!(tok.balance(&spike), 0);
    assert_eq!(tok.balance(&escrow_id), 100_000_000);

    e.mock_all_auths();
    // Step 0 is missed: an approver disputes it, the resolver sends it back to the pool.
    esc.dispute_milestones(&a1, &soroban_sdk::vec![&e, 0u32], &String::from_str(&e, "step missed"));
    esc.resolve_dispute(
        &resolver,
        &tw_fee,
        &soroban_sdk::vec![&e, 0u32],
        &map![&e, (spike.clone(), 40_000_000i128)],
    );
    // 0.3% of 40 = 0.12
    assert_eq!(tok.balance(&spike), 40_000_000 - 120_000);
    assert_eq!(tok.balance(&tw_fee), 120_000);

    // Step 1 released the normal way after 2 of 3 approvers.
    esc.approve_milestones(&soroban_sdk::vec![&e, 1u32], &a1);
    esc.approve_milestones(&soroban_sdk::vec![&e, 1u32], &a2);
    esc.release_funds(&a3, &tw_fee, &soroban_sdk::vec![&e, 1u32]);
    assert_eq!(tok.balance(&receiver), 60_000_000 - 180_000);
}

// Helper so the test can call the contract by its generated client.
impl FunderSpike {
    fn fund_client<'a>(e: &'a Env, id: &'a Address) -> FunderSpikeClient<'a> {
        FunderSpikeClient::new(e, id)
    }
}
