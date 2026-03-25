use crate::model::api::APICommand;
use crate::model::p2p::P2PMessage;
use crate::model::proof::UnSignedProof;
use crate::model::{effect::Effect, event::Event, state::State};

pub fn update(state: State, event: Event, time: i64) -> (State, Effect) {
    match event {
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
            match un_stamped_proof.create_signed_proof(state.node_sk.clone(), stamps.to_vec()) {
                Ok(proof) => {
                    let state = state.update_pool_and_count(state.proof_pool.add_proof(
                        state.address.clone(),
                        state.count,
                        proof,
                    ));
                    (
                        state.clone(),
                        Effect::Broadcast(P2PMessage::UpdateProofpool(state.proof_pool)),
                    )
                }
                Err(_) => (state, Effect::None),
            }
        }
        Event::P2PRequest(P2PMessage::UpdateProofpool(new_pool)) => {
            let r = state
                .proof_pool
                .update(state.address.clone(), state.count, &new_pool);
            (
                state.update_pool_and_count(r.clone()),
                match r.0 {
                    true => Effect::Broadcast(P2PMessage::UpdateProofpool(state.proof_pool)),
                    false => Effect::None,
                },
            )
        }
        Event::APIRequest(APICommand::Proof(data)) => {
            match UnSignedProof::create(
                data,
                state.address.clone(),
                time,
                &state.proof_pool.clone(),
            ) {
                Ok((un_signed_proof, pk)) => (
                    state.add_to_un_signed_proof_pool(un_signed_proof.clone()),
                    Effect::Broadcast(P2PMessage::RequestStamp(pk, un_signed_proof.difficulty)),
                ),
                Err(_) => (state, Effect::None),
            }
        }
        Event::APIRequest(APICommand::AddPeer(ip)) => (state.add_peer(ip), Effect::None),
    }
}
