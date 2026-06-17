#![no_std]

#[cfg(test)]
#[allow(clippy::byte_char_slices)]
mod test {
    use soroban_sdk::{testutils::Address as _, vec, Address, Bytes, BytesN, Env};

    use approval_gov::{ApprovalGovContract, ApprovalGovContractClient, StakeholderRole};
    use bdc_token::{BdcTokenContract, BdcTokenContractClient};
    use marketplace::{
        MarketplaceContract, MarketplaceContractClient, OrderRestriction, OrderSide,
    };
    use mrv_oracle::{
        BoundingBox, MrvOracleContract, MrvOracleContractClient, OracleType, SurveyData,
    };
    use retirement::{ClaimData, RetirementContract, RetirementContractClient};

    #[soroban_sdk::contract]
    pub struct MockUsdc;

    #[soroban_sdk::contractimpl]
    impl MockUsdc {
        pub fn transfer(_env: Env, _from: Address, _to: Address, _amount: i128) {}
    }

    fn setup_oracles(env: &Env, mrv_client: &MrvOracleContractClient) -> (BytesN<32>, BytesN<32>) {
        let o1 = BytesN::from_array(env, &[1u8; 32]);
        let o2 = BytesN::from_array(env, &[2u8; 32]);
        let o3 = BytesN::from_array(env, &[3u8; 32]);
        mrv_client.register_oracle(
            &o1,
            &Bytes::from_slice(env, b"ipfs://o1"),
            &OracleType::EdnaLab,
        );
        mrv_client.register_oracle(
            &o2,
            &Bytes::from_slice(env, b"ipfs://o2"),
            &OracleType::SatelliteImagery,
        );
        mrv_client.register_oracle(
            &o3,
            &Bytes::from_slice(env, b"ipfs://o3"),
            &OracleType::FieldSurvey,
        );
        mrv_client.set_threshold(&2, &3);
        (o1, o2)
    }

    fn setup_polygon(env: &Env, mrv_client: &MrvOracleContractClient) -> BytesN<32> {
        let pid = BytesN::from_array(env, &[10u8; 32]);
        mrv_client.register_polygon(
            &pid,
            &Bytes::from_slice(env, b"ipfs://poly1"),
            &BoundingBox {
                min_lat: 0,
                max_lat: 10,
                min_lon: 0,
                max_lon: 10,
            },
            &1000,
            &0,
            &BytesN::from_array(env, &[b'B', b'R']),
            &BytesN::from_array(env, &[1u8; 32]),
        );
        pid
    }

    fn setup_stakeholders(
        _env: &Env,
        gov_client: &ApprovalGovContractClient,
        e: &Address,
        c: &Address,
        a: &Address,
    ) {
        gov_client.register_stakeholder(e, &StakeholderRole::LeadEcologist, &3, &false);
        gov_client.register_stakeholder(c, &StakeholderRole::LocalCommunityRep, &2, &true);
        gov_client.register_stakeholder(a, &StakeholderRole::IndependentAuditor, &1, &false);
    }

    fn do_survey(
        env: &Env,
        mrv_client: &MrvOracleContractClient,
        pid: &BytesN<32>,
        beneficiary: &Address,
        o1: &BytesN<32>,
        o2: &BytesN<32>,
    ) {
        let sig1 = BytesN::from_array(env, &[0u8; 64]);
        let sig2 = BytesN::from_array(env, &[0u8; 64]);
        let signatures = vec![env, (o1.clone(), sig1), (o2.clone(), sig2)];
        let analyses = vec![
            env,
            BytesN::from_array(env, &[0u8; 32]),
            BytesN::from_array(env, &[0u8; 32]),
        ];
        mrv_client.submit_survey(&SurveyData {
            polygon_id: pid.clone(),
            ipfs_cid: Bytes::from_slice(env, b"ipfs://survey1"),
            survey_timestamp: env.ledger().timestamp(),
            signatures,
            analyses_hashes: analyses,
            baseline_bsi: 50,
            current_bsi: 80,
            area_contribution: 1,
            biome: 0,
            vintage_year: 2024,
            vintage_qtr: 1,
            methodology_id: BytesN::from_array(env, &[0u8; 8]),
            beneficiary: beneficiary.clone(),
        });
    }

    #[test]
    fn test_full_lifecycle() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let project_dev = Address::generate(&env);
        let buyer = Address::generate(&env);
        let ecologist = Address::generate(&env);
        let community_rep = Address::generate(&env);
        let auditor = Address::generate(&env);
        let fee_vault = Address::generate(&env);

        // Deploy BDC Token
        let bdc_id = env.register(BdcTokenContract, ());
        let bdc_client = BdcTokenContractClient::new(&env, &bdc_id);
        bdc_client.initialize(&admin);

        // Deploy MRV Oracle
        let mrv_id = env.register(MrvOracleContract, ());
        let mrv_client = MrvOracleContractClient::new(&env, &mrv_id);
        mrv_client.initialize(&admin);
        mrv_client.set_bdc_token(&bdc_id);

        // Deploy Approval Gov
        let gov_id = env.register(ApprovalGovContract, ());
        let gov_client = ApprovalGovContractClient::new(&env, &gov_id);
        gov_client.initialize(&admin, &6u32, &604800u64);
        gov_client.set_bdc_token(&bdc_id);
        gov_client.set_mrv_oracle(&mrv_id);

        mrv_client.set_approval_gov(&gov_id);
        bdc_client.authorize_minter(&gov_id);

        // Setup oracles
        let (o1, o2) = setup_oracles(&env, &mrv_client);
        let polygon_id = setup_polygon(&env, &mrv_client);
        setup_stakeholders(&env, &gov_client, &ecologist, &community_rep, &auditor);

        // Submit survey
        do_survey(&env, &mrv_client, &polygon_id, &project_dev, &o1, &o2);

        let pid = 1u64;
        gov_client.vote(
            &ecologist,
            &pid,
            &true,
            &BytesN::from_array(&env, &[0u8; 32]),
        );
        gov_client.vote(
            &community_rep,
            &pid,
            &true,
            &BytesN::from_array(&env, &[0u8; 32]),
        );
        gov_client.vote(&auditor, &pid, &true, &BytesN::from_array(&env, &[0u8; 32]));

        let proposal = gov_client.get_proposal(&pid);
        assert_eq!(proposal.state as u32, 2); // Approved

        // Verify BDC tokens minted: (80-50)*1 = 30 credits
        assert_eq!(bdc_client.total_supply(), 30);

        // Marketplace: deploy and match
        let usdc_id = env.register(MockUsdc, ());
        let mkt_id = env.register(MarketplaceContract, ());
        let mkt_client = MarketplaceContractClient::new(&env, &mkt_id);
        mkt_client.initialize(&admin);
        mkt_client.set_bdc_token(&bdc_id);
        mkt_client.set_usdc_token(&usdc_id);
        mkt_client.set_fee_vault(&fee_vault);

        let sell_id = mkt_client.place_order(
            &project_dev,
            &OrderSide::Sell,
            &100,
            &10,
            &OrderRestriction::None,
            &None,
            &None,
        );
        let buy_id = mkt_client.place_order(
            &buyer,
            &OrderSide::Buy,
            &100,
            &10,
            &OrderRestriction::None,
            &None,
            &None,
        );

        mkt_client.match_orders(&buy_id, &sell_id);

        assert_eq!(bdc_client.balance_of(&buyer), 10);
        assert_eq!(bdc_client.balance_of(&project_dev), 20);

        // Retirement
        let ret_id = env.register(RetirementContract, ());
        let ret_client = RetirementContractClient::new(&env, &ret_id);
        ret_client.initialize(&admin);
        ret_client.set_bdc_token(&bdc_id);
        bdc_client.authorize_burner(&ret_id);

        let buyer_tokens = bdc_client.tokens_by_owner(&buyer, &0, &10);
        let receipt_id = ret_client.retire(
            &buyer,
            &buyer_tokens,
            &polygon_id,
            &ClaimData {
                period_start: 0,
                period_end: 1000,
                purpose: Bytes::from_slice(&env, b"offset"),
                jurisdiction: Bytes::from_slice(&env, b"global"),
            },
        );

        let receipt = ret_client.get_receipt(&receipt_id);
        assert_eq!(receipt.total_credits, 10);
        assert_eq!(bdc_client.balance_of(&buyer), 0);
    }

    #[test]
    #[should_panic]
    fn test_unauthorized_mint_rejected() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let project_dev = Address::generate(&env);

        let bdc_id = env.register(BdcTokenContract, ());
        let bdc_client = BdcTokenContractClient::new(&env, &bdc_id);
        bdc_client.initialize(&admin);

        let mrv_id = env.register(MrvOracleContract, ());
        let mrv_client = MrvOracleContractClient::new(&env, &mrv_id);
        mrv_client.initialize(&admin);
        mrv_client.set_bdc_token(&bdc_id);

        let gov_id = env.register(ApprovalGovContract, ());
        let gov_client = ApprovalGovContractClient::new(&env, &gov_id);
        gov_client.initialize(&admin, &6u32, &604800u64);
        gov_client.set_bdc_token(&bdc_id);
        gov_client.set_mrv_oracle(&mrv_id);

        mrv_client.set_approval_gov(&gov_id);
        // NOTE: deliberately NOT calling bdc_client.authorize_minter(&gov_id)

        let o1 = BytesN::from_array(&env, &[1u8; 32]);
        let o2 = BytesN::from_array(&env, &[2u8; 32]);
        mrv_client.register_oracle(
            &o1,
            &Bytes::from_slice(&env, b"ipfs://o1"),
            &OracleType::EdnaLab,
        );
        mrv_client.register_oracle(
            &o2,
            &Bytes::from_slice(&env, b"ipfs://o2"),
            &OracleType::SatelliteImagery,
        );
        mrv_client.set_threshold(&2, &3);

        let polygon_id = BytesN::from_array(&env, &[10u8; 32]);
        mrv_client.register_polygon(
            &polygon_id,
            &Bytes::from_slice(&env, b"ipfs://poly1"),
            &BoundingBox {
                min_lat: 0,
                max_lat: 10,
                min_lon: 0,
                max_lon: 10,
            },
            &1000,
            &0,
            &BytesN::from_array(&env, &[b'B', b'R']),
            &BytesN::from_array(&env, &[1u8; 32]),
        );

        let ecologist = Address::generate(&env);
        gov_client.register_stakeholder(&ecologist, &StakeholderRole::LeadEcologist, &6, &false);

        let sig1 = BytesN::from_array(&env, &[0u8; 64]);
        let sig2 = BytesN::from_array(&env, &[0u8; 64]);
        let signatures = vec![&env, (o1.clone(), sig1), (o2.clone(), sig2)];
        let analyses = vec![
            &env,
            BytesN::from_array(&env, &[0u8; 32]),
            BytesN::from_array(&env, &[0u8; 32]),
        ];
        mrv_client.submit_survey(&SurveyData {
            polygon_id: polygon_id.clone(),
            ipfs_cid: Bytes::from_slice(&env, b"ipfs://survey1"),
            survey_timestamp: env.ledger().timestamp(),
            signatures,
            analyses_hashes: analyses,
            baseline_bsi: 50,
            current_bsi: 80,
            area_contribution: 1,
            biome: 0,
            vintage_year: 2024,
            vintage_qtr: 1,
            methodology_id: BytesN::from_array(&env, &[0u8; 8]),
            beneficiary: project_dev.clone(),
        });

        // Vote triggers mint_credits -> bdc_token.mint() which fails since no minter authorized
        gov_client.vote(&ecologist, &1, &true, &BytesN::from_array(&env, &[0u8; 32]));
    }

    #[test]
    fn test_community_veto_rejects_proposal() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let project_dev = Address::generate(&env);
        let ecologist = Address::generate(&env);
        let community_rep = Address::generate(&env);
        let auditor = Address::generate(&env);

        let bdc_id = env.register(BdcTokenContract, ());
        let bdc_client = BdcTokenContractClient::new(&env, &bdc_id);
        bdc_client.initialize(&admin);

        let mrv_id = env.register(MrvOracleContract, ());
        let mrv_client = MrvOracleContractClient::new(&env, &mrv_id);
        mrv_client.initialize(&admin);
        mrv_client.set_bdc_token(&bdc_id);

        let gov_id = env.register(ApprovalGovContract, ());
        let gov_client = ApprovalGovContractClient::new(&env, &gov_id);
        gov_client.initialize(&admin, &6u32, &604800u64);
        gov_client.set_bdc_token(&bdc_id);
        gov_client.set_mrv_oracle(&mrv_id);

        mrv_client.set_approval_gov(&gov_id);
        bdc_client.authorize_minter(&gov_id);

        let (o1, o2) = setup_oracles(&env, &mrv_client);
        let polygon_id = setup_polygon(&env, &mrv_client);
        setup_stakeholders(&env, &gov_client, &ecologist, &community_rep, &auditor);

        do_survey(&env, &mrv_client, &polygon_id, &project_dev, &o1, &o2);

        // Community rep vetoes
        gov_client.veto(&community_rep, &1);

        let proposal = gov_client.get_proposal(&1);
        assert!(proposal.community_veto);
        assert_eq!(proposal.state as u32, 3); // Rejected

        assert_eq!(bdc_client.total_supply(), 0);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #3)")]
    fn test_double_retire_rejected() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let buyer = Address::generate(&env);

        let bdc_id = env.register(BdcTokenContract, ());
        let bdc_client = BdcTokenContractClient::new(&env, &bdc_id);
        bdc_client.initialize(&admin);
        bdc_client.authorize_minter(&admin);

        let polygon_id = BytesN::from_array(&env, &[10u8; 32]);
        bdc_client.mint(
            &buyer,
            &bdc_token::types::MintParams {
                polygon_id: polygon_id.clone(),
                methodology_id: BytesN::from_array(&env, &[0u8; 8]),
                survey_ipfs_cid: Bytes::new(&env),
                baseline_bsi: 50,
                current_bsi: 80,
                area_ha_contribution: 100,
                biome: bdc_token::types::Biome::TropicalForest,
                vintage_year: 2024,
                vintage_quarter: 1,
                approval_governance_id: Address::generate(&env),
            },
        );

        let ret_id = env.register(RetirementContract, ());
        let ret_client = RetirementContractClient::new(&env, &ret_id);
        ret_client.initialize(&admin);
        ret_client.set_bdc_token(&bdc_id);
        bdc_client.authorize_burner(&ret_id);

        let tokens = vec![&env, 1u64];
        ret_client.retire(
            &buyer,
            &tokens,
            &polygon_id,
            &ClaimData {
                period_start: 0,
                period_end: 1000,
                purpose: Bytes::from_slice(&env, b"offset"),
                jurisdiction: Bytes::from_slice(&env, b"global"),
            },
        );

        ret_client.retire(
            &buyer,
            &tokens,
            &polygon_id,
            &ClaimData {
                period_start: 0,
                period_end: 1000,
                purpose: Bytes::from_slice(&env, b"offset"),
                jurisdiction: Bytes::from_slice(&env, b"global"),
            },
        );
    }
}
