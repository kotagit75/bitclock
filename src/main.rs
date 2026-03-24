#[macro_use]
extern crate log;
extern crate simple_logger as logger;

use clap::Parser;

use crate::{
    boot::{Args, boot},
    effect::loop_effect_run,
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
        tokio::spawn(async move { loop_effect_run(state_clone, effect).await });
    }
}
