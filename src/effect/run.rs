use crate::{
    adapter::p2p::P2PMessage,
    core::stamp::{calc_nonce, create_sign_to_stamp},
    model::{client::Client, effect::Effect, stamp::Stamp, state::State},
};

pub async fn run_effect(state: State, effect: Effect) {
    match effect {
        Effect::None => {}
        Effect::CreateStamp(pk, difficulty) => {
            let address = state.address;
            let count = state.count + 1;
            let id = 0;
            let nonce = calc_nonce(difficulty, &address, count, &pk, id);
            let Ok(sign) =
                create_sign_to_stamp(state.node_sk, &address, count, pk.clone(), nonce, id)
            else {
                return;
            };
            broadcast(
                state.peers,
                P2PMessage::ResponceStamp(
                    pk.clone(),
                    Stamp {
                        address,
                        count,
                        pk,
                        nonce,
                        id,
                        sign,
                    },
                ),
            )
            .await;
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
