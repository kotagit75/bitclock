use crate::{
    adapter::p2p::P2PMessage,
    model::{client::Client, effect::Effect, state::State},
};

pub async fn run_effect(state: State, effect: Effect) {
    match effect {
        Effect::None => {}
        Effect::CreateStamp(pk, difficulty) => {
            // create_stamp and broadcast
        }
        Effect::Broadcast(message) => {
            broadcast(state.peers, message).await;
        }
    }
}

async fn broadcast(peers: Vec<Client>, message: P2PMessage) {
    for peer in peers {
        peer.write(&message).await;
    }
}
