use std::process::Command;

use clap::{Parser, Subcommand};
use run_script::ScriptOptions;

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
    let mut options = ScriptOptions::new();
    options.runner = Some("bash".to_string());
    options.capture_output = true; // True to capture and return the output. False will print it to the parent process output.
    options.exit_on_error = true; // Adds set -e option (not available for windows)
    options.print_commands = false; // Adds set -x option (not available for windows)

    let (code, stdout, stderr) = run_script::run_script!(
        r#"
        yarn run test & solana-test-validator
        "#,
        &vec![],
        &options
    )
    .unwrap();
    println!("code: {}, stdout: {}, stderr: {}", code, stdout, stderr);
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        LiteCommand::Test => run_test(),
    }
}
