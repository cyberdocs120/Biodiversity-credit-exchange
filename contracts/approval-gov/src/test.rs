#![cfg(test)]
use super::*;
use soroban_sdk::testutils::{Address as _, Ledger as _};
use soroban_sdk::{Address, BytesN, Env};

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

    let polygon_id = BytesN::from_array(&env, &[1u8; 32]);
    let survey_hash = BytesN::from_array(&env, &[2u8; 32]);
    let methodology_id = BytesN::from_array(&env, &[3u8; 8]);
    let beneficiary = Address::generate(&env);

    let pid = client.propose(&admin, &polygon_id, &survey_hash, &methodology_id, &500, &beneficiary);
    assert_eq!(pid, 1);

    let proposal = client.get_proposal(&pid);
    assert_eq!(proposal.state, ProposalState::Voting);
    assert_eq!(proposal.voting_deadline, 1000 + 604800);
    assert_eq!(proposal.credit_qty, 500);
}

#[test]
fn test_vote_approve() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1000);
    let (_admin, client) = setup_test(&env);

    let stakeholder = Address::generate(&env);
    client.register_stakeholder(&stakeholder, &StakeholderRole::LeadEcologist, &10, &false);

    let polygon_id = BytesN::from_array(&env, &[1u8; 32]);
    let survey_hash = BytesN::from_array(&env, &[2u8; 32]);
    let methodology_id = BytesN::from_array(&env, &[3u8; 8]);
    let beneficiary = Address::generate(&env);

    let pid = client.propose(&stakeholder, &polygon_id, &survey_hash, &methodology_id, &500, &beneficiary);
    let comment = BytesN::from_array(&env, &[0u8; 32]);
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

    let polygon_id = BytesN::from_array(&env, &[1u8; 32]);
    let survey_hash = BytesN::from_array(&env, &[2u8; 32]);
    let methodology_id = BytesN::from_array(&env, &[3u8; 8]);
    let beneficiary = Address::generate(&env);

    let pid = client.propose(&rep, &polygon_id, &survey_hash, &methodology_id, &500, &beneficiary);
    client.veto(&rep, &pid);

    let proposal = client.get_proposal(&pid);
    assert_eq!(proposal.state, ProposalState::Rejected);
    assert_eq!(proposal.community_veto, true);
}

#[test]
#[should_panic(expected = "Error(Contract, #8)")]
fn test_double_vote_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1000);
    let (_admin, client) = setup_test(&env);

    let stakeholder = Address::generate(&env);
    client.register_stakeholder(&stakeholder, &StakeholderRole::LeadEcologist, &10, &false);

    let polygon_id = BytesN::from_array(&env, &[1u8; 32]);
    let survey_hash = BytesN::from_array(&env, &[2u8; 32]);
    let methodology_id = BytesN::from_array(&env, &[3u8; 8]);
    let beneficiary = Address::generate(&env);

    let pid = client.propose(&stakeholder, &polygon_id, &survey_hash, &methodology_id, &500, &beneficiary);
    let comment = BytesN::from_array(&env, &[0u8; 32]);
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
    let polygon_id = BytesN::from_array(&env, &[1u8; 32]);
    let survey_hash = BytesN::from_array(&env, &[2u8; 32]);
    let methodology_id = BytesN::from_array(&env, &[3u8; 8]);
    let beneficiary = Address::generate(&env);

    let pid = client.propose(&random, &polygon_id, &survey_hash, &methodology_id, &500, &beneficiary);
    let comment = BytesN::from_array(&env, &[0u8; 32]);
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

    let polygon_id = BytesN::from_array(&env, &[1u8; 32]);
    let survey_hash = BytesN::from_array(&env, &[2u8; 32]);
    let methodology_id = BytesN::from_array(&env, &[3u8; 8]);
    let beneficiary = Address::generate(&env);

    let pid = client.propose(&stakeholder, &polygon_id, &survey_hash, &methodology_id, &500, &beneficiary);

    // Advance past deadline
    env.ledger().set_timestamp(1000 + 604800 + 1);

    let comment = BytesN::from_array(&env, &[0u8; 32]);
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

    let polygon_id = BytesN::from_array(&env, &[1u8; 32]);
    let survey_hash = BytesN::from_array(&env, &[2u8; 32]);
    let methodology_id = BytesN::from_array(&env, &[3u8; 8]);
    let beneficiary = Address::generate(&env);

    let pid = client.propose(&ecologist, &polygon_id, &survey_hash, &methodology_id, &500, &beneficiary);
    client.veto(&ecologist, &pid);
}

#[test]
fn test_close_proposal_approved() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1000);
    let (_admin, client) = setup_test(&env);

    let s1 = Address::generate(&env);
    let s2 = Address::generate(&env);
    client.register_stakeholder(&s1, &StakeholderRole::LeadEcologist, &10, &false);
    client.register_stakeholder(&s2, &StakeholderRole::IndependentAuditor, &5, &false);

    let polygon_id = BytesN::from_array(&env, &[1u8; 32]);
    let survey_hash = BytesN::from_array(&env, &[2u8; 32]);
    let methodology_id = BytesN::from_array(&env, &[3u8; 8]);
    let beneficiary = Address::generate(&env);

    let pid = client.propose(&s1, &polygon_id, &survey_hash, &methodology_id, &500, &beneficiary);
    let comment = BytesN::from_array(&env, &[0u8; 32]);
    client.vote(&s1, &pid, &true, &comment);  // approve weight=10
    client.vote(&s2, &pid, &true, &comment);  // approve weight=5

    // threshold is 3, approve total is 15 >= 3
    env.ledger().set_timestamp(1000 + 604800 + 1);
    client.close_proposal(&_admin, &pid);

    let proposal = client.get_proposal(&pid);
    assert_eq!(proposal.state, ProposalState::Approved);
}

#[test]
fn test_close_proposal_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().set_timestamp(1000);
    let (_admin, client) = setup_test(&env);

    let s1 = Address::generate(&env);
    client.register_stakeholder(&s1, &StakeholderRole::LeadEcologist, &10, &false);

    let polygon_id = BytesN::from_array(&env, &[1u8; 32]);
    let survey_hash = BytesN::from_array(&env, &[2u8; 32]);
    let methodology_id = BytesN::from_array(&env, &[3u8; 8]);
    let beneficiary = Address::generate(&env);

    let pid = client.propose(&s1, &polygon_id, &survey_hash, &methodology_id, &500, &beneficiary);
    let comment = BytesN::from_array(&env, &[0u8; 32]);
    client.vote(&s1, &pid, &false, &comment);  // reject weight=10 >= threshold 3

    env.ledger().set_timestamp(1000 + 604800 + 1);
    client.close_proposal(&_admin, &pid);

    let proposal = client.get_proposal(&pid);
    assert_eq!(proposal.state, ProposalState::Rejected);
}
