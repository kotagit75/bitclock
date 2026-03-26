use tokio::sync::{mpsc, watch};

use crate::{
    adapter::init_adapter,
    core::node::load_key,
    model::{event::Event, state::State},
    util::status::get_status,
};

pub async fn boot() -> Result<(State, (mpsc::Receiver<Event>, watch::Sender<State>)), ()> {
    info!("BitClock is booting up");
    info!("{:?}", get_status());
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
