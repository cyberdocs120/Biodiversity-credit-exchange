#[cfg(test)]
mod test {
    use crate::types::*;
    use crate::{MrvOracleContract, MrvOracleContractClient};
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::{Address, Bytes, BytesN, Env};

    fn setup() -> (Env, MrvOracleContractClient<'static>, Address) {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let contract_id = env.register(MrvOracleContract, (&admin,));
        let client = MrvOracleContractClient::new(&env, &contract_id);

        (env, client, admin)
    }

    fn sample_pubkey(env: &Env, b: u8) -> BytesN<32> {
        BytesN::from_array(env, &[b; 32])
    }

    #[test]
    fn test_register_oracle() {
        let (env, client, _admin) = setup();
        let pubkey = sample_pubkey(&env, 1);
        let uri = Bytes::from_slice(&env, b"https://oracle.example.com");

        client.register_oracle(&pubkey, &uri, &OracleType::EdnaLab);

        assert_eq!(client.oracle_count(), 1);

        let oracle = client.get_oracle(&pubkey);
        assert!(oracle.active);
        assert_eq!(oracle.pubkey, pubkey);
        assert_eq!(oracle.oracle_type, OracleType::EdnaLab);
        assert_eq!(oracle.uri, uri);
        assert_eq!(oracle.total_surveys, 0);
        assert_eq!(oracle.accuracy_score, 100);
    }

    #[test]
    fn test_revoke_oracle() {
        let (env, client, _admin) = setup();
        let pubkey = sample_pubkey(&env, 1);
        let uri = Bytes::from_slice(&env, b"https://oracle.example.com");

        client.register_oracle(&pubkey, &uri, &OracleType::CameraTrapAi);
        assert_eq!(client.oracle_count(), 1);

        client.revoke_oracle(&pubkey);

        let oracle = client.get_oracle(&pubkey);
        assert!(!oracle.active);
    }

    #[test]
    fn test_set_threshold() {
        let (_env, client, _admin) = setup();

        let (n, d) = client.threshold();
        assert_eq!(n, 1);
        assert_eq!(d, 1);

        client.set_threshold(&3, &5);

        let (n, d) = client.threshold();
        assert_eq!(n, 3);
        assert_eq!(d, 5);
    }

    #[test]
    fn test_pause_resume() {
        let (_env, client, _admin) = setup();

        assert!(!client.paused());

        client.pause();
        assert!(client.paused());

        client.resume();
        assert!(!client.paused());
    }

    #[test]
    fn test_register_polygon() {
        let (env, client, _admin) = setup();
        let polygon_id = sample_pubkey(&env, 10);
        let cid = Bytes::from_slice(&env, b"QmGeoData");
        let bbox = BoundingBox {
            min_lat: -1000000,
            max_lat: 1000000,
            min_lon: -2000000,
            max_lon: 2000000,
        };
        let country = BytesN::from_array(&env, &[0x55, 0x53]);
        let project_id = sample_pubkey(&env, 99);

        client.register_polygon(&polygon_id, &cid, &bbox, &4500, &3u32, &country, &project_id);

        let polygon = client.get_polygon(&polygon_id);
        assert_eq!(polygon.polygon_id, polygon_id);
        assert_eq!(polygon.geometry_ipfs_cid, cid);
        assert_eq!(polygon.bounding_box.min_lat, -1000000);
        assert_eq!(polygon.bounding_box.max_lat, 1000000);
        assert_eq!(polygon.area_ha, 4500);
        assert_eq!(polygon.biome, 3u32);
        assert_eq!(polygon.country, country);
        assert_eq!(polygon.project_id, project_id);
        assert!(polygon.active);
        assert_eq!(polygon.total_credits_minted, 0);
        assert_eq!(polygon.total_credits_retired, 0);
        assert!(polygon.last_survey_cid.is_none());
    }

    #[test]
    fn test_close_polygon() {
        let (env, client, _admin) = setup();
        let polygon_id = sample_pubkey(&env, 10);
        let cid = Bytes::from_slice(&env, b"QmGeoData");
        let bbox = BoundingBox {
            min_lat: -1000000,
            max_lat: 1000000,
            min_lon: -2000000,
            max_lon: 2000000,
        };
        let country = BytesN::from_array(&env, &[0x55, 0x53]);
        let project_id = sample_pubkey(&env, 99);

        client.register_polygon(&polygon_id, &cid, &bbox, &4500, &3u32, &country, &project_id);
        assert!(client.get_polygon(&polygon_id).active);

        client.close_polygon(&polygon_id);

        assert!(!client.get_polygon(&polygon_id).active);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #2)")]
    fn test_duplicate_oracle_rejected() {
        let (env, client, _admin) = setup();
        let pubkey = sample_pubkey(&env, 1);
        let uri = Bytes::from_slice(&env, b"https://oracle.example.com");

        client.register_oracle(&pubkey, &uri, &OracleType::EdnaLab);
        client.register_oracle(&pubkey, &uri, &OracleType::EdnaLab);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #3)")]
    fn test_get_nonexistent_oracle() {
        let (env, client, _admin) = setup();
        let phantom = sample_pubkey(&env, 99);
        client.get_oracle(&phantom);
    }
}
