use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{
    model::{address::Address, data::Data, signature::Signature, stamp::Stamp},
    util::key::SK,
};

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Proof {
    pub data: Data,
    pub stamps: Vec<Stamp>,
    pub sk: SK,
    pub address: Address,
    pub difficulty: usize,
    pub time: i64,
    pub sign: Signature,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ProofPool {
    pub pool: HashSet<Proof>,
}
pub type UnSignedProof = Proof;
