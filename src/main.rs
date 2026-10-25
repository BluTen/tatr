use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

mod sub_cmd;
mod task;
mod utils;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Args)]
struct InitArgs {
    /// the directory to initialize at
    #[arg(default_value = ".", value_parser = parse_existing_dir)]
    dir: PathBuf,
}

#[derive(Args)]
struct NewArgs {
    /// Tags to add to task
    #[arg(short, long, value_delimiter = ',')]
    tags: Option<Vec<task::Tag>>,

    /// Priority of the task
    #[arg(short, long, default_value = "50")]
    priority: u8,

    /// Initial status of the task
    #[arg(short, long, default_value = "open")]
    status: task::Status,

    /// Title of the task
    #[arg(num_args=1..)]
    title: Vec<String>,
}

#[derive(Subcommand)]
enum Commands {
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

fn parse_existing_dir(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    if !path.exists() {
        Err(format!("The path '{}' does not exist.", s))
    } else if !path.is_dir() {
        Err(format!(
            "The path '{}' is a file, but a directory was expected.",
            s
        ))
    } else {
        Ok(path)
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(args) => sub_cmd::init(args),

        Commands::New(args) => sub_cmd::new(args),

        Commands::Meow => {
            println!("mrawww!");
        }
    }
}
