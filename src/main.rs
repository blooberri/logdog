use std::fs::File;
use std::io::Write;
use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(name = "logdog")]
#[command(version = "1.0")]
#[command(about = "Automates bug bounty reporting from the terminal", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    //intializes a new report.md file
    Init,
}
fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init => {
            let mut file = File::create("report.md").expect("ERROR! Failed to create file");
            file.write_all(b"# Vulnerability Report\n\n## Steps to Reproduce:\n").expect("Failed to write");
        }
    }
}