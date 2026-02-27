use clap::{Parser, Subcommand};
mod commands;
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
    Step {
        description:String,
    },
    Fetch,
    View,
}
fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init => {
            commands::init::execute();
        }
        Commands::Step{description}=>{
            commands::step::execute_step(description.to_string());
        }
        Commands::Fetch => {
            commands::fetch::execute_fetch();
        }
        Commands::View => {
            commands::view::execute_view();
        }
    }
}