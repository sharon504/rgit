use clap::{Parser, Subcommand};
use rgit_lib::commands::Repository;
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum Commands {
    Init,
    Log,
    Add {
        files: Vec<String>,
    },
    Commit {
        #[arg(short, long)]
        message: String,
    },
    Branch {
        name: String,
    },
    Checkout {
        name: String,
    },
    Tag {
        name: String,
    },
    Config {
        key: String,
        value: Option<String>,
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
    let mut repo = Repository::new(PathBuf::from("."));
    match &cli.command {
        Commands::Init => {
            if let Err(err) = repo.init() {
                eprintln!("{}", err)
            }
        }
        _ => match repo.find() {
            Ok(_) => match cli.command {
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
                Commands::Log => {
                    if let Err(err) = repo.log() {
                        eprintln!("{}", err)
                    }
                }
                Commands::Branch { name } => {
                    if let Err(e) = repo.branch(name) {
                        eprintln!("{}", e);
                    }
                }
                Commands::Checkout { name } => {
                    if let Err(e) = repo.checkout(name) {
                        eprint!("{e}");
                    }
                }
                Commands::Tag { name } => {
                    if let Err(e) = repo.tag(name) {
                        eprintln!("{e}");
                    }
                }
                Commands::Config { key, value } => {
                    if let Err(e) = repo.config(key, value) {
                        eprintln!("{e}")
                    }
                }
                _ => unreachable!(),
            },
            Err(e) => eprintln!("{}", e),
        },
    }
}
