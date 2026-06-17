#![no_std]
use soroban_sdk::{
    contract, contractimpl, panic_with_error, symbol_short, Address, BytesN, Env, IntoVal, Val, Vec,
};

mod errors;
mod storage;
mod types;

#[cfg(test)]
mod test;

pub use crate::errors::ApprovalError;
use crate::storage::*;
use crate::types::Stakeholder as StakeholderType;
pub use crate::types::{
    Biome, MintParams, Proposal, ProposalState, ProposeParams, Stakeholder, StakeholderRole, Vote,
};

#[contract]
pub struct ApprovalGovContract;

#[contractimpl]
impl ApprovalGovContract {
    pub fn initialize(env: Env, admin: Address, min_weight: u32, voting_period_secs: u64) {
        admin.require_auth();
        write_admin(&env, &admin);
        write_min_threshold(&env, min_weight);
        write_voting_period(&env, voting_period_secs);
        write_proposal_counter(&env, 0);
        write_stakeholder_counter(&env, 0);
    }

    pub fn admin(env: Env) -> Address {
        read_admin(&env)
    }

    pub fn transfer_admin(env: Env, new_admin: Address) {
        let current_admin = read_admin(&env);
        current_admin.require_auth();
        new_admin.require_auth();
        write_admin(&env, &new_admin);
    }

    pub fn set_bdc_token(env: Env, addr: Address) {
        read_admin(&env).require_auth();
        write_bdc_token(&env, &addr);
    }

    pub fn bdc_token(env: Env) -> Address {
        read_bdc_token(&env)
    }

    pub fn set_mrv_oracle(env: Env, addr: Address) {
        read_admin(&env).require_auth();
        write_mrv_oracle(&env, &addr);
    }

    pub fn mrv_oracle(env: Env) -> Address {
        read_mrv_oracle(&env)
    }

    pub fn register_stakeholder(
        env: Env,
        addr: Address,
        role: StakeholderRole,
        weight: u32,
        has_veto: bool,
    ) {
        read_admin(&env).require_auth();

        if has_stakeholder(&env, &addr) {
            panic_with_error!(&env, ApprovalError::StakeholderAlreadyRegistered);
        }

        if weight == 0 {
            panic_with_error!(&env, ApprovalError::InvalidWeight);
        }

        let role_clone = role.clone();
        let stakeholder = StakeholderType {
            addr: addr.clone(),
            role,
            weight,
            has_veto,
            active: true,
            registered_at: env.ledger().timestamp(),
        };

        write_stakeholder(&env, &addr, &stakeholder);
        let count = read_stakeholder_counter(&env) + 1;
        write_stakeholder_counter(&env, count);

        env.events().publish(
            (symbol_short!("gov"), symbol_short!("reg")),
            (addr, role_clone, weight, has_veto),
        );
    }

    pub fn remove_stakeholder(env: Env, addr: Address) {
        read_admin(&env).require_auth();

        let mut stakeholder = read_stakeholder(&env, &addr).unwrap_or_else(|| {
            panic_with_error!(&env, ApprovalError::StakeholderNotFound);
        });

        stakeholder.active = false;
        write_stakeholder(&env, &addr, &stakeholder);

        env.events()
            .publish((symbol_short!("gov"), symbol_short!("remv")), addr);
    }

    pub fn get_stakeholder(env: Env, addr: Address) -> Stakeholder {
        read_stakeholder(&env, &addr).unwrap_or_else(|| {
            panic_with_error!(&env, ApprovalError::StakeholderNotFound);
        })
    }

    pub fn stakeholder_count(env: Env) -> u32 {
        read_stakeholder_counter(&env)
    }

    pub fn set_min_threshold(env: Env, min_weight: u32) {
        read_admin(&env).require_auth();
        write_min_threshold(&env, min_weight);
    }

    pub fn min_threshold(env: Env) -> u32 {
        read_min_threshold(&env)
    }

    pub fn set_voting_period(env: Env, secs: u64) {
        read_admin(&env).require_auth();
        write_voting_period(&env, secs);
    }

    pub fn voting_period(env: Env) -> u64 {
        read_voting_period(&env)
    }

    pub fn propose(env: Env, proposer: Address, params: ProposeParams) -> u64 {
        proposer.require_auth();
        let counter = read_proposal_counter(&env) + 1;
        write_proposal_counter(&env, counter);

        let now = env.ledger().timestamp();
        let period = read_voting_period(&env);

        let proposal = Proposal {
            proposal_id: counter,
            polygon_id: params.polygon_id,
            survey_hash: params.survey_hash,
            methodology_id: params.methodology_id,
            credit_qty: params.credit_qty,
            beneficiary: params.beneficiary,
            proposer,
            created_at: now,
            voting_deadline: now + period,
            state: ProposalState::Voting,
            votes: Vec::new(&env),
            community_veto: false,
            weighted_total_approve: 0,
            weighted_total_reject: 0,
            survey_ipfs_cid: params.survey_ipfs_cid,
            baseline_bsi: params.baseline_bsi,
            current_bsi: params.current_bsi,
            area_ha_contribution: params.area_ha_contribution,
            biome: params.biome,
            vintage_year: params.vintage_year,
            vintage_quarter: params.vintage_quarter,
            approval_governance_id: params.approval_governance_id,
        };

        write_proposal(&env, counter, &proposal);

        env.events().publish(
            (symbol_short!("gov"), symbol_short!("prop")),
            (counter, proposal.survey_hash, proposal.credit_qty),
        );

        counter
    }

    pub fn vote(
        env: Env,
        voter: Address,
        proposal_id: u64,
        approve: bool,
        comment_hash: BytesN<32>,
    ) {
        voter.require_auth();

        let mut proposal = read_proposal(&env, proposal_id).unwrap_or_else(|| {
            panic_with_error!(&env, ApprovalError::ProposalNotFound);
        });

        if proposal.state != ProposalState::Voting {
            panic_with_error!(&env, ApprovalError::ProposalNotVoting);
        }

        if env.ledger().timestamp() > proposal.voting_deadline {
            panic_with_error!(&env, ApprovalError::VotingPeriodExpired);
        }

        let stakeholder = read_stakeholder(&env, &voter).unwrap_or_else(|| {
            panic_with_error!(&env, ApprovalError::VoterNotStakeholder);
        });

        if !stakeholder.active {
            panic_with_error!(&env, ApprovalError::VoterNotStakeholder);
        }

        for i in 0..proposal.votes.len() {
            let v = proposal.votes.get(i).unwrap();
            if v.voter == voter {
                panic_with_error!(&env, ApprovalError::VoteAlreadyCast);
            }
        }

        let vote = Vote {
            voter: voter.clone(),
            approve,
            weight: stakeholder.weight,
            comment_hash,
            timestamp: env.ledger().timestamp(),
        };

        proposal.votes.push_back(vote);

        if approve {
            proposal.weighted_total_approve += stakeholder.weight;
        } else {
            proposal.weighted_total_reject += stakeholder.weight;
        }

        env.events().publish(
            (symbol_short!("gov"), symbol_short!("vote")),
            (proposal_id, voter, approve, stakeholder.weight),
        );

        Self::check_and_transition(&env, &mut proposal);

        write_proposal(&env, proposal_id, &proposal);
    }

    pub fn veto(env: Env, voter: Address, proposal_id: u64) {
        voter.require_auth();

        let mut proposal = read_proposal(&env, proposal_id).unwrap_or_else(|| {
            panic_with_error!(&env, ApprovalError::ProposalNotFound);
        });

        if proposal.state != ProposalState::Voting {
            panic_with_error!(&env, ApprovalError::ProposalNotVoting);
        }

        let stakeholder = read_stakeholder(&env, &voter).unwrap_or_else(|| {
            panic_with_error!(&env, ApprovalError::VoterNotStakeholder);
        });

        if !stakeholder.active || !stakeholder.has_veto {
            panic_with_error!(&env, ApprovalError::VetoPowerRequired);
        }

        proposal.community_veto = true;
        proposal.state = ProposalState::Rejected;

        write_proposal(&env, proposal_id, &proposal);

        env.events().publish(
            (symbol_short!("gov"), symbol_short!("veto")),
            (proposal_id, voter),
        );
    }

    pub fn on_approved(env: Env, proposal_id: u64) {
        let proposal = read_proposal(&env, proposal_id).unwrap_or_else(|| {
            panic_with_error!(&env, ApprovalError::ProposalNotFound);
        });

        if proposal.state != ProposalState::Approved {
            panic_with_error!(&env, ApprovalError::ProposalNotVoting);
        }

        Self::mint_credits(&env, &proposal);

        env.events()
            .publish((symbol_short!("gov"), symbol_short!("appr")), proposal_id);
    }

    pub fn close_proposal(env: Env, caller: Address, proposal_id: u64) {
        caller.require_auth();
        let admin = read_admin(&env);

        let mut proposal = read_proposal(&env, proposal_id).unwrap_or_else(|| {
            panic_with_error!(&env, ApprovalError::ProposalNotFound);
        });

        if proposal.state != ProposalState::Voting {
            panic_with_error!(&env, ApprovalError::ProposalAlreadyClosed);
        }

        let can_close = caller == admin || env.ledger().timestamp() > proposal.voting_deadline;
        if !can_close {
            panic_with_error!(&env, ApprovalError::Unauthorized);
        }

        Self::check_and_transition(&env, &mut proposal);

        if proposal.state == ProposalState::Voting {
            proposal.state = ProposalState::Rejected;
        }

        write_proposal(&env, proposal_id, &proposal);

        if proposal.state == ProposalState::Approved {
            env.events()
                .publish((symbol_short!("gov"), symbol_short!("appr")), proposal_id);
        } else {
            env.events()
                .publish((symbol_short!("gov"), symbol_short!("rej")), proposal_id);
        }
    }

    pub fn get_proposal(env: Env, proposal_id: u64) -> Proposal {
        read_proposal(&env, proposal_id).unwrap_or_else(|| {
            panic_with_error!(&env, ApprovalError::ProposalNotFound);
        })
    }
}

impl ApprovalGovContract {
    fn check_and_transition(env: &Env, proposal: &mut Proposal) {
        if proposal.community_veto {
            proposal.state = ProposalState::Rejected;
            return;
        }

        let min = read_min_threshold(env);
        if proposal.weighted_total_approve >= min {
            proposal.state = ProposalState::Approved;
            Self::mint_credits(env, proposal);
        } else if proposal.weighted_total_reject >= min {
            proposal.state = ProposalState::Rejected;
        }
    }

    fn mint_credits(env: &Env, proposal: &Proposal) {
        if !has_bdc_token(env) {
            return;
        }

        let bdc_id = read_bdc_token(env);

        for _ in 0..proposal.credit_qty {
            let _: Val = env.invoke_contract(
                &bdc_id,
                &symbol_short!("mint"),
                (
                    proposal.beneficiary.clone(),
                    MintParams {
                        polygon_id: proposal.polygon_id.clone(),
                        methodology_id: proposal.methodology_id.clone(),
                        survey_ipfs_cid: proposal.survey_ipfs_cid.clone(),
                        baseline_bsi: proposal.baseline_bsi,
                        current_bsi: proposal.current_bsi,
                        area_ha_contribution: proposal.area_ha_contribution,
                        biome: match proposal.biome {
                            0 => Biome::TropicalForest,
                            1 => Biome::TemperateForest,
                            2 => Biome::Grassland,
                            3 => Biome::Wetland,
                            4 => Biome::Mangrove,
                            5 => Biome::CoralReef,
                            _ => Biome::Other,
                        },
                        vintage_year: proposal.vintage_year,
                        vintage_quarter: proposal.vintage_quarter,
                        approval_governance_id: proposal.approval_governance_id.clone(),
                    },
                )
                    .into_val(env),
            );
        }
    }
}
