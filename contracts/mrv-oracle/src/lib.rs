#![no_std]
mod errors;
mod storage;
mod types;
#[cfg(test)]
mod test;

use soroban_sdk::{contract, contractimpl, panic_with_error, symbol_short, Address, Bytes, BytesN, Env};
use storage::*;
use types::*;
use errors::MrvOracleError;

#[contract]
pub struct MrvOracleContract;

#[contractimpl]
impl MrvOracleContract {
    pub fn __constructor(env: Env, admin: Address) {
        admin.require_auth();
        write_admin(&env, &admin);
        write_paused(&env, false);
        write_threshold_n(&env, 1);
        write_threshold_d(&env, 1);
        write_oracle_count(&env, 0);
    }

    pub fn admin(env: Env) -> Address {
        read_admin(&env)
    }

    pub fn transfer_admin(env: Env, new_admin: Address) {
        let admin = read_admin(&env);
        admin.require_auth();
        new_admin.require_auth();
        write_admin(&env, &new_admin);
    }

    pub fn set_bdc_token(env: Env, addr: Address) {
        let admin = read_admin(&env);
        admin.require_auth();
        write_bdc_token(&env, &addr);
    }

    pub fn bdc_token(env: Env) -> Address {
        read_bdc_token(&env)
    }

    pub fn register_oracle(env: Env, pubkey: BytesN<32>, uri: Bytes, oracle_type: OracleType) {
        let admin = read_admin(&env);
        admin.require_auth();

        if has_oracle(&env, &pubkey) {
            panic_with_error!(&env, MrvOracleError::OracleAlreadyRegistered);
        }

        let oracle = OracleNode {
            pubkey: pubkey.clone(),
            uri,
            oracle_type,
            active: true,
            registered_at: env.ledger().timestamp(),
            total_surveys: 0,
            accuracy_score: 100,
        };

        write_oracle(&env, &pubkey, &oracle);
        write_oracle_count(&env, read_oracle_count(&env) + 1);

        env.events().publish(
            (symbol_short!("mrvo"), symbol_short!("reg")),
            (pubkey, oracle.oracle_type),
        );
    }

    pub fn revoke_oracle(env: Env, pubkey: BytesN<32>) {
        let admin = read_admin(&env);
        admin.require_auth();

        if !has_oracle(&env, &pubkey) {
            panic_with_error!(&env, MrvOracleError::OracleNotFound);
        }

        let mut oracle = read_oracle(&env, &pubkey).unwrap();
        oracle.active = false;
        write_oracle(&env, &pubkey, &oracle);

        env.events().publish(
            (symbol_short!("mrvo"), symbol_short!("rev")),
            pubkey.clone(),
        );
    }

    pub fn oracle_count(env: Env) -> u32 {
        read_oracle_count(&env)
    }

    pub fn get_oracle(env: Env, pubkey: BytesN<32>) -> OracleNode {
        if !has_oracle(&env, &pubkey) {
            panic_with_error!(&env, MrvOracleError::OracleNotFound);
        }
        read_oracle(&env, &pubkey).unwrap()
    }

    pub fn set_threshold(env: Env, n: u32, d: u32) {
        let admin = read_admin(&env);
        admin.require_auth();

        if n == 0 || d == 0 || n > d {
            panic_with_error!(&env, MrvOracleError::InvalidSurveyData);
        }

        write_threshold_n(&env, n);
        write_threshold_d(&env, d);
    }

    pub fn threshold(env: Env) -> (u32, u32) {
        (read_threshold_n(&env), read_threshold_d(&env))
    }

    pub fn register_polygon(
        env: Env,
        polygon_id: BytesN<32>,
        geometry_ipfs_cid: Bytes,
        bounding_box: BoundingBox,
        area_ha: u64,
        biome: u32,
        country: BytesN<2>,
        project_id: BytesN<32>,
    ) {
        let admin = read_admin(&env);
        admin.require_auth();

        let polygon = HabitatPolygon {
            polygon_id: polygon_id.clone(),
            geometry_ipfs_cid,
            bounding_box,
            area_ha,
            biome,
            country,
            project_id,
            registered_at: env.ledger().timestamp(),
            active: true,
            total_credits_minted: 0,
            total_credits_retired: 0,
            last_survey_cid: None,
            last_survey_timestamp: None,
        };

        write_polygon(&env, &polygon_id, &polygon);

        env.events().publish(
            (symbol_short!("mrvo"), symbol_short!("poly")),
            polygon_id.clone(),
        );
    }

    pub fn close_polygon(env: Env, polygon_id: BytesN<32>) {
        let admin = read_admin(&env);
        admin.require_auth();

        if !has_polygon(&env, &polygon_id) {
            panic_with_error!(&env, MrvOracleError::PolygonNotFound);
        }

        let mut polygon = read_polygon(&env, &polygon_id).unwrap();
        polygon.active = false;
        write_polygon(&env, &polygon_id, &polygon);

        env.events().publish(
            (symbol_short!("mrvo"), symbol_short!("clos")),
            polygon_id,
        );
    }

    pub fn get_polygon(env: Env, polygon_id: BytesN<32>) -> HabitatPolygon {
        if !has_polygon(&env, &polygon_id) {
            panic_with_error!(&env, MrvOracleError::PolygonNotFound);
        }
        read_polygon(&env, &polygon_id).unwrap()
    }

    pub fn pause(env: Env) {
        let admin = read_admin(&env);
        admin.require_auth();
        write_paused(&env, true);
        env.events().publish(
            (symbol_short!("mrvo"), symbol_short!("paus")),
            (),
        );
    }

    pub fn resume(env: Env) {
        let admin = read_admin(&env);
        admin.require_auth();
        write_paused(&env, false);
        env.events().publish(
            (symbol_short!("mrvo"), symbol_short!("resm")),
            (),
        );
    }

    pub fn paused(env: Env) -> bool {
        read_paused(&env)
    }

    // ── Stubs for Day 6 ──

    pub fn submit_survey(
        env: Env,
        _polygon_id: BytesN<32>,
        _ipfs_cid: Bytes,
        _survey_timestamp: u64,
    ) -> u64 {
        panic_with_error!(&env, MrvOracleError::InvalidSurveyData);
    }

    pub fn dispute(_env: Env, _survey_hash: BytesN<32>) {
        panic_with_error!(&_env, MrvOracleError::SurveyNotFound);
    }

    pub fn resolve_dispute(
        _env: Env,
        _survey_hash: BytesN<32>,
        _outcome: bool,
        _slashed_oracles: soroban_sdk::Vec<BytesN<32>>,
    ) {
        panic_with_error!(&_env, MrvOracleError::SurveyNotFound);
    }
}
