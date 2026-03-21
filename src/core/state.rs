use crate::model::{
    client::Client,
    proof::{ProofPool, UnSignedProof},
    stamp::Stamp,
    state::State,
};

impl State {
    pub fn update_proof_pool(&self, pool: ProofPool) -> Self {
        State {
            proof_pool: pool,
            stamp_pool: self.stamp_pool.clone(),
            un_signed_proof_pool: self.un_signed_proof_pool.clone(),
            count: self.count,
            node_sk: self.node_sk.clone(),
            address: self.address.clone(),
            peers: self.peers.clone(),
        }
    }
    pub fn update_count(&self, count: u32) -> Self {
        State {
            proof_pool: self.proof_pool.clone(),
            stamp_pool: self.stamp_pool.clone(),
            un_signed_proof_pool: self.un_signed_proof_pool.clone(),
            count: count,
            node_sk: self.node_sk.clone(),
            address: self.address.clone(),
            peers: self.peers.clone(),
        }
    }
    pub fn update_stamp_pool(&self, pool: Vec<Stamp>) -> Self {
        State {
            proof_pool: self.proof_pool.clone(),
            stamp_pool: pool,
            un_signed_proof_pool: self.un_signed_proof_pool.clone(),
            count: self.count,
            node_sk: self.node_sk.clone(),
            address: self.address.clone(),
            peers: self.peers.clone(),
        }
    }
    pub fn update_un_signed_proof_pool(&self, pool: Vec<UnSignedProof>) -> Self {
        State {
            proof_pool: self.proof_pool.clone(),
            stamp_pool: self.stamp_pool.clone(),
            un_signed_proof_pool: pool,
            count: self.count,
            node_sk: self.node_sk.clone(),
            address: self.address.clone(),
            peers: self.peers.clone(),
        }
    }
    pub fn update_peers(&self, peers: Vec<Client>) -> Self {
        State {
            proof_pool: self.proof_pool.clone(),
            stamp_pool: self.stamp_pool.clone(),
            un_signed_proof_pool: self.un_signed_proof_pool.clone(),
            count: self.count,
            node_sk: self.node_sk.clone(),
            address: self.address.clone(),
            peers,
        }
    }
    pub fn update_pool_and_count(&self, (pool, count): (ProofPool, u32)) -> Self {
        self.update_proof_pool(pool).update_count(count)
    }
}
impl State {
    pub fn add_to_stamp_pool(&self, stamp: Stamp) -> Self {
        let mut new_stamp_pool = self.stamp_pool.clone();
        new_stamp_pool.push(stamp);
        self.update_stamp_pool(new_stamp_pool)
    }
    pub fn add_to_un_signed_proof_pool(&self, un_signed_proof: UnSignedProof) -> Self {
        let mut new_un_signed_proof_pool = self.un_signed_proof_pool.clone();
        new_un_signed_proof_pool.push(un_signed_proof);
        self.update_un_signed_proof_pool(new_un_signed_proof_pool)
    }
    pub fn add_peer(&self, ip: String) -> Self {
        let mut new_peers = self.peers.clone();
        new_peers.push(Client::new(ip));
        self.update_peers(new_peers)
    }
}
