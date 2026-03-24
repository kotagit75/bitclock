#[macro_use]
extern crate log;
extern crate simple_logger as logger;

use clap::Parser;

use crate::{
    boot::{Args, boot},
    effect::run::run_effect,
    update::update,
};

mod adapter;
mod boot;
mod core;
mod effect;
mod model;
mod update;
mod util;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    simple_logger::init_with_level(args.level).unwrap();

    let Ok((mut state, (mut event_rx, state_tx))) = boot(args).await else {
        return;
    };

    debug!("New state: {:?}", state);
    while let Some((new_state, effect)) = event_rx.recv().await.and_then(|event| {
        debug!("Event received: {:?}", event);
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
                debug!("Running effect: {:?}", effect);
                effect_opt = run_effect(state_clone.clone(), effect).await;
            }
        });
        debug!("New state: {:?}", state);
    }
}
