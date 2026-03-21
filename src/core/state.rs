use crate::{
    model::{proof::ProofPool, stamp::Stamp, state::State},
    util::key::PK,
};

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
    pub fn add_to_stamp_pool(&self, pk: PK, stamp: Stamp) -> Self {
        if self.un_signed_proof.get(&pk).is_none() {
            return self.clone();
        }
        let mut new_stamp_pool = self.stamp_pool.clone();
        new_stamp_pool.insert(pk, stamp);
        State {
            proof_pool: self.proof_pool.clone(),
            stamp_pool: new_stamp_pool,
            un_signed_proof: self.un_signed_proof.clone(),
            count: self.count,
            node_sk: self.node_sk.clone(),
            address: self.address.clone(),
            peers: self.peers.clone(),
        }
    }
}
