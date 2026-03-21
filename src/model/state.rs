use std::collections::HashMap;

use crate::{
    model::{
        address::Address,
        client::Client,
        proof::{ProofPool, UnSignedProof},
        stamp::Stamp,
    },
    util::key::{PK, SK},
};

#[derive(Clone)]
pub struct State {
    pub proof_pool: ProofPool,
    pub stamp_pool: HashMap<PK, Stamp>,
    pub un_signed_proof: HashMap<PK, UnSignedProof>,
    pub count: u32,
    pub node_sk: SK,
    pub address: Address,
    pub peers: Vec<Client>,
}
