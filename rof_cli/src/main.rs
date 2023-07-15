use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "RoF")]
#[command(author = "TC <tscasady@gmail.com>")]
#[command(version = "0.1")]
#[command(about = "Creates a new RoF project", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8
}

#[derive(Subcommand)]
enum Commands {
    /// Creates a new project 
    New(Name),
    #[clap(about = "Starts the Server")]
    Server,
}

#[derive(Args)]
pub struct Name {
    name: String
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Server => {
            println!("here");
            let mut cmd = std::process::Command::new("./target/debug/rof_server").spawn().expect("SErver?");
        }
        Commands::New(name) => {
            println!("{}", name.name)
        }
    } 
}
