use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::{
    core::proof::PROOF_KEY_BITS,
    effect::run::run_effect,
    model::{event::Event, proof::ProofPool, state::State},
    update::update,
    util::key::generate_pk_and_sk,
};

mod adapter;
mod core;
mod effect;
mod model;
mod update;
mod util;

const NODE_KEY_BITS: u32 = 512;

#[tokio::main]
async fn main() {
    let Ok((address, node_sk)) = generate_pk_and_sk(NODE_KEY_BITS) else {
        return;
    };
    let mut state = State {
        proof_pool: ProofPool::new(),
        count: 0,
        node_sk,
        address,
    };

    let (tx, mut rx): (Sender<Event>, Receiver<Event>) = mpsc::channel(100);

    loop {
        let Some(event) = rx.recv().await else {
            continue;
        };
        let (new_state, effect) = update(state.clone(), event);
        state = new_state;
        tokio::spawn(async move { run_effect(effect).await });
    }
}
