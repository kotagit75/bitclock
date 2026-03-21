use crate::adapter::p2p::P2PMessage;
use crate::model::{effect::Effect, event::Event, state::State};

pub fn update(state: State, event: Event) -> (State, Effect) {
    match event {
        Event::None => (state, Effect::None),
        Event::P2PRequest(P2PMessage::None) => (state, Effect::None),
        Event::P2PRequest(P2PMessage::RequestStamp(pk, difficulty)) => {
            (state, Effect::CreateStamp(pk, difficulty))
        }
        Event::P2PRequest(P2PMessage::ResponceStamp(pk, stamp)) => (state, Effect::None),
        Event::P2PRequest(P2PMessage::UpdateProofpool(new_pool)) => (
            state.update_pool_and_count(state.proof_pool.update(
                state.address.clone(),
                state.count,
                &new_pool,
            )),
            Effect::None,
        ),
    }
}
