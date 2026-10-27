use crate::{task, utils};
use clap::Args;

#[derive(Args)]
pub(crate) struct NewArgs {
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

pub(crate) fn new(args: NewArgs) {
    let tasks_dir_path = utils::find_tasks_dir().unwrap_or_else(|err| {
        eprintln!("Error: {err}");
        std::process::exit(2);
    });

    let task_id = utils::generate_huid();

    let new_task_dir = tasks_dir_path.join(format!("{}/", task_id));
    std::fs::create_dir_all(&new_task_dir).unwrap_or_else(|err| {
        eprintln!(
            "Error: Failed to create current task directory at {:?}\nReason: {}",
            tasks_dir_path.display(),
            err
        );
        std::process::exit(1);
    });

    let NewArgs {
        tags,
        priority,
        status,
        title,
    } = args;

    let parsed_title = title.join(" ").parse::<task::Title>().unwrap_or_else(|e| {
        eprintln!("Error: Invalid title - {}", e);
        std::process::exit(1);
    });
    let task = task::Task::new(parsed_title, status, tags.unwrap_or(vec![]), priority);

    task.save_to_file(&new_task_dir.join("TASK.md"));

    println!(
        "Generated task '{task_id}' at '{}'",
        tasks_dir_path.display()
    )
    // Implement the logic for adding a new task here
}
