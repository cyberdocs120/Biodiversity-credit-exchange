#![no_std]
mod storage;
mod types;
mod errors;
#[cfg(test)]
mod test;

use soroban_sdk::{contract, contractimpl, panic_with_error, symbol_short, Address, Bytes, BytesN, Env, Vec};
use storage::*;
use types::*;
use errors::BdcTokenError;

#[contract]
pub struct BdcTokenContract;

#[contractimpl]
impl BdcTokenContract {
    pub fn __constructor(env: Env, admin: Address) {
        admin.require_auth();
        write_admin(&env, &admin);
        write_token_id_counter(&env, 0);
        write_total_supply(&env, 0);
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

    pub fn total_supply(env: Env) -> u64 {
        read_total_supply(&env)
    }

    pub fn balance_of(env: Env, owner: Address) -> u64 {
        read_owner_count(&env, &owner)
    }

    pub fn owner_of(env: Env, token_id: u64) -> Address {
        if !has_token(&env, token_id) {
            panic_with_error!(&env, BdcTokenError::TokenNotFound);
        }
        let token = read_token(&env, token_id).unwrap();
        token.owner
    }

    pub fn token_uri(env: Env, token_id: u64) -> Bytes {
        if !has_token(&env, token_id) {
            panic_with_error!(&env, BdcTokenError::TokenNotFound);
        }
        let token = read_token(&env, token_id).unwrap();
        token.metadata.metadata_uri
    }

    pub fn token_metadata(env: Env, token_id: u64) -> BdcMetadata {
        if !has_token(&env, token_id) {
            panic_with_error!(&env, BdcTokenError::TokenNotFound);
        }
        let token = read_token(&env, token_id).unwrap();
        token.metadata
    }

    pub fn authorize_minter(env: Env, minter: Address) {
        let admin = read_admin(&env);
        admin.require_auth();
        write_authorized_minter(&env, &minter);
    }

    pub fn authorize_burner(env: Env, burner: Address) {
        let admin = read_admin(&env);
        admin.require_auth();
        write_authorized_burner(&env, &burner);
    }

    pub fn revoke_minter(env: Env) {
        let admin = read_admin(&env);
        admin.require_auth();
        env.storage().instance().remove(&authorized_minter_key());
    }

    pub fn revoke_burner(env: Env) {
        let admin = read_admin(&env);
        admin.require_auth();
        env.storage().instance().remove(&authorized_burner_key());
    }

    pub fn mint(env: Env, to: Address, params: MintParams) -> u64 {
        env.current_contract_address().require_auth();

        let token_id = read_token_id_counter(&env) + 1;
        write_token_id_counter(&env, token_id);

        let metadata = BdcMetadata {
            token_id,
            polygon_id: params.polygon_id.clone(),
            methodology_id: params.methodology_id,
            survey_ipfs_cid: params.survey_ipfs_cid.clone(),
            baseline_bsi: params.baseline_bsi,
            current_bsi: params.current_bsi,
            area_ha_contribution: params.area_ha_contribution,
            biome: params.biome,
            vintage_year: params.vintage_year,
            vintage_quarter: params.vintage_quarter,
            approval_governance_id: params.approval_governance_id,
            metadata_uri: Bytes::new(&env),
            state: BdcState::Active,
            retired_at: None,
            retirement_receipt: None,
        };

        let token = BdcTokenValue {
            owner: to.clone(),
            metadata,
        };

        write_token(&env, token_id, &token);
        write_owner_count(&env, &to, read_owner_count(&env, &to) + 1);
        write_total_supply(&env, read_total_supply(&env) + 1);
        write_polygon_token_count(&env, &params.polygon_id, read_polygon_token_count(&env, &params.polygon_id) + 1);

        env.events().publish(
            (symbol_short!("bdc"), symbol_short!("mint")),
            (token_id, params.polygon_id, params.survey_ipfs_cid, 1u64),
        );

        token_id
    }

    pub fn transfer(env: Env, from: Address, to: Address, token_id: u64) {
        from.require_auth();

        if !has_token(&env, token_id) {
            panic_with_error!(&env, BdcTokenError::TokenNotFound);
        }

        let mut token = read_token(&env, token_id).unwrap();
        if token.owner != from {
            panic_with_error!(&env, BdcTokenError::Unauthorized);
        }
        if token.metadata.state != BdcState::Active {
            panic_with_error!(&env, BdcTokenError::BdcAlreadyRetired);
        }

        token.owner = to.clone();
        write_token(&env, token_id, &token);

        write_owner_count(&env, &from, read_owner_count(&env, &from) - 1);
        write_owner_count(&env, &to, read_owner_count(&env, &to) + 1);

        env.events().publish(
            (symbol_short!("bdc"), symbol_short!("xfer")),
            (token_id, from, to),
        );
    }

    pub fn burn(env: Env, caller: Address, token_id: u64) {
        caller.require_auth();

        if !has_token(&env, token_id) {
            panic_with_error!(&env, BdcTokenError::TokenNotFound);
        }

        let mut token = read_token(&env, token_id).unwrap();
        if token.metadata.state != BdcState::Active {
            panic_with_error!(&env, BdcTokenError::BdcAlreadyRetired);
        }

        token.metadata.state = BdcState::Retired;
        token.metadata.retired_at = Some(env.ledger().timestamp());
        write_token(&env, token_id, &token);

        write_owner_count(&env, &token.owner, read_owner_count(&env, &token.owner) - 1);
        write_total_supply(&env, read_total_supply(&env) - 1);

        env.events().publish(
            (symbol_short!("bdc"), symbol_short!("burn")),
            (token_id, caller),
        );
    }

    pub fn set_metadata_uri(env: Env, token_id: u64, new_uri: Bytes) {
        let admin = read_admin(&env);
        admin.require_auth();

        if !has_token(&env, token_id) {
            panic_with_error!(&env, BdcTokenError::TokenNotFound);
        }

        let mut token = read_token(&env, token_id).unwrap();
        token.metadata.metadata_uri = new_uri;
        write_token(&env, token_id, &token);
    }

    pub fn tokens_by_owner(env: Env, owner: Address, start: u64, limit: u64) -> Vec<u64> {
        let counter = read_token_id_counter(&env);
        let mut result: Vec<u64> = Vec::new(&env);
        let mut skipped: u64 = 0;

        for token_id in 1..=counter {
            if !has_token(&env, token_id) {
                continue;
            }
            let token = read_token(&env, token_id).unwrap();
            if token.owner == owner && token.metadata.state == BdcState::Active {
                if skipped < start {
                    skipped += 1;
                } else if u64::from(result.len()) < limit {
                    result.push_back(token_id);
                } else {
                    break;
                }
            }
        }

        result
    }

    pub fn tokens_by_polygon(env: Env, polygon_id: BytesN<32>, start: u64, limit: u64) -> Vec<u64> {
        let counter = read_token_id_counter(&env);
        let mut result: Vec<u64> = Vec::new(&env);
        let mut skipped: u64 = 0;

        for token_id in 1..=counter {
            if !has_token(&env, token_id) {
                continue;
            }
            let token = read_token(&env, token_id).unwrap();
            if token.metadata.polygon_id == polygon_id && token.metadata.state == BdcState::Active {
                if skipped < start {
                    skipped += 1;
                } else if u64::from(result.len()) < limit {
                    result.push_back(token_id);
                } else {
                    break;
                }
            }
        }

        result
    }
}
