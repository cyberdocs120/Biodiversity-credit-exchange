#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Bytes, BytesN, Env, panic_with_error};

mod errors;
mod storage;
mod types;

#[cfg(test)]
mod test;

pub use crate::errors::MrvOracleError;
pub use crate::types::{OracleNode, OracleType, HabitatPolygon, SurveyRecord, BoundingBox};
use crate::storage::*;

#[contract]
pub struct MrvOracleContract;

#[contractimpl]
impl MrvOracleContract {
    pub fn __constructor(env: Env, admin: Address) {
        write_admin(&env, &admin);
        write_paused(&env, false);
        write_threshold(&env, 1, 1);
        write_oracle_count(&env, 0);
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

    pub fn register_oracle(env: Env, pubkey: BytesN<32>, uri: Bytes, oracle_type: OracleType) {
        read_admin(&env).require_auth();
        if has_oracle(&env, &pubkey) {
            panic_with_error!(&env, MrvOracleError::OracleAlreadyRegistered);
        }

        let node = OracleNode {
            pubkey: pubkey.clone(),
            uri,
            oracle_type,
            active: true,
            registered_at: env.ledger().timestamp(),
            total_surveys: 0,
            accuracy_score: 100,
        };

        write_oracle(&env, &pubkey, &node);
        let count = read_oracle_count(&env) + 1;
        write_oracle_count(&env, count);

        env.events().publish((symbol_short!("mrvo"), symbol_short!("reg")), pubkey);
    }

    pub fn revoke_oracle(env: Env, pubkey: BytesN<32>) {
        read_admin(&env).require_auth();
        let mut node = read_oracle(&env, &pubkey).unwrap_or_else(|| {
            panic_with_error!(&env, MrvOracleError::OracleNotFound);
        });

        node.active = false;
        write_oracle(&env, &pubkey, &node);

        env.events().publish((symbol_short!("mrvo"), symbol_short!("rev")), pubkey);
    }

    pub fn oracle_count(env: Env) -> u32 {
        read_oracle_count(&env)
    }

    pub fn get_oracle(env: Env, pubkey: BytesN<32>) -> OracleNode {
        read_oracle(&env, &pubkey).unwrap_or_else(|| {
            panic_with_error!(&env, MrvOracleError::OracleNotFound);
        })
    }

    pub fn set_threshold(env: Env, n: u32, d: u32) {
        read_admin(&env).require_auth();
        if n == 0 || d == 0 || n > d {
            panic_with_error!(&env, MrvOracleError::ThresholdNotMet); // Or a more specific error if available
        }
        write_threshold(&env, n, d);
    }

    pub fn threshold(env: Env) -> (u32, u32) {
        read_threshold(&env)
    }

    pub fn register_polygon(
        env: Env,
        polygon_id: BytesN<32>,
        geometry_cid: Bytes,
        bbox: BoundingBox,
        area_ha: u64,
        biome: u32,
        country: BytesN<2>,
        project_id: BytesN<32>,
    ) {
        read_admin(&env).require_auth();
        
        let polygon = HabitatPolygon {
            polygon_id: polygon_id.clone(),
            geometry_ipfs_cid: geometry_cid,
            bounding_box: bbox,
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
        env.events().publish((symbol_short!("mrvo"), symbol_short!("poly")), polygon_id);
    }

    pub fn close_polygon(env: Env, polygon_id: BytesN<32>) {
        read_admin(&env).require_auth();
        let mut polygon = read_polygon(&env, &polygon_id).unwrap_or_else(|| {
            panic_with_error!(&env, MrvOracleError::PolygonNotFound);
        });

        polygon.active = false;
        write_polygon(&env, &polygon_id, &polygon);
        env.events().publish((symbol_short!("mrvo"), symbol_short!("close")), polygon_id);
    }

    pub fn get_polygon(env: Env, polygon_id: BytesN<32>) -> HabitatPolygon {
        read_polygon(&env, &polygon_id).unwrap_or_else(|| {
            panic_with_error!(&env, MrvOracleError::PolygonNotFound);
        })
    }

    pub fn pause(env: Env) {
        read_admin(&env).require_auth();
        write_paused(&env, true);
        env.events().publish((symbol_short!("mrvo"), symbol_short!("pause")), ());
    }

    pub fn resume(env: Env) {
        read_admin(&env).require_auth();
        write_paused(&env, false);
        env.events().publish((symbol_short!("mrvo"), symbol_short!("resume")), ());
    }

    pub fn paused(env: Env) -> bool {
        read_paused(&env)
    }

    pub fn submit_survey(_env: Env) {
        panic!("not implemented");
    }

    pub fn dispute(_env: Env) {
        panic!("not implemented");
    }

    pub fn resolve_dispute(_env: Env) {
        panic!("not implemented");
    }
}
