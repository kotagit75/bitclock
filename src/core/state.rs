use crate::model::{proof::ProofPool, state::State};

impl State {
    pub fn update_pool_and_count(&self, (pool, count): (ProofPool, u32)) -> Self {
        State {
            proof_pool: pool,
            count: count,
            node_sk: self.node_sk.clone(),
            address: self.address.clone(),
            peers: self.peers.clone(),
        }
    }
}
