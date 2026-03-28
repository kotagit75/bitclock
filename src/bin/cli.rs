use bitclock::{
    model::{
        address::Address,
        api::{APICommand, APIResponse, ApiOrdering, SKPair},
        data::Data,
        proof::Proof,
        state::State,
    },
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
    AddPeer {
        ip: String,
    },
    Proof {
        recipient_der: String,
        credential: String,
    },
    State,
    Address,
    Pool,
    Peers,
    Find {
        der: String,
    },
    Verify {
        proof_str: String,
    },
    Compare {
        der1: String,
        der2: String,
    },
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

async fn compare_proof(keys: SKPair) -> Result<ApiOrdering, reqwest::Error> {
    let client = Client::new();
    let response = client
        .get("http://localhost:8080/query/compare")
        .json(&keys)
        .send()
        .await?;
    response.json::<ApiOrdering>().await
}

async fn post_api_command(command: APICommand) -> Result<APIResponse, reqwest::Error> {
    let client = Client::new();
    let response = client
        .post("http://localhost:8080/")
        .json(&command)
        .send()
        .await?;
    response.json::<APIResponse>().await
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let Ok(state) = get_state().await else {
        return;
    };

    match args.subcommand {
        SubCommands::AddPeer { ip } => {
            let _ = post_api_command(APICommand::AddPeer(ip)).await;
        }
        SubCommands::Proof {
            recipient_der,
            credential,
        } => {
            let recipient = Address { der: recipient_der };
            let data = Data {
                recipient,
                issuer: state.address.clone(),
                credential,
            };
            let Ok(res) = post_api_command(APICommand::Proof(data)).await else {
                return;
            };
            if let APIResponse::Proof(Ok(sk)) = res {
                print!("{}", sk.der);
            }
        }
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
        SubCommands::Compare { der1, der2 } => {
            let Ok(result) = compare_proof(SKPair {
                sk1: SK { der: der1 },
                sk2: SK { der: der2 },
            })
            .await
            else {
                return;
            };
            print!("{:?}", result);
        }
    }
}
