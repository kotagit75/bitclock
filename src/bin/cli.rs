use bitclock::{
    model::{api::APICommand, proof::Proof, state::State},
    util::key::SK,
};
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
    Find { der: String },
    Verify { proof_str: String },
}

async fn get_state() -> Result<State, reqwest::Error> {
    let client = Client::new();
    let response = client.get("http://localhost:8080/query").send().await?;
    response.json::<State>().await
}

async fn verify_proof(proof: Proof) -> Result<bool, reqwest::Error> {
    let client = Client::new();
    let response = client
        .get("http://localhost:8080/query/verify")
        .json(&proof)
        .send()
        .await?;
    response.json::<bool>().await
}

async fn post_api_command(command: APICommand) {
    let client = Client::new();
    let _ = client
        .post("http://localhost:8080/")
        .json(&command)
        .send()
        .await;
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
            print!("{}", json_str);
        }
        SubCommands::Address => {
            let Ok(json_str) = serde_json::to_string(&state.address) else {
                return;
            };
            print!("{}", json_str);
        }
        SubCommands::Pool => {
            let Ok(json_str) = serde_json::to_string(&state.proof_pool) else {
                return;
            };
            print!("{}", json_str);
        }
        SubCommands::Peers => {
            let Ok(json_str) = serde_json::to_string(&state.peers) else {
                return;
            };
            print!("{}", json_str);
        }
        SubCommands::Proof { data } => {
            let _ = post_api_command(APICommand::Proof(data)).await;
        }
        SubCommands::Find { der } => {
            if let Some(proof) = state.proof_pool.find_by_sk(&SK { der }) {
                let Ok(json_str) = serde_json::to_string(&proof) else {
                    return;
                };
                print!("{}", json_str);
            }
        }
        SubCommands::Verify { proof_str } => {
            if let Ok(proof) = serde_json::from_str::<Proof>(&proof_str) {
                let Ok(result) = verify_proof(proof).await else {
                    return;
                };
                print!("{}", result);
            } else {
                print!("Invalid proof string");
            }
        }
    }
}
