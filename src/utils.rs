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

    // Example HUID:
    // 20261215-163529-niansaysmeow
    // [   date-time ]-[  suffix  ]

    // Minimum length of HUID must be exactly 15 characters
    if bytes.len() < 15 {
        return false;
    }

    // validate just the [date-time] part
    let base_valid = bytes[8] == b'-'
        && bytes[0..8].iter().all(|b| b.is_ascii_digit())
        && bytes[9..15].iter().all(|b| b.is_ascii_digit());

    if !base_valid {
        return false;
    }

    // Early return if its just [date-time]
    if bytes.len() == 15 {
        return true;
    }

    // Else check that the next charecter is '-'
    if bytes[15] != b'-' {
        return false;
    }

    // And validate suffix to contain just [a-zA-Z0-9\-]
    bytes[16..]
        .iter()
        .all(|b| b.is_ascii_alphanumeric() || *b == b'-')
}

pub(crate) fn find_tasks_dir() -> Result<PathBuf> {
    let mut current_dir =
        env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))?;

    let mut depth = 0;
    loop {
        let potential_dir = current_dir.join("tasks");

        if is_tasks_dir(&potential_dir)? {
            return Ok(format!("{}tasks/", {
                if depth == 0 {
                    "./".to_string()
                } else {
                    "../".repeat(depth)
                }
            })
            .into());
        }

        if !current_dir.pop() {
            return Err("Could not find a 'tasks' directory in this folder or any parent folders. Did you run 'tatr init'? ".to_string());
        }

        depth += 1;
    }

    // for ancestor in current_dir.ancestors() {
    //     let potential_tasks_dir = ancestor.join("tasks");

    //     if is_tasks_dir(&potential_tasks_dir)? {
    //         return Ok(potential_tasks_dir);
    //     }
    // }
}

pub(crate) fn is_tasks_dir(path: &Path) -> Result<bool> {
    if !path.ends_with("tasks") {
        return Ok(false);
    }

    use std::io::ErrorKind;
    match path.metadata() {
        Ok(metadata) => Ok(metadata.is_dir()),

        Err(err) if err.kind() == ErrorKind::NotFound => Ok(false),

        Err(err) => Err(format!(
            "Unable to read metadata of path `{}`. Reason: {}",
            path.display(),
            err
        )),
    }
}
