use crate::Result;
use std::env;
use std::path::PathBuf;

pub(crate) fn generate_huid() -> Result<String> {
    let now = time::UtcDateTime::now();

    now.format(time::macros::format_description!(
        "[year][month][day]-[hour][minute][second]"
    ))
    .map_err(|err| format!("Unable to generate HUID from datetime, {}", err))
}

pub(crate) fn find_tasks_dir() -> Result<PathBuf> {
    let current_dir =
        env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))?;

    for ancestor in current_dir.ancestors() {
        let potential_tasks_dir = ancestor.join("tasks");

        if potential_tasks_dir.is_dir() {
            return Ok(potential_tasks_dir);
        }
    }

    Err("Could not find a 'tasks' directory in this folder or any parent folders. Did you run 'tatr init'? ".to_string())
}
