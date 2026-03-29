use std::cmp::max;
use std::collections::HashSet;

use crate::core::proof::{compare_time, is_valid_proof};
use crate::model::address::Address;
use crate::model::proof::Proof;
use crate::model::proofpool::ProofPool;
use crate::model::stamp::Stamp;
use crate::util::key::SK;
use crate::util::math::median;

fn find_counts_by_address(proof: Proof, address: Address) -> Vec<u32> {
    proof
        .stamps
        .iter()
        .filter(|stamp| stamp.address == address)
        .map(|stamp| stamp.count)
        .collect()
}

impl ProofPool {
    pub fn sort_pool(&self) -> Vec<Proof> {
        let mut vec_proof: Vec<Proof> = self.pool.iter().cloned().collect();
        vec_proof.sort_by(compare_time);
        vec_proof
    }
    pub fn sort_pool_to_time(&self) -> Vec<i64> {
        self.sort_pool().iter().map(|proof| proof.time).collect()
    }
    pub fn get_lastest_proofs(&self, number_of_proofs: usize) -> Vec<Proof> {
        let sorted = self.sort_pool();
        if sorted.len() < number_of_proofs {
            return Vec::new();
        }
        sorted.split_at(sorted.len() - number_of_proofs).1.to_vec()
    }
    pub fn get_lastest_stamp_of_address(&self, address: Address) -> Option<Stamp> {
        self.sort_pool()
            .iter()
            .filter_map(|proof| proof.find_stamp_has_address(address.clone()))
            .last()
    }
    pub fn get_lastest_count_of_address(&self, address: Address) -> u32 {
        if let Some(lastest_stamp) = self.get_lastest_stamp_of_address(address) {
            return lastest_stamp.count;
        }
        0
    }
    pub fn check_proof(&self, proof: Proof) -> bool {
        let addresses: Vec<Address> = proof
            .stamps
            .iter()
            .map(|stamp| stamp.address.clone())
            .collect();
        let is_valid_stamp_counts = addresses.iter().all(|address| {
            let expected_count = self.get_lastest_count_of_address(address.clone()) + 1;
            find_counts_by_address(proof.clone(), address.clone())
                .iter()
                .all(|count| *count == expected_count)
        });

        let recent_time_median = median(
            self.get_lastest_proofs(11)
                .iter()
                .map(|proof| proof.time)
                .collect::<Vec<i64>>()
                .as_slice(),
        );
        let is_valid_time = match recent_time_median {
            Some(median) => proof.time > median,
            None => true,
        };

        is_valid_stamp_counts && is_valid_time
    }
    pub fn add_proof(&self, address: Address, count: u32, proof: Proof) -> (bool, Self, u32) {
        if is_valid_proof(proof.clone()) && self.check_proof(proof.clone()) {
            let mut new_pool = self.pool.clone();
            let is_inserted = new_pool.insert(proof.clone());
            if let Some(my_stamp) = proof.stamps.iter().find(|stamp| stamp.address == address) {
                return (
                    is_inserted,
                    ProofPool { pool: new_pool },
                    max(count, my_stamp.count),
                );
            } else {
                return (is_inserted, ProofPool { pool: new_pool }, count);
            }
        }
        (false, self.clone(), count)
    }
    pub fn update(
        &self,
        address: Address,
        current_count: u32,
        new_pool: &Self,
    ) -> (bool, Self, u32) {
        let mut count = current_count;
        let mut proof_pool = self.clone();
        let mut added = false;
        for proof in new_pool.pool.clone() {
            let (result, new_proof_pool, new_count) =
                proof_pool.add_proof(address.clone(), count, proof);
            added = result || added;
            count = new_count;
            proof_pool = new_proof_pool;
        }
        let diff: HashSet<Proof> = self.pool.difference(&new_pool.pool).cloned().collect();
        if added || !diff.is_empty() {
            return (true, proof_pool, count);
        }
        (false, proof_pool, count)
    }

    pub fn find_by_sk(&self, sk: &SK) -> Option<Proof> {
        self.pool
            .iter()
            .filter(|proof| proof.sk == sk.clone())
            .last()
            .cloned()
    }

    pub fn verify(&self, proof: &Proof) -> bool {
        self.pool.iter().any(|p| p == proof)
    }
}
