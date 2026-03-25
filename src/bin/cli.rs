use bitclock::model::state::State;
use clap::{Parser, Subcommand};
use reqwest::Client;

#[derive(Parser, Debug)]
struct Args {
    #[clap(subcommand)]
    subcommand: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    State,
    Address,
    Pool,
}

async fn get_state() -> Result<State, reqwest::Error> {
    let client = Client::new();
    let response = client
        .get("http://localhost:8080/query")
        .send()
        .await
        .unwrap();
    response.json::<State>().await
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let Ok(state) = get_state().await else {
        return;
    };
    match args.subcommand {
        SubCommands::State => {
            println!("{:?}", state)
        }
        SubCommands::Address => {
            println!("{:?}", state.address)
        }
        SubCommands::Pool => {
            println!("{:?}", state.proof_pool)
        }
    }
}
