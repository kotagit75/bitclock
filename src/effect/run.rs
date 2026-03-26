use crate::{
    core::stamp::{calc_nonce, create_sign_to_stamp},
    model::{client::Client, effect::Effect, p2p::P2PMessage, stamp::Stamp, state::State},
};

pub async fn run_effect(state: State, effect: Effect) -> Option<Effect> {
    match effect {
        Effect::CreateStamp(pk, difficulty) => {
            let address = state.address;
            let count = state.count + 1;
            let id = 0;
            let nonce = calc_nonce(difficulty, &address, count, &pk, id);
            if let Ok(sign) =
                create_sign_to_stamp(state.node_sk, &address, count, pk.clone(), nonce, id)
            {
                let stamp = Stamp {
                    address,
                    count,
                    pk: pk.clone(),
                    nonce,
                    id,
                    sign,
                };
                return Some(Effect::Broadcast(P2PMessage::ResponceStamp(pk, stamp)));
            }
        }
        Effect::Broadcast(message) => {
            broadcast(state.peers, message).await;
        }
        Effect::APIResponce(tx, res) => {
            let _ = tx.send(res).await;
        }
    }
    None
}

async fn broadcast(peers: Vec<Client>, message: P2PMessage) {
    for peer in peers {
        peer.write(&message).await;
    }
}
