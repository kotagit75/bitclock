use std::process::exit;

use crate::{boot::boot, effect::loop_effect_run, update::update};

pub async fn start(level: log::Level) {
    simple_logger::init_with_level(level).unwrap();
    let Ok((mut state, (mut event_rx, state_tx))) = boot().await else {
        exit(1);
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
