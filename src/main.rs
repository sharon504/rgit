mod commands;
mod errors;

use clap::{Parser, Subcommand};
use commands::Repository;
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum Commands {
    Init,
    Add {
        files: Vec<String>,
    },
    Commit {
        #[arg(short, long)]
        message: String,
    },
}

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(short, long, action=clap::ArgAction::SetTrue)]
    debug: bool,

    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();
    if cli.debug {
        println!("Debug mode is on");
    }
    match &cli.command {
        Commands::Init => {
            let repo = Repository::new(PathBuf::from("."));
            if let Err(err) = repo.init() {
                eprintln!("{}", err)
            }
        }
        _ => match Repository::find() {
            Ok(repo) => match cli.command {
                Commands::Add { files } => {
                    if let Err(err) = repo.add(files) {
                        eprintln!("{}", err)
                    }
                }
                Commands::Commit { message } => {
                    if let Err(err) = repo.commit(message) {
                        eprintln!("{}", err)
                    }
                }
                _ => unreachable!(),
            },
            Err(e) => eprintln!("{}", e),
        },
    }
}
