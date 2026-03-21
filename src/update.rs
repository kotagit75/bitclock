use crate::adapter::api::APIRequest;
use crate::adapter::p2p::P2PMessage;
use crate::core::proof::PROOF_KEY_BITS;
use crate::model::proof::UnSignedProof;
use crate::model::{effect::Effect, event::Event, state::State};
use crate::util::key::generate_pk_and_sk;

pub fn update(state: State, event: Event, time: i64) -> (State, Effect) {
    match event {
        Event::P2PRequest(P2PMessage::None) => (state, Effect::None),
        Event::P2PRequest(P2PMessage::RequestStamp(pk, difficulty)) => {
            (state, Effect::CreateStamp(pk, difficulty))
        }
        Event::P2PRequest(P2PMessage::ResponceStamp(pk, stamp)) => {
            (state.add_to_stamp_pool(stamp), Effect::None)
        }
        Event::P2PRequest(P2PMessage::UpdateProofpool(new_pool)) => (
            state.update_pool_and_count(state.proof_pool.update(
                state.address.clone(),
                state.count,
                &new_pool,
            )),
            Effect::None,
        ),
        Event::APIRequest(APIRequest::None) => (state, Effect::None),
        Event::APIRequest(APIRequest::Proof(data)) => {
            let Ok((pk, sk)) = generate_pk_and_sk(PROOF_KEY_BITS) else {
                return (state, Effect::None);
            };
            let difficulty = state.proof_pool.calc_difficulty(time);
            let un_signed_proof =
                UnSignedProof::new(data, sk, state.address.clone(), difficulty, time);
            (
                state.add_to_un_signed_proof_pool(un_signed_proof),
                Effect::Broadcast(P2PMessage::RequestStamp(pk, difficulty)),
            )
        }
        Event::APIRequest(APIRequest::AddPeer(ip)) => (state.add_peer(ip), Effect::None),
    }
}
