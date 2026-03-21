use crate::model::{proof::ProofPool, state::State};

impl State {
    pub fn update_pool_and_count(&self, (pool, count): (ProofPool, u32)) -> Self {
        State {
            proof_pool: pool,
            stamp_pool: self.stamp_pool.clone(),
            un_signed_proof: self.un_signed_proof.clone(),
            count: count,
            node_sk: self.node_sk.clone(),
            address: self.address.clone(),
            peers: self.peers.clone(),
        }
    }
}
