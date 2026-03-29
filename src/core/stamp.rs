use sha2::Digest;

use crate::{
    core::signature::{sign, verify},
    model::{address::Address, signature::Signature, stamp::Stamp},
    util::key::{PK, SK},
};

impl Stamp {
    pub fn to_buf_for_sign(&self) -> Result<Vec<u8>, ()> {
        stamp_to_buf_for_sign(&self.address, self.count, &self.pk, self.nonce, self.id)
    }

    pub fn verify_sign(&self) -> bool {
        self.to_buf_for_sign()
            .map(|buf| verify(&buf, self.address.clone(), self.sign.clone()))
            .is_ok()
    }
}
fn stamp_to_buf_for_sign(
    address: &Address,
    count: u32,
    pk: &PK,
    nonce: u32,
    id: usize,
) -> Result<Vec<u8>, ()> {
    let Ok(address_buf) = address.key().public_key_to_der() else {
        return Err(());
    };
    let count_buf = count.to_be_bytes();
    let Ok(pk_buf) = pk.key().public_key_to_der() else {
        return Err(());
    };
    let nonce_buf = nonce.to_be_bytes();
    let id_buf = id.to_be_bytes();
    Ok(vec![
        address_buf,
        count_buf.to_vec(),
        pk_buf,
        nonce_buf.to_vec(),
        id_buf.to_vec(),
    ]
    .concat())
}
fn stamp_to_buf_for_nonce(
    address: &Address,
    count: u32,
    pk: &PK,
    nonce: u32,
    id: usize,
) -> Result<Vec<u8>, ()> {
    let Ok(memo) = stamp_to_buf_for_nonce_memo(address, count, pk, id) else {
        return Err(());
    };
    let nonce_buf = nonce.to_be_bytes();
    Ok([memo, nonce_buf.to_vec()].concat())
}
fn stamp_to_buf_for_nonce_memo(
    address: &Address,
    count: u32,
    pk: &PK,
    id: usize,
) -> Result<Vec<u8>, ()> {
    let Ok(address_buf) = address.key().public_key_to_der() else {
        return Err(());
    };
    let count_buf = count.to_be_bytes();
    let Ok(pk_buf) = pk.key().public_key_to_der() else {
        return Err(());
    };
    let id_buf = id.to_be_bytes();
    Ok([address_buf, count_buf.to_vec(), pk_buf, id_buf.to_vec()].concat())
}

pub fn create_sign_to_stamp(
    node_sk: SK,
    address: &Address,
    count: u32,
    pk: PK,
    nonce: u32,
    id: usize,
) -> Result<Signature, ()> {
    match stamp_to_buf_for_sign(address, count, &pk, nonce, id).and_then(|buf| sign(&buf, node_sk))
    {
        Ok(sign) => Ok(sign),
        Err(_) => Err(()),
    }
}

fn nonce_start_with_str(difficulty: usize) -> String {
    "0".repeat(difficulty)
}
pub fn verify_nonce(
    difficulty: usize,
    address: &Address,
    count: u32,
    pk: &PK,
    nonce: u32,
    id: usize,
) -> bool {
    stamp_to_buf_for_nonce(address, count, pk, nonce, id)
        .map(|buf| {
            hex::encode(sha2::Sha256::digest(buf)).starts_with(&nonce_start_with_str(difficulty))
        })
        .is_ok()
}
pub fn verify_nonce_for_calc(memo: &[u8], nonce: u32, starts_with: &str) -> bool {
    hex::encode(sha2::Sha256::digest(
        vec![memo, &nonce.to_be_bytes()].concat(),
    ))
    .starts_with(starts_with)
}

pub fn calc_nonce(difficulty: usize, address: &Address, count: u32, pk: &PK, id: usize) -> u32 {
    let start_with = &nonce_start_with_str(difficulty);
    let memo = stamp_to_buf_for_nonce_memo(address, count, pk, id).unwrap();
    let mut nonce = 0;
    while !verify_nonce_for_calc(&memo, nonce, start_with) {
        nonce += 1
    }
    nonce
}

pub fn is_valid_stamp(stamp: &Stamp, difficulty: usize, proof_pk: PK) -> bool {
    let is_valid_pk = stamp.pk == proof_pk;
    let is_valid_nonce = verify_nonce(
        difficulty,
        &stamp.address,
        stamp.count,
        &stamp.pk,
        stamp.nonce,
        stamp.id,
    );
    let is_valid_sign = stamp.verify_sign();
    is_valid_pk && is_valid_nonce && is_valid_sign
}

pub fn sum_of_count(stamps: Vec<Stamp>) -> u32 {
    stamps.iter().map(|stamp| stamp.count).sum()
}
