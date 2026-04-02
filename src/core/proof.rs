use std::cmp::Ordering;

use crate::core::signature::{sign, verify};
use crate::core::stamp::is_valid_stamp;
use crate::core::stamp::sum_of_count;
use crate::model::address::Address;
use crate::model::data::Data;
use crate::model::proof::{Proof, UnSignedProof};
use crate::model::proofpool::ProofPool;
use crate::model::signature::Signature;
use crate::model::stamp::Stamp;
use crate::util::key::{PK, SK, generate_pk_and_sk};

pub const PROOF_KEY_BITS: u32 = 512;

impl Proof {
    pub fn to_buf_for_sign(&self) -> Vec<u8> {
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
        verify(
            &self.to_buf_for_sign(),
            self.address.clone(),
            self.sign.clone(),
        )
        .is_ok()
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
    data: Data,
    stamps: Vec<Stamp>,
    sk: SK,
    address: Address,
    difficulty: usize,
    time: i64,
) -> Vec<u8> {
    return format!(
        "{:?} {:?} {:?} {:?} {:?} {:?}",
        data.to_string(),
        {
            let stamp_bufs: Vec<Vec<u8>> = stamps
                .iter()
                .map(|stamp| stamp.to_buf_for_sign())
                .filter_map(Result::ok)
                .collect();
            stamp_bufs
        }
        .concat(),
        sk.der,
        address.der,
        difficulty,
        time
    )
    .as_bytes()
    .to_vec();
}
pub fn create_sign_to_proof(
    node_sk: SK,
    data: Data,
    stamps: Vec<Stamp>,
    sk: SK,
    address: Address,
    difficulty: usize,
    time: i64,
) -> Result<Signature, ()> {
    match sign(
        &proof_to_buf_for_sign(data, stamps, sk, address, difficulty, time),
        node_sk,
    ) {
        Ok(sign) => Ok(sign),
        Err(_) => Err(()),
    }
}

pub fn calc_number_of_stamps() -> usize {
    10
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
        dedup_stamps.dedup();
        dedup_stamps.len() == proof.stamps.len()
    };
    let is_valid_data_sign = proof.data.verify_sign();
    let is_valid_sign = proof.verify_sign();
    is_valid_stamps
        && is_valid_number_of_stamps
        && is_not_duplicated_stamps
        && is_valid_sign
        && is_valid_data_sign
}

pub fn compare_time(proof1: &Proof, proof2: &Proof) -> Ordering {
    let proof1_addresses = proof1.stamps.iter().map(|stamp| stamp.address.clone());
    let proof2_addresses = proof2.stamps.iter().map(|stamp| stamp.address.clone());
    let proof1_duplicated_stamps: Vec<Stamp> = proof2_addresses
        .filter_map(|address| {
            proof1
                .stamps
                .iter()
                .filter(|stamp| stamp.address == address)
                .max_by_key(|stamp| stamp.count)
                .cloned()
        })
        .collect();
    let proof2_duplicated_stamps: Vec<Stamp> = proof1_addresses
        .filter_map(|address| {
            proof2
                .stamps
                .iter()
                .filter(|stamp| stamp.address == address)
                .max_by_key(|stamp| stamp.count)
                .cloned()
        })
        .collect();
    sum_of_count(proof1_duplicated_stamps).cmp(&sum_of_count(proof2_duplicated_stamps))
}

impl UnSignedProof {
    pub fn new(data: Data, sk: SK, address: Address, difficulty: usize, time: i64) -> Self {
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
        data: Data,
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

#[cfg(test)]
mod tests {
    use crate::util::key::generate_pk_and_sk;

    use super::*;

    fn data_for_test(address: &Address) -> Data {
        Data {
            recipient: address.clone(),
            issuer: address.clone(),
            content: "test".to_string(),
            sign: Vec::new(),
        }
    }

    #[test]
    fn test_create_sign_to_proof() {
        let (address, node_sk) = generate_pk_and_sk(512).unwrap();
        let (_, sk) = generate_pk_and_sk(512).unwrap();
        let data = data_for_test(&address);
        let Ok(signature) = create_sign_to_proof(
            node_sk.clone(),
            data.clone(),
            Vec::new(),
            sk.clone(),
            address.clone(),
            0,
            0,
        ) else {
            panic!();
        };
        assert_eq!(
            sign(
                &proof_to_buf_for_sign(data, Vec::new(), sk, address, 0, 0),
                node_sk
            )
            .unwrap(),
            signature
        );
    }

    #[test]
    fn test_verify_sign() {
        let (address, node_sk) = generate_pk_and_sk(512).unwrap();
        let (_, sk) = generate_pk_and_sk(512).unwrap();
        let data = data_for_test(&address);
        let Ok(signature) = create_sign_to_proof(
            node_sk.clone(),
            data.clone(),
            Vec::new(),
            sk.clone(),
            address.clone(),
            0,
            0,
        ) else {
            panic!();
        };
        let proof = Proof {
            data: data.clone(),
            stamps: Vec::new(),
            sk: sk.clone(),
            address: address.clone(),
            difficulty: 0,
            time: 0,
            sign: signature.clone(),
        };
        assert!(
            verify(
                &proof_to_buf_for_sign(data, Vec::new(), sk, address.clone(), 0, 0),
                address,
                signature
            )
            .is_ok()
                && proof.verify_sign()
        );
    }
}
