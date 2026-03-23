use clap::Parser;

#[macro_use]
extern crate log;
extern crate simple_logger as logger;

use tokio::sync::{mpsc, watch};

use crate::{
    adapter::init_adapter,
    core::node::load_key,
    effect::run::run_effect,
    model::{event::Event, state::State},
    update::update,
    util::status::get_status,
};

mod adapter;
mod core;
mod effect;
mod model;
mod update;
mod util;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = log::Level::Info)]
    level: log::Level,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    simple_logger::init_with_level(args.level).unwrap();

    info!("BitClock is booting up");
    info!("{:?}", get_status());
    let Ok((mut state, (mut event_rx, state_tx))) = init().await else {
        return;
    };

    debug!("New state: {:?}", state);
    while let Some((new_state, effect)) = event_rx.recv().await.and_then(|event| {
        debug!("Got an event: {:?}", event);
        Some(update(
            state.clone(),
            event,
            chrono::prelude::Utc::now().timestamp_millis(),
        ))
    }) {
        state = new_state.clone();
        let _ = state_tx.send(state.clone());
        let state_clone = state.clone();
        tokio::spawn(async move {
            let mut effect_opt = Some(effect);
            while let Some(effect) = effect_opt {
                effect_opt = run_effect(state_clone.clone(), effect).await;
            }
        });
        debug!("New state: {:?}", state);
    }
}

async fn init() -> Result<(State, (mpsc::Receiver<Event>, watch::Sender<State>)), ()> {
    let Ok(key_pair) = load_key().await else {
        error!("Failed to load the private key");
        return Err(());
    };
    let Ok(state) = State::new(key_pair) else {
        error!("Failed to create state");
        return Err(());
    };
    Ok((state.clone(), init_adapter(state)))
}
