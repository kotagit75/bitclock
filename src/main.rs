#[macro_use]
extern crate log;
extern crate simple_logger as logger;

use clap::Parser;

use crate::bitclock::start;

mod adapter;
mod bitclock;
mod boot;
mod core;
mod effect;
mod model;
mod update;
mod util;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = log::Level::Info)]
    pub level: log::Level,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    start(args.level).await;
}
