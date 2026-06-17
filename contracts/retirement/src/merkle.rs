#![allow(dead_code)]
use soroban_sdk::{Bytes, BytesN, Env, Vec};

pub fn compute_root(env: &Env, token_ids: &Vec<u64>) -> BytesN<32> {
    if token_ids.is_empty() {
        return BytesN::from_array(env, &[0u8; 32]);
    }

    let mut leaves: Vec<BytesN<32>> = Vec::new(env);
    for i in 0..token_ids.len() {
        let id = token_ids.get(i).unwrap();
        let bytes = id.to_be_bytes();
        leaves.push_back(env.crypto().sha256(&Bytes::from_slice(env, &bytes)).into());
    }

    compute_merkle_root(env, leaves)
}

fn compute_merkle_root(env: &Env, nodes: Vec<BytesN<32>>) -> BytesN<32> {
    if nodes.len() == 1 {
        return nodes.get(0).unwrap();
    }

    let mut next_level: Vec<BytesN<32>> = Vec::new(env);
    for i in (0..nodes.len()).step_by(2) {
        let left = nodes.get(i).unwrap();
        let right = if i + 1 < nodes.len() {
            nodes.get(i + 1).unwrap()
        } else {
            left.clone()
        };

        let mut hash_input = Bytes::new(env);
        hash_input.append(&Bytes::from_slice(env, &left.to_array()));
        hash_input.append(&Bytes::from_slice(env, &right.to_array()));
        next_level.push_back(env.crypto().sha256(&hash_input).into());
    }

    compute_merkle_root(env, next_level)
}

pub fn generate_proof(env: &Env, token_ids: &Vec<u64>, leaf_index: u32) -> Vec<BytesN<32>> {
    let mut leaves: Vec<BytesN<32>> = Vec::new(env);
    for i in 0..token_ids.len() {
        let id = token_ids.get(i).unwrap();
        let bytes = id.to_be_bytes();
        leaves.push_back(env.crypto().sha256(&Bytes::from_slice(env, &bytes)).into());
    }

    let mut proof: Vec<BytesN<32>> = Vec::new(env);
    get_proof_recursive(env, leaves, leaf_index, &mut proof);
    proof
}

fn get_proof_recursive(env: &Env, nodes: Vec<BytesN<32>>, index: u32, proof: &mut Vec<BytesN<32>>) {
    if nodes.len() <= 1 {
        return;
    }

    let mut next_level: Vec<BytesN<32>> = Vec::new(env);
    for i in (0..nodes.len()).step_by(2) {
        let left = nodes.get(i).unwrap();
        let right = if i + 1 < nodes.len() {
            nodes.get(i + 1).unwrap()
        } else {
            left.clone()
        };

        if i == index || i + 1 == index {
            if i == index {
                proof.push_back(right.clone());
            } else {
                proof.push_back(left.clone());
            }
        }

        let mut hash_input = Bytes::new(env);
        hash_input.append(&Bytes::from_slice(env, &left.to_array()));
        hash_input.append(&Bytes::from_slice(env, &right.to_array()));
        next_level.push_back(env.crypto().sha256(&hash_input).into());
    }

    get_proof_recursive(env, next_level, index / 2, proof);
}

pub fn verify(
    env: &Env,
    root: &BytesN<32>,
    proof: &Vec<BytesN<32>>,
    leaf: &BytesN<32>,
    index: u32,
) -> bool {
    let mut current_hash = leaf.clone();
    let mut current_index = index;

    for i in 0..proof.len() {
        let sibling = proof.get(i).unwrap();
        let mut hash_input = Bytes::new(env);
        if current_index.is_multiple_of(2) {
            hash_input.append(&Bytes::from_slice(env, &current_hash.to_array()));
            hash_input.append(&Bytes::from_slice(env, &sibling.to_array()));
        } else {
            hash_input.append(&Bytes::from_slice(env, &sibling.to_array()));
            hash_input.append(&Bytes::from_slice(env, &current_hash.to_array()));
        }
        current_hash = env.crypto().sha256(&hash_input).into();
        current_index /= 2;
    }

    &current_hash == root
}
