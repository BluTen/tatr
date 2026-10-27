use clap::{Parser, Subcommand};

use crate::cmds::*;

mod cmds;
mod task;
mod utils;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Initialize a new tasks directory
    Init(InitArgs),

    /// Add a new task to the repository
    ///
    /// Will find a task folder from cwd and go up each time it doesn't find one.
    New(NewArgs),

    // Eh dont worry about it
    #[command(hide = true)]
    Meow,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(args) => cmds::init(args),

        Commands::New(args) => cmds::new(args),

        Commands::Meow => {
            println!("mrawww!");
        }
    }
}
