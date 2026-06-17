#[cfg(test)]
#[allow(clippy::module_inception)]
mod test {
    use crate::types::*;
    use crate::{BdcTokenContract, BdcTokenContractClient};
    use soroban_sdk::testutils::{Address as _, Events, Ledger as _};
    use soroban_sdk::{symbol_short, Address, Bytes, BytesN, Env, IntoVal};

    fn setup() -> (
        Env,
        BdcTokenContractClient<'static>,
        Address,
        Address,
        Address,
    ) {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let minter = Address::generate(&env);
        let receiver = Address::generate(&env);

        let contract_id = env.register(BdcTokenContract, ());
        let client = BdcTokenContractClient::new(&env, &contract_id);

        client.initialize(&admin);
        client.authorize_minter(&minter);

        (env, client, admin, minter, receiver)
    }

    fn sample_params(env: &Env) -> MintParams {
        MintParams {
            polygon_id: BytesN::from_array(env, &[1u8; 32]),
            methodology_id: BytesN::from_array(env, &[2u8; 8]),
            survey_ipfs_cid: Bytes::from_slice(env, b"QmTest"),
            baseline_bsi: 28,
            current_bsi: 64,
            area_ha_contribution: 100,
            biome: Biome::TropicalForest,
            vintage_year: 2025,
            vintage_quarter: 2,
            approval_governance_id: Address::generate(env),
        }
    }

    fn mint_token(env: &Env, client: &BdcTokenContractClient, receiver: &Address) -> u64 {
        let params = sample_params(env);
        client.mint(receiver, &params)
    }

    #[test]
    fn test_mint_increases_supply() {
        let (env, client, _admin, _minter, receiver) = setup();
        let token_id = mint_token(&env, &client, &receiver);

        assert_eq!(client.total_supply(), 1);
        assert_eq!(client.balance_of(&receiver), 1);
        assert_eq!(client.owner_of(&token_id), receiver);
    }

    #[test]
    fn test_mint_multiple_tokens() {
        let (env, client, _admin, _minter, receiver) = setup();
        let params = sample_params(&env);

        let id1 = client.mint(&receiver, &params);
        let id2 = client.mint(&receiver, &params);

        assert_eq!(id2, id1 + 1);
        assert_eq!(client.total_supply(), 2);
        assert_eq!(client.balance_of(&receiver), 2);
    }

    #[test]
    fn test_mint_with_polygon_binding() {
        let (env, client, _admin, _minter, receiver) = setup();
        let params = sample_params(&env);
        let polygon_1 = params.polygon_id.clone();

        client.mint(&receiver, &params);

        let params2 = MintParams {
            polygon_id: BytesN::from_array(&env, &[4u8; 32]),
            ..sample_params(&env)
        };
        client.mint(&receiver, &params2);

        let meta = client.token_metadata(&1);
        assert_eq!(meta.polygon_id, polygon_1);
        assert_eq!(client.total_supply(), 2);
    }

    #[test]
    fn test_transfer_changes_owner() {
        let (env, client, _admin, _minter, receiver) = setup();
        let token_id = mint_token(&env, &client, &receiver);
        let new_owner = Address::generate(&env);

        client.transfer(&receiver, &new_owner, &token_id);

        assert_eq!(client.owner_of(&token_id), new_owner);
        assert_eq!(client.balance_of(&receiver), 0);
        assert_eq!(client.balance_of(&new_owner), 1);
    }

    #[test]
    fn test_burn_retires_token() {
        let (env, client, _admin, _minter, receiver) = setup();
        env.ledger().set_timestamp(5000);
        let token_id = mint_token(&env, &client, &receiver);

        client.burn(&receiver, &token_id);

        let meta = client.token_metadata(&token_id);
        assert_eq!(meta.state, BdcState::Retired);
        assert_eq!(meta.retired_at, Some(5000));
        assert_eq!(client.total_supply(), 0);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #2)")]
    fn test_double_burn_rejected() {
        let (env, client, _admin, _minter, receiver) = setup();
        env.ledger().set_timestamp(5000);
        let token_id = mint_token(&env, &client, &receiver);

        client.burn(&receiver, &token_id);
        client.burn(&receiver, &token_id);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #1)")]
    fn test_non_owner_transfer_rejected() {
        let (env, client, _admin, _minter, receiver) = setup();
        let token_id = mint_token(&env, &client, &receiver);
        let attacker = Address::generate(&env);

        client.transfer(&attacker, &Address::generate(&env), &token_id);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #4)")]
    fn test_burn_nonexistent_token() {
        let (_env, client, _admin, _minter, receiver) = setup();

        client.burn(&receiver, &999);
    }

    #[test]
    fn test_set_metadata_uri() {
        let (env, client, _admin, _minter, receiver) = setup();
        let token_id = mint_token(&env, &client, &receiver);

        let new_uri = Bytes::from_slice(&env, b"https://example.com/token/1");
        client.set_metadata_uri(&token_id, &new_uri);

        let uri = client.token_uri(&token_id);
        assert_eq!(uri, new_uri);
    }

    #[test]
    fn test_tokens_by_owner_pagination() {
        let (env, client, _admin, _minter, receiver) = setup();
        let params = sample_params(&env);

        for _ in 0..5 {
            client.mint(&receiver, &params);
        }

        let page_1 = client.tokens_by_owner(&receiver, &0, &2);
        assert_eq!(page_1.len(), 2);
        assert_eq!(page_1.get(0).unwrap(), 1);
        assert_eq!(page_1.get(1).unwrap(), 2);

        let page_2 = client.tokens_by_owner(&receiver, &2, &2);
        assert_eq!(page_2.len(), 2);
        assert_eq!(page_2.get(0).unwrap(), 3);
        assert_eq!(page_2.get(1).unwrap(), 4);

        let page_last = client.tokens_by_owner(&receiver, &4, &2);
        assert_eq!(page_last.len(), 1);
        assert_eq!(page_last.get(0).unwrap(), 5);
    }

    #[test]
    fn test_tokens_by_polygon() {
        let (env, client, _admin, _minter, receiver) = setup();

        let polygon_a = BytesN::from_array(&env, &[1u8; 32]);
        let polygon_b = BytesN::from_array(&env, &[2u8; 32]);

        let mut params_a = sample_params(&env);
        params_a.polygon_id = polygon_a.clone();

        let mut params_b = sample_params(&env);
        params_b.polygon_id = polygon_b.clone();

        client.mint(&receiver, &params_a);
        client.mint(&receiver, &params_a);
        client.mint(&receiver, &params_b);

        let tokens_a = client.tokens_by_polygon(&polygon_a, &0, &10);
        assert_eq!(tokens_a.len(), 2);
        assert_eq!(tokens_a.get(0).unwrap(), 1);
        assert_eq!(tokens_a.get(1).unwrap(), 2);

        let tokens_b = client.tokens_by_polygon(&polygon_b, &0, &10);
        assert_eq!(tokens_b.len(), 1);
        assert_eq!(tokens_b.get(0).unwrap(), 3);
    }

    #[test]
    fn test_tokens_by_owner_excludes_retired() {
        let (env, client, _admin, _minter, receiver) = setup();
        env.ledger().set_timestamp(5000);
        let params = sample_params(&env);

        let id1 = client.mint(&receiver, &params);
        client.mint(&receiver, &params);

        client.burn(&receiver, &id1);

        let tokens = client.tokens_by_owner(&receiver, &0, &10);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.get(0).unwrap(), 2);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #2)")]
    fn test_transfer_retired_token_rejected() {
        let (env, client, _admin, _minter, receiver) = setup();
        env.ledger().set_timestamp(5000);
        let token_id = mint_token(&env, &client, &receiver);

        client.burn(&receiver, &token_id);

        let new_owner = Address::generate(&env);
        client.transfer(&receiver, &new_owner, &token_id);
    }

    #[test]
    fn test_mint_emits_event() {
        let (env, client, _admin, _minter, receiver) = setup();
        let _token_id = mint_token(&env, &client, &receiver);

        let events = env.events().all();
        // Events are tuples of (contract_id, topics, data)
        let raw = events.get(events.len() - 1).unwrap();
        let (_contract_id, topics, _data) = raw;

        assert_eq!(topics.len(), 2);
        let topic0: soroban_sdk::Symbol = topics.get(0).unwrap().into_val(&env);
        let topic1: soroban_sdk::Symbol = topics.get(1).unwrap().into_val(&env);
        assert_eq!(topic0, symbol_short!("bdc"));
        assert_eq!(topic1, symbol_short!("mint"));
    }
}
