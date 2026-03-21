use crate::model::{proof::ProofPool, state::State};

impl State {
    pub fn update_pool(&self, pool: ProofPool) -> Self {
        State {
            proof_pool: pool,
            count: self.count,
            node_sk: self.node_sk.clone(),
            address: self.address.clone(),
            peers: self.peers.clone(),
        }
    }
}
