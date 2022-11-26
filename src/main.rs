use std::process::Command;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    pub command: LiteCommand,
}

#[derive(Subcommand)]
enum LiteCommand {
    /// Test ts script
    Test,
}

fn run_test() {
    let mut binding = Command::new("solana-test-validator");
    //let res = binding.args(["./lite-rpc", "yarn", "run", "test"]);
    println!("{:?}", binding.output());
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        LiteCommand::Test => run_test(),
    }
}
