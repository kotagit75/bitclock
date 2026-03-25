use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct Args {
    #[clap(subcommand)]
    subcommand: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    Address,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    match args.subcommand {
        SubCommands::Address => {
            println!("address");
        }
    }
}
