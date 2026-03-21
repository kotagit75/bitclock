use crate::{
    model::{
        proof::{ProofPool, UnSignedProof},
        stamp::Stamp,
        state::State,
    },
    util::key::PK,
};

impl State {
    pub fn update_pool_and_count(&self, (pool, count): (ProofPool, u32)) -> Self {
        State {
            proof_pool: pool,
            stamp_pool: self.stamp_pool.clone(),
            un_signed_proof_pool: self.un_signed_proof_pool.clone(),
            count: count,
            node_sk: self.node_sk.clone(),
            address: self.address.clone(),
            peers: self.peers.clone(),
        }
    }
    pub fn add_to_stamp_pool(&self, pk: PK, stamp: Stamp) -> Self {
        if self.un_signed_proof_pool.get(&pk).is_none() {
            return self.clone();
        }
        let mut new_stamp_pool = self.stamp_pool.clone();
        new_stamp_pool.insert(pk, stamp);
        State {
            proof_pool: self.proof_pool.clone(),
            stamp_pool: new_stamp_pool,
            un_signed_proof_pool: self.un_signed_proof_pool.clone(),
            count: self.count,
            node_sk: self.node_sk.clone(),
            address: self.address.clone(),
            peers: self.peers.clone(),
        }
    }
    pub fn add_to_un_signed_proof_pool(&self, un_signed_proof: UnSignedProof) -> Self {
        let mut new_un_signed_proof_pool = self.un_signed_proof_pool.clone();
        let Ok(pk) = un_signed_proof.get_proof_pk() else {
            return self.clone();
        };
        new_un_signed_proof_pool.insert(pk, un_signed_proof);
        State {
            proof_pool: self.proof_pool.clone(),
            stamp_pool: self.stamp_pool.clone(),
            un_signed_proof_pool: new_un_signed_proof_pool,
            count: self.count,
            node_sk: self.node_sk.clone(),
            address: self.address.clone(),
            peers: self.peers.clone(),
        }
    }
}
