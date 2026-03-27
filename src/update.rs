use crate::core::proof::calc_number_of_stamps;
use crate::model::api::{APICommand, APIResponse};
use crate::model::p2p::P2PMessage;
use crate::model::proof::UnSignedProof;
use crate::model::{effect::Effect, event::Event, state::State};

pub fn update(state: State, event: Event, time: i64) -> (State, Vec<Effect>) {
    match event {
        Event::P2PRequest(P2PMessage::RequestStamp(pk, difficulty)) => {
            let effects = (0..calc_number_of_stamps())
                .map(|x| Effect::CreateStamp(pk.clone(), difficulty, x))
                .collect::<Vec<_>>();
            (state, effects)
        }
        Event::P2PRequest(P2PMessage::ResponceStamp(pk, stamp)) => {
            let state = state.add_to_stamp_pool(stamp);
            let (Some(un_stamped_proof), Some(stamps)) = (
                state.find_from_un_stamped_proof_pool(&pk),
                state.find_from_stamp_pool(&pk),
            ) else {
                return (state, Vec::new());
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
                        vec![Effect::Broadcast(P2PMessage::UpdateProofpool(
                            state.proof_pool,
                        ))],
                    )
                }
                Err(_) => (state, Vec::new()),
            }
        }
        Event::P2PRequest(P2PMessage::UpdateProofpool(new_pool)) => {
            let r = state
                .proof_pool
                .update(state.address.clone(), state.count, &new_pool);
            (
                state.update_pool_and_count(r.clone()),
                match r.0 {
                    true => vec![Effect::Broadcast(P2PMessage::UpdateProofpool(
                        state.proof_pool,
                    ))],
                    false => Vec::new(),
                },
            )
        }
        Event::APIRequest(APICommand::Proof(data), tx) => {
            match UnSignedProof::create(
                data,
                state.address.clone(),
                time,
                &state.proof_pool.clone(),
            ) {
                Ok((un_signed_proof, pk)) => (
                    state.add_to_un_signed_proof_pool(un_signed_proof.clone()),
                    vec![
                        Effect::APIResponce(tx, APIResponse::Proof(Ok(un_signed_proof.sk))),
                        Effect::Broadcast(P2PMessage::RequestStamp(pk, un_signed_proof.difficulty)),
                    ],
                ),
                Err(_) => (
                    state,
                    vec![Effect::APIResponce(tx, APIResponse::Proof(Err(())))],
                ),
            }
        }
        Event::APIRequest(APICommand::AddPeer(ip), tx) => (
            state.add_peer(ip),
            vec![Effect::APIResponce(tx, APIResponse::AddPeer)],
        ),
    }
}
