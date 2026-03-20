#[macro_use]
extern crate log;
extern crate simple_logger as logger;

use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::{
    adapter::init_adapter,
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
    simple_logger::init_with_level(log::Level::Debug).unwrap();

    let Ok((address, node_sk)) = generate_pk_and_sk(NODE_KEY_BITS) else {
        error!("Failed to generate the node's private key.");
        return;
    };
    let mut state = State {
        proof_pool: ProofPool::new(),
        count: 0,
        node_sk,
        address,
    };

    let (tx, mut rx): (Sender<Event>, Receiver<Event>) = mpsc::channel(100);
    init_adapter(tx);

    loop {
        let Some(event) = rx.recv().await else {
            continue;
        };
        debug!("Got an event: {:?}", event);
        let (new_state, effect) = update(state.clone(), event);
        state = new_state;
        tokio::spawn(async move { run_effect(effect).await });
    }
}
