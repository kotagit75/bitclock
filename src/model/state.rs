use crate::{
    model::{address::Address, client::Client, proof::ProofPool},
    util::key::SK,
};

#[derive(Clone)]
pub struct State {
    pub proof_pool: ProofPool,
    pub count: u32,
    pub node_sk: SK,
    pub address: Address,
    pub peers: Vec<Client>,
}
