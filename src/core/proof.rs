use std::cmp::{Ordering, max};
use std::collections::HashSet;

use openssl::hash::MessageDigest;
use openssl::sign::{Signer, Verifier};

use crate::core::stamp::sum_of_count;
use crate::core::stamp::{is_same_stamps, is_valid_stamp};
use crate::model::address::Address;
use crate::model::proof::{Proof, ProofPool, UnSignedProof};
use crate::model::signature::Signature;
use crate::model::stamp::Stamp;
use crate::util::key::{PK, SK, generate_pk_and_sk};
use crate::util::math::median;

pub const PROOF_KEY_BITS: u32 = 512;

impl PartialEq for Proof {
    fn eq(&self, other: &Self) -> bool {
        compare_time(self, other) == Ordering::Equal
    }
}
impl PartialOrd for Proof {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(compare_time(self, other))
    }
}
impl Eq for Proof {}
impl Ord for Proof {
    fn cmp(&self, other: &Self) -> Ordering {
        compare_time(self, other)
    }
}

impl Proof {
    pub fn to_buf_for_sign(&self) -> Result<Vec<u8>, ()> {
        proof_to_buf_for_sign(
            self.data.clone(),
            self.stamps.clone(),
            self.sk.clone(),
            self.address.clone(),
            self.difficulty,
            self.time,
        )
    }

    pub fn verify_sign(&self) -> bool {
        let Ok(buf) = self.to_buf_for_sign() else {
            return false;
        };

        let key = self.address.key();
        let Ok(mut verifyer) = Verifier::new(MessageDigest::sha256(), &key) else {
            return false;
        };
        let Ok(_) = verifyer.update(&buf) else {
            return false;
        };
        let Ok(result) = verifyer.verify(&self.sign) else {
            return false;
        };
        result
    }
}
impl Proof {
    pub fn get_proof_pk(&self) -> Result<PK, ()> {
        self.sk.to_pk()
    }

    pub fn find_stamp_has_address(&self, address: Address) -> Option<Stamp> {
        self.stamps
            .iter()
            .find(|stamp| stamp.address == address)
            .cloned()
    }
}

fn proof_to_buf_for_sign(
    data: String,
    stamps: Vec<Stamp>,
    sk: SK,
    address: Address,
    difficulty: usize,
    time: i64,
) -> Result<Vec<u8>, ()> {
    let data_buf = data.as_bytes().to_vec();
    let stamp_buf = {
        let stamp_bufs: Vec<Vec<u8>> = stamps
            .iter()
            .map(|stamp| stamp.to_buf_for_sign())
            .filter_map(Result::ok)
            .collect();
        stamp_bufs
    }
    .concat();
    let Ok(sk_buf) = sk.key().private_key_to_der() else {
        return Err(());
    };
    let Ok(address_buf) = address.key().public_key_to_der() else {
        return Err(());
    };
    let difficulty_buf = difficulty.to_be_bytes();
    let time_buf = time.to_be_bytes();
    Ok(vec![
        data_buf,
        stamp_buf,
        sk_buf,
        address_buf,
        difficulty_buf.to_vec(),
        time_buf.to_vec(),
    ]
    .concat())
}
pub fn create_sign_to_proof(
    node_sk: SK,
    data: String,
    stamps: Vec<Stamp>,
    sk: SK,
    address: Address,
    difficulty: usize,
    time: i64,
) -> Result<Signature, ()> {
    let Ok(buf) = proof_to_buf_for_sign(data, stamps, sk, address, difficulty, time) else {
        return Err(());
    };
    let Ok(mut signer) = Signer::new(MessageDigest::sha256(), &node_sk.key()) else {
        return Err(());
    };
    let Ok(_) = signer.update(&buf) else {
        return Err(());
    };
    let Ok(sign) = signer.sign_to_vec() else {
        return Err(());
    };
    Ok(sign)
}

pub fn calc_number_of_stamps() -> usize {
    1000
}

pub fn is_valid_proof(proof: Proof) -> bool {
    let Ok(proof_pk) = proof.get_proof_pk() else {
        return false;
    };
    let is_valid_stamps = proof
        .stamps
        .iter()
        .all(|stamp| is_valid_stamp(stamp, proof.difficulty, proof_pk.clone()));
    let is_valid_number_of_stamps = proof.stamps.len() >= calc_number_of_stamps();
    let is_not_duplicated_stamps = {
        let mut dedup_stamps: Vec<Stamp> = proof.stamps.clone();
        dedup_stamps
            .dedup_by(|stamp1: &mut Stamp, stamp2: &mut Stamp| is_same_stamps(stamp1, stamp2));
        dedup_stamps.len() == proof.stamps.len()
    };
    let is_valid_sign = proof.verify_sign();
    is_valid_stamps && is_valid_number_of_stamps && is_not_duplicated_stamps && is_valid_sign
}

pub fn compare_time(proof1: &Proof, proof2: &Proof) -> Ordering {
    let proof1_addresses = proof1.stamps.iter().map(|stamp| stamp.address.clone());
    let proof2_addresses = proof2.stamps.iter().map(|stamp| stamp.address.clone());
    let proof1_duplicated_stamps: Vec<Stamp> = proof2_addresses
        .map(|address| {
            proof1
                .stamps
                .iter()
                .filter(|stamp| stamp.address == address)
                .max_by_key(|stamp| stamp.count)
                .cloned()
        })
        .flatten()
        .collect();
    let proof2_duplicated_stamps: Vec<Stamp> = proof1_addresses
        .map(|address| {
            proof2
                .stamps
                .iter()
                .filter(|stamp| stamp.address == address)
                .max_by_key(|stamp| stamp.count)
                .cloned()
        })
        .flatten()
        .collect();
    sum_of_count(proof1_duplicated_stamps).cmp(&sum_of_count(proof2_duplicated_stamps))
}

impl UnSignedProof {
    pub fn new(data: String, sk: SK, address: Address, difficulty: usize, time: i64) -> Self {
        Proof {
            data,
            stamps: Vec::new(),
            sk,
            address,
            difficulty,
            time,
            sign: Vec::new(),
        }
    }
    pub fn create(
        data: String,
        address: Address,
        time: i64,
        proof_pool: &ProofPool,
    ) -> Result<(Self, PK), ()> {
        let Ok((pk, sk)) = generate_pk_and_sk(PROOF_KEY_BITS) else {
            return Err(());
        };
        let difficulty = proof_pool.calc_difficulty(time);
        Ok((
            UnSignedProof::new(data, sk, address.clone(), difficulty, time),
            pk,
        ))
    }

    pub fn create_signed_proof(&self, node_sk: SK, stamps: Vec<Stamp>) -> Result<Proof, ()> {
        let Ok(sign) = create_sign_to_proof(
            node_sk,
            self.data.clone(),
            stamps.clone(),
            self.sk.clone(),
            self.address.clone(),
            self.difficulty,
            self.time,
        ) else {
            return Err(());
        };
        let mut new_proof = self.clone();
        new_proof.stamps = stamps;
        new_proof.sign = sign;
        Ok(new_proof)
    }
}

fn find_counts_by_address(proof: Proof, address: Address) -> Vec<u32> {
    proof
        .stamps
        .iter()
        .filter(|stamp| stamp.address == address)
        .map(|stamp| stamp.count)
        .collect()
}

impl ProofPool {
    pub fn new() -> Self {
        ProofPool {
            pool: HashSet::new(),
        }
    }

    pub fn sort_pool(&self) -> Vec<Proof> {
        let mut vec_proof: Vec<Proof> = self.pool.iter().map(|p| p.clone()).collect();
        vec_proof.sort_by(|proof1, proof2| compare_time(proof1, proof2));
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
            .map(|proof| proof.find_stamp_has_address(address.clone()))
            .flatten()
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
                .all(|count| count.clone() == expected_count)
        });

        let recent_time_median = median(
            &self
                .get_lastest_proofs(11)
                .iter()
                .map(|proof| proof.time)
                .collect(),
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
        if added || diff.len() > 0 {
            return (true, proof_pool, count);
        }
        (false, proof_pool, count)
    }

    pub fn find_by_sk(&self, sk: &SK) -> Vec<Proof> {
        self.pool
            .iter()
            .filter(|proof| proof.sk == sk.clone())
            .cloned()
            .collect()
    }
}
