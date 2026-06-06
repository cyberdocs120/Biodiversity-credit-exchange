#[cfg(test)]
mod test {
    use crate::types::*;
    use crate::{BdcTokenContract, BdcTokenContractClient};
    use soroban_sdk::testutils::{Address as _, Ledger as _};
    use soroban_sdk::{Address, Bytes, BytesN, Env};

    fn setup() -> (Env, BdcTokenContractClient<'static>, Address, Address, Address) {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let minter = Address::generate(&env);
        let receiver = Address::generate(&env);

        let contract_id = env.register(BdcTokenContract, (&admin,));
        let client = BdcTokenContractClient::new(&env, &contract_id);

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
            approval_governance_id: BytesN::from_array(env, &[3u8; 32]),
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
}
