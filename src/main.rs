use clap::{command, Parser, Subcommand};
use std::process::Command;

#[derive(Subcommand)]
enum Commands {
    /// Run test from specified problem id
    Test {
        /// The problem id
        id: String,
    },
}

#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// The problem id
    #[arg(global = true, required = false)]
    id: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() {
    let args = Args::parse();
    let mut cargo = Command::new("cargo");

    let status = if let Some(Commands::Test { id }) = args.command {
        let id = format!("prob{}", id);

        cargo.args(["test", "--bin", &id])
    } else {
        let id = format!("prob{}", args.id);

        cargo.args(["run", "--release", "--bin", &id])
    }
    .status()
    .expect("cargo command failed to start");

    assert!(status.success());
}
