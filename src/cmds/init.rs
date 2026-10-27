use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub(crate) struct InitArgs {
    /// the directory to initialize at
    #[arg(default_value = ".", value_parser = parse_existing_dir)]
    dir: PathBuf,
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

pub(crate) fn init(args: InitArgs) {
    let tasks_dir = args.dir.join("tasks");

    if tasks_dir.exists() {
        println!(
            "Warning: The tasks directory at {} already exists. No changes were made.",
            tasks_dir.display()
        );
        return;
    }

    std::fs::create_dir_all(&tasks_dir).unwrap_or_else(|err| {
        eprintln!(
            "Error: Failed to create tasks directory at {}\nReason: {}",
            tasks_dir.display(),
            err
        );
        std::process::exit(1);
    });

    let absolute_tasks_dir = tasks_dir
        .canonicalize()
        .unwrap_or_else(|_| tasks_dir.clone());

    if let Some(parent_path) = absolute_tasks_dir.parent() {
        if let Some(parent_name) = parent_path.file_name() {
            println!("Initialized tasks at {}", parent_name.to_string_lossy());
        } else {
            println!("Initialized tasks at root {}", parent_path.display());
        }
    }
}
