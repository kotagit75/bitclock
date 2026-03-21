#[macro_use]
extern crate log;
extern crate simple_logger as logger;

use std::collections::HashMap;

use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::{
    adapter::init_adapter,
    effect::run::run_effect,
    model::{client::Client, event::Event, proof::ProofPool, state::State},
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
        stamp_pool: Vec::new(),
        un_signed_proof_pool: Vec::new(),
        count: 0,
        node_sk,
        address,
        peers: vec![Client::new("localhost".to_string())],
    };

    let (tx, mut rx): (Sender<Event>, Receiver<Event>) = mpsc::channel(100);
    init_adapter(tx);

    loop {
        let Some(event) = rx.recv().await else {
            continue;
        };
        debug!("Got an event: {:?}", event);
        let time = chrono::prelude::Utc::now().timestamp_millis();
        let (new_state, effect) = update(state.clone(), event, time);
        state = new_state;
        let state_clone = state.clone();
        tokio::spawn(async move { run_effect(state_clone, effect).await });
    }
}
