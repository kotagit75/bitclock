use crate::adapter::api::APIRequest;
use crate::adapter::p2p::P2PMessage;
use crate::model::proof::UnSignedProof;
use crate::model::{effect::Effect, event::Event, state::State};

pub fn update(state: State, event: Event, time: i64) -> (State, Effect) {
    match event {
        Event::P2PRequest(P2PMessage::None) => (state, Effect::None),
        Event::P2PRequest(P2PMessage::RequestStamp(pk, difficulty)) => {
            (state, Effect::CreateStamp(pk, difficulty))
        }
        Event::P2PRequest(P2PMessage::ResponceStamp(pk, stamp)) => {
            let state = state.add_to_stamp_pool(stamp);
            let (Some(un_stamped_proof), Some(stamps)) = (
                state.find_from_un_stamped_proof_pool(&pk),
                state.find_from_stamp_pool(&pk),
            ) else {
                return (state, Effect::None);
            };
            let Ok(proof) =
                un_stamped_proof.create_signed_proof(state.node_sk.clone(), stamps.to_vec())
            else {
                return (state, Effect::None);
            };
            (
                state.update_pool_and_count(state.proof_pool.add_proof(
                    state.address.clone(),
                    state.count,
                    proof,
                )),
                Effect::None,
            )
        }
        Event::P2PRequest(P2PMessage::UpdateProofpool(new_pool)) => {
            let r = state
                .proof_pool
                .update(state.address.clone(), state.count, &new_pool);
            if r.0 {
                return (
                    state.update_pool_and_count(r),
                    Effect::Broadcast(P2PMessage::UpdateProofpool(state.proof_pool)),
                );
            }
            (state.update_pool_and_count(r), Effect::None)
        }
        Event::APIRequest(APIRequest::None) => (state, Effect::None),
        Event::APIRequest(APIRequest::Proof(data)) => {
            let Ok((un_signed_proof, pk)) =
                UnSignedProof::create(data, state.address.clone(), time, &state.proof_pool.clone())
            else {
                return (state, Effect::None);
            };
            (
                state.add_to_un_signed_proof_pool(un_signed_proof.clone()),
                Effect::Broadcast(P2PMessage::RequestStamp(pk, un_signed_proof.difficulty)),
            )
        }
        Event::APIRequest(APIRequest::AddPeer(ip)) => (state.add_peer(ip), Effect::None),
    }
}
