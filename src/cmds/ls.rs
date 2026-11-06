use crate::{Result, task, utils};
use clap::Args;

#[derive(Args)]
pub(crate) struct LsArgs {
    /// Tags to add to task
    #[arg(short, long, default_value = "false")]
    all: bool,

    /// `Tatr Query Language` prompt to filter the list
    #[arg(default_value = "")]
    tql: String,
}

pub(crate) fn ls(args: LsArgs) -> Result<()> {
    let path = utils::find_tasks_dir()?;
    let mut tasks = task::read_all_tasks(&path)?;
    // task::sort_by_priority(&mut tasks, false);
    use std::cmp::Reverse;
    tasks.sort_by_key(|(_, a)| Reverse(a.priority));

    for (huid, task) in tasks {
        if !args.all && task.status == task::Status::Closed {
            continue;
        }

        println!(
            "{}",
            task::summary_line(&task, path.to_str().unwrap_or("INVALID_PATH/"), &huid)
        )
    }

    Ok(())
}
