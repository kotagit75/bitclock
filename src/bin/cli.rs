use bitclock::model::{api::APICommand, state::State};
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
    Peers,
    Proof { data: String },
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

async fn post_api_command(command: APICommand) -> Result<State, reqwest::Error> {
    let client = Client::new();
    let response = client
        .post("http://localhost:8080/")
        .json(&command)
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
            let Ok(json_str) = serde_json::to_string(&state) else {
                return;
            };
            println!("{}", json_str);
        }
        SubCommands::Address => {
            let Ok(json_str) = serde_json::to_string(&state.address) else {
                return;
            };
            println!("{}", json_str);
        }
        SubCommands::Pool => {
            let Ok(json_str) = serde_json::to_string(&state.proof_pool) else {
                return;
            };
            println!("{}", json_str);
        }
        SubCommands::Peers => {
            let Ok(json_str) = serde_json::to_string(&state.peers) else {
                return;
            };
            println!("{}", json_str);
        }
        SubCommands::Proof { data } => {
            let _ = post_api_command(APICommand::Proof(data)).await;
        }
    }
}
