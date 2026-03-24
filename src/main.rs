#[macro_use]
extern crate log;
extern crate simple_logger as logger;

use crate::bitclock::start;

mod adapter;
mod bitclock;
mod boot;
mod core;
mod effect;
mod model;
mod update;
mod util;

#[tokio::main]
async fn main() {
    start().await;
}
