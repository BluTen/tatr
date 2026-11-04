use crate::Result;
use std::env;
use std::path::{Path, PathBuf};

pub(crate) fn generate_huid() -> Result<String> {
    let now = time::UtcDateTime::now();

    now.format(time::macros::format_description!(
        "[year][month][day]-[hour][minute][second]"
    ))
    .map_err(|err| format!("Unable to generate HUID from datetime, {}", err))
}

pub(crate) fn is_valid_huid(huid: &str) -> bool {
    let bytes = huid.as_bytes();

    // 1. Regex: [0-9]{8}-[0-9]{6}
    // Minimum length must be exactly 15 characters
    if bytes.len() < 15 {
        return false;
    }

    let base_valid = bytes[8] == b'-'
        && bytes[0..8].iter().all(|b| b.is_ascii_digit())
        && bytes[9..15].iter().all(|b| b.is_ascii_digit());

    if !base_valid {
        return false;
    }

    // Regex: ? (Optional group)
    // If it is exactly 15 characters, it matches without the optional suffix
    if bytes.len() == 15 {
        return true;
    }

    // Regex: (- ... )
    // If it's longer than 15, the optional group MUST begin with a hyphen at index 15
    if bytes[15] != b'-' {
        return false;
    }

    // Regex: [a-zA-Z0-9\-]*
    // The remainder of the string can be alphanumeric or hyphens
    bytes[16..]
        .iter()
        .all(|b| b.is_ascii_alphanumeric() || *b == b'-')
}

pub(crate) fn find_tasks_dir() -> Result<PathBuf> {
    let current_dir =
        env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))?;

    for ancestor in current_dir.ancestors() {
        let potential_tasks_dir = ancestor.join("tasks");

        if is_tasks_dir(&potential_tasks_dir)? {
            return Ok(potential_tasks_dir);
        }
    }

    Err("Could not find a 'tasks' directory in this folder or any parent folders. Did you run 'tatr init'? ".to_string())
}

pub(crate) fn is_tasks_dir(path: &Path) -> Result<bool> {
    let absolute_path = path.canonicalize().map_err(|err| {
        format!(
            "Unable to get absolute path for `{}`. Reason: {}",
            path.display(),
            err
        )
    })?;

    let path_is_dir = absolute_path
        .metadata()
        .map_err(|err| {
            format!(
                "Unable to read metadata of path `{}`. Reason: {}",
                path.display(),
                err
            )
        })?
        .is_dir();

    let dir_is_called_tasks = absolute_path.ends_with("tasks");

    if dir_is_called_tasks && path_is_dir {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub(crate) fn is_valid_task_dir(path: &Path) -> Result<bool> {
    let absolute_path = path.canonicalize().map_err(|err| {
        format!(
            "Unable to get absolute path for `{}`. Reason: {}",
            path.display(),
            err
        )
    })?;

    // if absoluteabsolute_path.pop()

    Ok(false)
}
