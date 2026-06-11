#![cfg(test)]
use super::*;
use soroban_sdk::testutils::{Address as _, Ledger as _};
use soroban_sdk::{Address, Bytes, BytesN, Env};

fn make_polygon_id(env: &Env, val: u8) -> BytesN<32> {
    BytesN::from_array(env, &[val; 32])
}

fn make_survey_hash(env: &Env, val: u8) -> BytesN<32> {
    BytesN::from_array(env, &[val; 32])
}

fn make_methodology_id(env: &Env, val: u8) -> BytesN<8> {
    BytesN::from_array(env, &[val; 8])
}

fn make_comment(env: &Env) -> BytesN<32> {
    BytesN::from_array(env, &[0u8; 32])
}

fn make_ipfs_cid(env: &Env) -> Bytes {
    Bytes::from_slice(env, b"QmTest123")
}

fn make_gov_id(env: &Env) -> BytesN<32> {
    BytesN::from_array(env, &[0u8; 32])
}

fn make_propose_params(
    env: &Env,
    beneficiary: &Address,
) -> ProposeParams {
    ProposeParams {
        polygon_id: make_polygon_id(env, 1),
        survey_hash: make_survey_hash(env, 2),
        methodology_id: make_methodology_id(env, 3),
        credit_qty: 500,
        beneficiary: beneficiary.clone(),
        survey_ipfs_cid: make_ipfs_cid(env),
        baseline_bsi: 100,
        current_bsi: 200,
        area_ha_contribution: 4500,
        biome: 0,
        vintage_year: 2025,
        vintage_quarter: 2,
        approval_governance_id: make_gov_id(env),
    }
}

fn setup_test(env: &Env) -> (Address, ApprovalGovContractClient<'static>) {
    let admin = Address::generate(env);
    let contract_id = env.register(ApprovalGovContract, (&admin, 3u32, 604800u64));
    let client = ApprovalGovContractClient::new(env, &contract_id);
    (admin, client)
}

#[test]
fn test_register_stakeholder() {
    let env = Env::default();
    env.mock_all_auths();
    let (_admin, client) = setup_test(&env);

    let addr = Address::generate(&env);
    client.register_stakeholder(&addr, &StakeholderRole::LeadEcologist, &10, &false);

    let stakeholder = client.get_stakeholder(&addr);
    assert_eq!(stakeholder.role, StakeholderRole::LeadEcologist);
    assert_eq!(stakeholder.weight, 10);
    assert_eq!(stakeholder.has_veto, false);
    assert_eq!(stakeholder.active, true);
    assert_eq!(client.stakeholder_count(), 1);
}

#[test]
fn test_register_all_roles() {
    let env = Env::default();
    env.mock_all_auths();
    let (_admin, client) = setup_test(&env);

    let roles = [
        StakeholderRole::LeadEcologist,
        StakeholderRole::PeerEcologist,
        StakeholderRole::LocalCommunityRep,
        StakeholderRole::IndependentAuditor,
        StakeholderRole::MethodologyExpert,
        StakeholderRole::RegulatoryObserver,
    ];

    for (i, role) in roles.iter().enumerate() {
        let addr = Address::generate(&env);
        client.register_stakeholder(&addr, role, &((i as u32 + 1) * 5), &false);
    }

    assert_eq!(client.stakeholder_count(), 6);
}

#[test]
fn test_remove_stakeholder() {
    let env = Env::default();
    env.mock_all_auths();
    let (_admin, client) = setup_test(&env);

    let addr = Address::generate(&env);
    client.register_stakeholder(&addr, &StakeholderRole::IndependentAuditor, &5, &false);
    client.remove_stakeholder(&addr);

    let stakeholder = client.get_stakeholder(&addr);
    assert_eq!(stakeholder.active, false);
}

#[test]
fn test_set_threshold() {
    let env = Env::default();
    env.mock_all_auths();
    let (_admin, client) = setup_test(&env);

    assert_eq!(client.min_threshold(), 3);

    client.set_min_threshold(&10);
    assert_eq!(client.min_threshold(), 10);
}

#[test]
fn test_set_voting_period() {
    let env = Env::default();
    env.mock_all_auths();
    let (_admin, client) = setup_test(&env);

    assert_eq!(client.voting_period(), 604800);

    client.set_voting_period(&86400);
    assert_eq!(client.voting_period(), 86400);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn test_duplicate_stakeholder_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    let (_admin, client) = setup_test(&env);

    let addr = Address::generate(&env);
    client.register_stakeholder(&addr, &StakeholderRole::LeadEcologist, &10, &false);
    client.register_stakeholder(&addr, &StakeholderRole::PeerEcologist, &5, &false);
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn test_get_nonexistent_stakeholder() {
    let env = Env::default();
    env.mock_all_auths();
    let (_admin, client) = setup_test(&env);

    let addr = Address::generate(&env);
    client.get_stakeholder(&addr);
}

#[test]
fn test_propose_creates_proposal() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1000);
    let (admin, client) = setup_test(&env);

    let beneficiary = Address::generate(&env);
    let params = make_propose_params(&env, &beneficiary);

    let pid = client.propose(&admin, &params);
    assert_eq!(pid, 1);

    let proposal = client.get_proposal(&pid);
    assert_eq!(proposal.state, ProposalState::Voting);
    assert_eq!(proposal.voting_deadline, 1000 + 604800);
    assert_eq!(proposal.credit_qty, 500);
    assert_eq!(proposal.baseline_bsi, 100);
    assert_eq!(proposal.current_bsi, 200);
}

#[test]
fn test_vote_approve() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1000);
    let (_admin, client) = setup_test(&env);

    let stakeholder = Address::generate(&env);
    client.register_stakeholder(&stakeholder, &StakeholderRole::LeadEcologist, &10, &false);

    let beneficiary = Address::generate(&env);
    let params = make_propose_params(&env, &beneficiary);

    let pid = client.propose(&stakeholder, &params);
    let comment = make_comment(&env);
    client.vote(&stakeholder, &pid, &true, &comment);

    let proposal = client.get_proposal(&pid);
    assert_eq!(proposal.weighted_total_approve, 10);
    assert_eq!(proposal.votes.len(), 1);
}

#[test]
fn test_community_veto() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1000);
    let (_admin, client) = setup_test(&env);

    let rep = Address::generate(&env);
    client.register_stakeholder(&rep, &StakeholderRole::LocalCommunityRep, &5, &true);

    let beneficiary = Address::generate(&env);
    let params = make_propose_params(&env, &beneficiary);

    let pid = client.propose(&rep, &params);
    client.veto(&rep, &pid);

    let proposal = client.get_proposal(&pid);
    assert_eq!(proposal.state, ProposalState::Rejected);
    assert_eq!(proposal.community_veto, true);
}

#[test]
fn test_threshold_met_triggers_approval() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1000);
    let (_admin, client) = setup_test(&env);

    let s1 = Address::generate(&env);
    client.register_stakeholder(&s1, &StakeholderRole::LeadEcologist, &10, &false);

    let beneficiary = Address::generate(&env);
    let params = make_propose_params(&env, &beneficiary);

    // threshold is 3, weight is 10 -> meets threshold
    let pid = client.propose(&s1, &params);
    let comment = make_comment(&env);
    client.vote(&s1, &pid, &true, &comment);

    let proposal = client.get_proposal(&pid);
    assert_eq!(proposal.state, ProposalState::Approved);
    assert_eq!(proposal.weighted_total_approve, 10);
}

#[test]
#[should_panic(expected = "Error(Contract, #8)")]
fn test_double_vote_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1000);
    let (_admin, client) = setup_test(&env);

    // Set threshold high so first vote doesn't auto-approve
    client.set_min_threshold(&100);

    let stakeholder = Address::generate(&env);
    client.register_stakeholder(&stakeholder, &StakeholderRole::LeadEcologist, &10, &false);

    let beneficiary = Address::generate(&env);
    let params = make_propose_params(&env, &beneficiary);

    let pid = client.propose(&stakeholder, &params);
    let comment = make_comment(&env);
    client.vote(&stakeholder, &pid, &true, &comment);
    client.vote(&stakeholder, &pid, &false, &comment);
}

#[test]
#[should_panic(expected = "Error(Contract, #7)")]
fn test_non_stakeholder_cannot_vote() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1000);
    let (_admin, client) = setup_test(&env);

    let random = Address::generate(&env);
    let beneficiary = Address::generate(&env);
    let params = make_propose_params(&env, &beneficiary);

    let pid = client.propose(&random, &params);
    let comment = make_comment(&env);
    client.vote(&random, &pid, &true, &comment);
}

#[test]
#[should_panic(expected = "Error(Contract, #12)")]
fn test_vote_after_deadline() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1000);
    let (_admin, client) = setup_test(&env);

    let stakeholder = Address::generate(&env);
    client.register_stakeholder(&stakeholder, &StakeholderRole::LeadEcologist, &10, &false);

    let beneficiary = Address::generate(&env);
    let params = make_propose_params(&env, &beneficiary);

    let pid = client.propose(&stakeholder, &params);

    // Advance past deadline
    env.ledger().set_timestamp(1000 + 604800 + 1);

    let comment = make_comment(&env);
    client.vote(&stakeholder, &pid, &true, &comment);
}

#[test]
#[should_panic(expected = "Error(Contract, #13)")]
fn test_veto_only_for_community_role() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1000);
    let (_admin, client) = setup_test(&env);

    let ecologist = Address::generate(&env);
    client.register_stakeholder(&ecologist, &StakeholderRole::LeadEcologist, &10, &false);

    let beneficiary = Address::generate(&env);
    let params = make_propose_params(&env, &beneficiary);

    let pid = client.propose(&ecologist, &params);
    client.veto(&ecologist, &pid);
}

#[test]
fn test_close_proposal_after_deadline_approved() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1000);
    let (admin, client) = setup_test(&env);

    let s1 = Address::generate(&env);
    let s2 = Address::generate(&env);
    // Set threshold to 30 so it won't be met during voting (auto-approval)
    client.set_min_threshold(&30);
    client.register_stakeholder(&s1, &StakeholderRole::LeadEcologist, &10, &false);
    client.register_stakeholder(&s2, &StakeholderRole::IndependentAuditor, &5, &false);

    let beneficiary = Address::generate(&env);
    let params = make_propose_params(&env, &beneficiary);

    let pid = client.propose(&s1, &params);
    let comment = make_comment(&env);
    client.vote(&s1, &pid, &true, &comment);  // approve weight=10 (below threshold of 30)
    client.vote(&s2, &pid, &true, &comment);  // approve weight=5 (total=15, still below 30)

    // Now lower the threshold and have admin close the proposal
    client.set_min_threshold(&3);
    client.close_proposal(&admin, &pid);

    let proposal = client.get_proposal(&pid);
    assert_eq!(proposal.state, ProposalState::Approved);
}

#[test]
fn test_reject_threshold_met() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1000);
    let (_admin, client) = setup_test(&env);

    let s1 = Address::generate(&env);
    client.register_stakeholder(&s1, &StakeholderRole::LeadEcologist, &10, &false);

    let beneficiary = Address::generate(&env);
    let params = make_propose_params(&env, &beneficiary);

    let pid = client.propose(&s1, &params);
    let comment = make_comment(&env);
    client.vote(&s1, &pid, &false, &comment);

    // reject weight 10 >= threshold 3 -> auto-rejected
    let proposal = client.get_proposal(&pid);
    assert_eq!(proposal.state, ProposalState::Rejected);
}

#[test]
fn test_close_proposal_rejected_after_deadline() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1000);
    let (admin, client) = setup_test(&env);

    let s1 = Address::generate(&env);
    client.register_stakeholder(&s1, &StakeholderRole::LeadEcologist, &2, &false);

    let beneficiary = Address::generate(&env);
    let params = make_propose_params(&env, &beneficiary);

    let pid = client.propose(&s1, &params);
    let comment = make_comment(&env);
    client.vote(&s1, &pid, &true, &comment);  // approve weight=2 < threshold=3

    // Advance past deadline
    env.ledger().set_timestamp(1000 + 604800 + 1);

    // close_proposal will check: threshold not met, not veto -> stays Voting -> set to Rejected
    client.close_proposal(&admin, &pid);

    let proposal = client.get_proposal(&pid);
    assert_eq!(proposal.state, ProposalState::Rejected);
}
