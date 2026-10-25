use crate::{InitArgs, NewArgs, task, utils};

pub(crate) fn init(args: InitArgs) {
    let tasks_dir = args.dir.join("tasks");

    if tasks_dir.exists() {
        println!(
            "Warning: The tasks directory at {:?} already exists. No changes were made.",
            tasks_dir.display()
        );
        return;
    }

    std::fs::create_dir_all(&tasks_dir).unwrap_or_else(|err| {
        eprintln!(
            "Error: Failed to create tasks directory at {:?}\nReason: {}",
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
