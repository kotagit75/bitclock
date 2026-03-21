#[macro_use]
extern crate log;
extern crate simple_logger as logger;

use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    watch,
};

use crate::{
    adapter::init_adapter,
    core::node::load_key,
    effect::run::run_effect,
    model::{event::Event, state::State},
    update::update,
};

mod adapter;
mod core;
mod effect;
mod model;
mod update;
mod util;

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();

    let Ok(key_pair) = load_key().await else {
        error!("Failed to load the private key");
        return;
    };
    let Ok(mut state) = State::new(key_pair) else {
        return;
    };
    let (tx, mut rx): (Sender<Event>, Receiver<Event>) = mpsc::channel(100);
    let (state_tx, state_rx) = watch::channel(state.clone());
    init_adapter(tx, state_rx);
    loop {
        debug!("New state: {:?}", state);
        let Some(event) = rx.recv().await else {
            continue;
        };
        debug!("Got an event: {:?}", event);
        let (new_state, effect) = update(
            state.clone(),
            event,
            chrono::prelude::Utc::now().timestamp_millis(),
        );
        state = new_state;
        let _ = state_tx.send(state.clone());
        let state_clone = state.clone();
        tokio::spawn(async move { run_effect(state_clone, effect).await });
    }
}
