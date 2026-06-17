#![no_std]
#![allow(clippy::too_many_arguments)]
use soroban_sdk::{
    contract, contractimpl, panic_with_error, symbol_short, Address, Bytes, BytesN, Env, IntoVal,
    Symbol, Vec,
};

mod errors;
mod storage;
mod types;

#[cfg(test)]
mod test;

pub use crate::errors::MrvOracleError;
use crate::storage::*;
pub use crate::types::{
    BoundingBox, HabitatPolygon, OracleNode, OracleType, ProposeParams, SurveyData, SurveyRecord,
};

#[contract]
pub struct MrvOracleContract;

#[contractimpl]
impl MrvOracleContract {
    pub fn initialize(env: Env, admin: Address) {
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

    pub fn set_approval_gov(env: Env, addr: Address) {
        read_admin(&env).require_auth();
        write_approval_gov(&env, &addr);
    }

    pub fn approval_gov(env: Env) -> Address {
        read_approval_gov(&env)
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

        env.events()
            .publish((symbol_short!("mrvo"), symbol_short!("reg")), pubkey);
    }

    pub fn revoke_oracle(env: Env, pubkey: BytesN<32>) {
        read_admin(&env).require_auth();
        let mut node = read_oracle(&env, &pubkey).unwrap_or_else(|| {
            panic_with_error!(&env, MrvOracleError::OracleNotFound);
        });

        node.active = false;
        write_oracle(&env, &pubkey, &node);

        env.events()
            .publish((symbol_short!("mrvo"), symbol_short!("rev")), pubkey);
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
            panic_with_error!(&env, MrvOracleError::ThresholdNotMet);
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
        env.events()
            .publish((symbol_short!("mrvo"), symbol_short!("poly")), polygon_id);
    }

    pub fn close_polygon(env: Env, polygon_id: BytesN<32>) {
        read_admin(&env).require_auth();
        let mut polygon = read_polygon(&env, &polygon_id).unwrap_or_else(|| {
            panic_with_error!(&env, MrvOracleError::PolygonNotFound);
        });

        polygon.active = false;
        write_polygon(&env, &polygon_id, &polygon);
        env.events()
            .publish((symbol_short!("mrvo"), symbol_short!("close")), polygon_id);
    }

    pub fn get_polygon(env: Env, polygon_id: BytesN<32>) -> HabitatPolygon {
        read_polygon(&env, &polygon_id).unwrap_or_else(|| {
            panic_with_error!(&env, MrvOracleError::PolygonNotFound);
        })
    }

    pub fn pause(env: Env) {
        read_admin(&env).require_auth();
        write_paused(&env, true);
        env.events()
            .publish((symbol_short!("mrvo"), symbol_short!("pause")), ());
    }

    pub fn resume(env: Env) {
        read_admin(&env).require_auth();
        write_paused(&env, false);
        env.events()
            .publish((symbol_short!("mrvo"), symbol_short!("resume")), ());
    }

    pub fn paused(env: Env) -> bool {
        read_paused(&env)
    }

    pub fn submit_survey(env: Env, data: SurveyData) -> BytesN<32> {
        data.beneficiary.require_auth();

        if read_paused(&env) {
            panic_with_error!(&env, MrvOracleError::ContractPaused);
        }

        let mut polygon = read_polygon(&env, &data.polygon_id).unwrap_or_else(|| {
            panic_with_error!(&env, MrvOracleError::PolygonNotFound);
        });

        if !polygon.active {
            panic_with_error!(&env, MrvOracleError::PolygonInactive);
        }

        let (n, _d) = read_threshold(&env);
        if data.signatures.len() < n {
            panic_with_error!(&env, MrvOracleError::ThresholdNotMet);
        }

        // Compute survey hash: polygon_id + ipfs_cid + survey_timestamp
        let mut msg_bytes = Bytes::new(&env);
        msg_bytes.append(&Bytes::from_slice(&env, &data.polygon_id.to_array()));
        msg_bytes.append(&data.ipfs_cid);
        let ts_bytes = data.survey_timestamp.to_be_bytes();
        msg_bytes.append(&Bytes::from_slice(&env, &ts_bytes));

        let survey_hash: BytesN<32> = env.crypto().sha256(&msg_bytes).into();

        if has_survey(&env, &survey_hash) {
            panic_with_error!(&env, MrvOracleError::DuplicateSurvey);
        }

        // Validate signatures
        for i in 0..data.signatures.len() {
            let (pubkey, _sig) = data.signatures.get(i).unwrap();
            let node = read_oracle(&env, &pubkey).unwrap_or_else(|| {
                panic_with_error!(&env, MrvOracleError::InvalidSignature);
            });
            if !node.active {
                panic_with_error!(&env, MrvOracleError::InvalidSignature);
            }
        }

        let survey = SurveyRecord {
            survey_hash: survey_hash.clone(),
            polygon_id: data.polygon_id.clone(),
            ipfs_cid: data.ipfs_cid.clone(),
            survey_timestamp: data.survey_timestamp,
            oracle_count: data.signatures.len(),
            threshold_met: true,
            disputed: false,
            resolved: false,
            token_ids: Vec::new(&env),
            analyses_hashes: data.analyses_hashes,
        };

        write_survey(&env, &survey_hash, &survey);

        // Update polygon
        polygon.last_survey_cid = Some(data.ipfs_cid.clone());
        polygon.last_survey_timestamp = Some(data.survey_timestamp);
        write_polygon(&env, &data.polygon_id, &polygon);

        env.events().publish(
            (symbol_short!("mrvo"), symbol_short!("surv")),
            survey_hash.clone(),
        );

        // Calculate credit_qty = (current_bsi - baseline_bsi) * area_contribution
        let credit_qty = if data.current_bsi > data.baseline_bsi {
            (data.current_bsi - data.baseline_bsi) as u64 * data.area_contribution
        } else {
            0
        };

        if has_approval_gov(&env) {
            let gov_id = read_approval_gov(&env);
            let params = ProposeParams {
                polygon_id: data.polygon_id,
                survey_hash: survey_hash.clone(),
                methodology_id: data.methodology_id,
                credit_qty,
                beneficiary: data.beneficiary.clone(),
                survey_ipfs_cid: data.ipfs_cid,
                baseline_bsi: data.baseline_bsi,
                current_bsi: data.current_bsi,
                area_ha_contribution: data.area_contribution,
                biome: data.biome,
                vintage_year: data.vintage_year,
                vintage_quarter: data.vintage_qtr,
                approval_governance_id: gov_id.clone(),
            };
            let _proposal_id: u64 = env.invoke_contract(
                &gov_id,
                &Symbol::new(&env, "propose"),
                (data.beneficiary, params).into_val(&env),
            );
        }

        survey_hash
    }

    pub fn dispute(env: Env, survey_hash: BytesN<32>) {
        let mut survey = read_survey(&env, &survey_hash).unwrap_or_else(|| {
            panic_with_error!(&env, MrvOracleError::SurveyNotFound);
        });

        if survey.resolved {
            panic_with_error!(&env, MrvOracleError::SurveyAlreadyResolved);
        }

        survey.disputed = true;
        write_survey(&env, &survey_hash, &survey);

        env.events()
            .publish((symbol_short!("mrvo"), symbol_short!("disp")), survey_hash);
    }

    pub fn resolve_dispute(
        env: Env,
        survey_hash: BytesN<32>,
        outcome: bool,
        slashed_oracles: Vec<BytesN<32>>,
    ) {
        read_admin(&env).require_auth();

        let mut survey = read_survey(&env, &survey_hash).unwrap_or_else(|| {
            panic_with_error!(&env, MrvOracleError::SurveyNotFound);
        });

        if survey.resolved {
            panic_with_error!(&env, MrvOracleError::SurveyAlreadyResolved);
        }

        survey.resolved = true;

        if outcome {
            for i in 0..slashed_oracles.len() {
                let pubkey = slashed_oracles.get(i).unwrap();
                if let Some(mut node) = read_oracle(&env, &pubkey) {
                    node.active = false;
                    node.accuracy_score = 0;
                    write_oracle(&env, &pubkey, &node);
                }
            }
        }

        write_survey(&env, &survey_hash, &survey);
        env.events()
            .publish((symbol_short!("mrvo"), symbol_short!("resd")), survey_hash);
    }
}
