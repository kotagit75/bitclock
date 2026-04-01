use crate::{
    core::stamp::{calc_nonce, calc_solution, create_sign_to_stamp},
    model::{
        effect::Effect,
        p2p::{Client, P2PMessage},
        stamp::Stamp,
        state::State,
    },
};

pub async fn run_effect(state: State, effect: Effect) -> Option<Effect> {
    match effect {
        Effect::CreateStamp(pk, difficulty, id) => {
            let address = state.address;
            let count = state.count + 1;
            let nonce = calc_nonce(difficulty, &address, count, &pk, id);
            let Ok(solution) = calc_solution(&address, count, &pk, nonce, id) else {
                return None;
            };
            if let Ok(sign) =
                create_sign_to_stamp(state.node_sk, &address, count, pk.clone(), nonce, id)
            {
                let stamp = Stamp {
                    address,
                    count,
                    pk: pk.clone(),
                    solution,
                    nonce,
                    id,
                    sign,
                };
                return Some(Effect::Broadcast(P2PMessage::ResponseStamp(pk, stamp)));
            }
        }
        Effect::Broadcast(message) => {
            broadcast(state.peers, message).await;
        }
        Effect::APIResponse(tx, res) => {
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
