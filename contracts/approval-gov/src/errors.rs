use soroban_sdk::contracterror;

#[derive(Copy, Clone, Debug, PartialEq)]
#[contracterror]
pub enum ApprovalError {
    Unauthorized = 1,
    StakeholderAlreadyRegistered = 2,
    StakeholderNotFound = 3,
    ProposalNotFound = 4,
    ProposalNotVoting = 5,
    ProposalAlreadyClosed = 6,
    VoterNotStakeholder = 7,
    VoteAlreadyCast = 8,
    CommunityVetoActivated = 9,
    ThresholdNotMet = 10,
    InvalidWeight = 11,
    VotingPeriodExpired = 12,
    VetoPowerRequired = 13,
    InvalidRole = 14,
}
