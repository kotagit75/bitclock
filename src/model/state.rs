use serde::{Deserialize, Serialize};

use crate::{
    model::{
        address::Address, p2p::Client, proof::UnSignedProof, proofpool::ProofPool, stamp::Stamp,
    },
    util::key::SK,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct State {
    pub proof_pool: ProofPool,
    pub stamp_pool: Vec<Stamp>,
    pub un_signed_proof_pool: Vec<UnSignedProof>,
    pub count: u32,
    pub node_sk: SK,
    pub address: Address,
    pub peers: Vec<Client>,
}
