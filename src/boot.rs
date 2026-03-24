use clap::Parser;
use tokio::sync::{mpsc, watch};

use crate::{
    adapter::init_adapter,
    core::node::load_key,
    model::{event::Event, state::State},
    util::status::get_status,
};

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long, default_value_t = log::Level::Info)]
    pub level: log::Level,

    #[arg(short, long, default_value_t = 8080)]
    pub api_port: u32,
}

pub async fn boot(
    args: Args,
) -> Result<(State, (mpsc::Receiver<Event>, watch::Sender<State>)), ()> {
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
    Ok((state.clone(), init_adapter(state, args.api_port)))
}
