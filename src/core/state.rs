use std::collections::HashMap;

use crate::{
    model::{
        address::Address,
        p2p::{Client, MY_IP_ADDR},
        proof::{Proof, UnSignedProof},
        proofpool::ProofPool,
        stamp::Stamp,
        state::State,
    },
    util::key::{PK, SK},
};

impl State {
    pub fn new((address, node_sk): (Address, SK)) -> Self {
        State {
            proof_pool: ProofPool::default(),
            stamp_pool: Vec::new(),
            un_signed_proof_pool: Vec::new(),
            count: 0,
            node_sk,
            address,
            peers: vec![Client::new(MY_IP_ADDR.to_string())],
        }
    }
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
            count,
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
    pub fn update_pool_and_count(&self, (_, pool, count): (bool, ProofPool, u32)) -> Self {
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
    fn stamp_pool_to_map(&self) -> HashMap<PK, Vec<Stamp>> {
        let mut stamp_map: HashMap<PK, Vec<Stamp>> = HashMap::new();
        for stamp in self.stamp_pool.clone() {
            stamp_map.entry(stamp.pk.clone()).or_default().push(stamp);
        }
        stamp_map
    }
    fn un_signed_proof_pool_to_map(&self) -> HashMap<PK, Proof> {
        let mut proof_map: HashMap<PK, Proof> = HashMap::new();
        for proof in self.un_signed_proof_pool.clone() {
            if let Ok(pk_) = proof.get_proof_pk() {
                proof_map.insert(pk_, proof);
            }
        }
        proof_map
    }
    pub fn find_from_stamp_pool(&self, pk: &PK) -> Option<Vec<Stamp>> {
        self.stamp_pool_to_map().get(pk).cloned()
    }
    pub fn find_from_un_stamped_proof_pool(&self, pk: &PK) -> Option<Proof> {
        self.un_signed_proof_pool_to_map().get(pk).cloned()
    }
}

#[cfg(test)]
mod tests {
    use crate::{model::data::Data, util::key::generate_pk_and_sk};

    use super::*;

    #[test]
    fn test_add_peer() {
        let (address, node_sk) = generate_pk_and_sk(512).unwrap();
        let state = State::new((address.clone(), node_sk.clone()));
        let ip_addr = "ip_addr".to_string();
        let new_state = state.add_peer(ip_addr.clone());
        assert!(new_state.peers.contains(&Client::new(ip_addr)));
    }
    #[test]
    fn test_add_to_un_signed_proof_pool() {
        let (address, node_sk) = generate_pk_and_sk(512).unwrap();
        let data = Data::new(
            node_sk.clone(),
            address.clone(),
            address.clone(),
            "content".to_string(),
        )
        .unwrap();
        let state = State::new((address.clone(), node_sk.clone()));
        let un_signed_proof = UnSignedProof::new(data, node_sk, address.clone(), 0, 0);
        let new_state = state.add_to_un_signed_proof_pool(un_signed_proof.clone());
        assert_eq!(new_state.un_signed_proof_pool.clone().len(), 1);
        assert_eq!(new_state.un_signed_proof_pool[0], un_signed_proof);
    }
}
