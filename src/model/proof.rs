use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{
    model::{address::Address, signature::Signature, stamp::Stamp},
    util::key::SK,
};

#[derive(Clone, Debug, Serialize, Deserialize, Hash)]
pub struct Proof {
    pub data: String,
    pub stamps: Vec<Stamp>,
    pub sk: SK,
    pub address: Address,
    pub difficulty: usize,
    pub time: i64,
    pub sign: Signature,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProofPool {
    pub pool: HashSet<Proof>,
}
pub type UnSignedProof = Proof;
