use crate::{Result, utils};
use std::{collections::HashMap, fmt, fs, path::Path, str::FromStr};

#[derive(Clone, Debug)]
pub(crate) struct Title(String);

impl fmt::Display for Title {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl FromStr for Title {
    type Err = String;

    fn from_str(s: &str) -> Result<Self> {
        if s.contains("\n") {
            Err("Title must be single line, found '\\n'.".to_string())
        } else {
            Ok(Title(s.to_string()))
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Tag(String);

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl FromStr for Tag {
    type Err = String;

    /// Returns `Tag` while making sure the string is a valid identifier
    fn from_str(s: &str) -> Result<Self> {
        if s.is_empty() {
            return Err("Tag can't be empty string".to_string());
        }

        let mut chars = s.chars();
        let firstchar = chars.next().unwrap();

        if !firstchar.is_alphabetic() && firstchar != '_' {
            return Err(format!(
                "Tag '{s}' must begin with a alphabet or '_', not '{firstchar}"
            ));
        }

        if !chars.all(|c| c.is_alphanumeric() || c == '_') {
            return Err(format!(
                "Tag '{s}' should only contain alphanumeric and '_' charecters"
            ));
        }

        Ok(Tag(s.to_string()))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Status {
    Open,
    Closed,
    Other(String),
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Open => fmt::Display::fmt("OPEN", f),
            Self::Closed => fmt::Display::fmt("CLOSED", f),
            Self::Other(s) => fmt::Display::fmt(&s, f),
        }
    }
}

impl FromStr for Status {
    type Err = String;

    fn from_str(s: &str) -> Result<Self> {
        let capitalized = s.to_ascii_uppercase();

        match capitalized.as_str() {
            "OPEN" => Ok(Self::Open),
            "CLOSED" => Ok(Self::Closed),
            _ => Ok(Self::Other(capitalized)),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Task {
    title: Title,

    pub(crate) status: Status,
    tags: Vec<Tag>,
    pub(crate) priority: i32,

    other_properties: HashMap<String, String>,

    content: String,
}

impl Task {
    pub(crate) fn new(title: Title, status: Status, tags: Vec<Tag>, priority: i32) -> Self {
        Self {
            title,
            status,
            tags,
            priority,
            other_properties: HashMap::new(),
            content: String::new(),
        }
    }

    pub(crate) fn to_markdown(&self) -> String {
        let mut md = String::new();

        md.push_str(&format!("# {}\n\n", self.title));

        md.push_str(&format!("- STATUS: {}\n", self.status));
        md.push_str(&format!("- PRIORITY: {}\n", self.priority));
        md.push_str(&format!("- TAGS: "));
        for (i, tag) in self.tags.iter().enumerate() {
            if i > 0 {
                md.push_str(", ");
            }
            use std::fmt::Write;
            let _ = write!(md, "{}", tag);
        }
        md.push('\n');

        for (property, value) in self.other_properties.iter() {
            md.push_str(&format!("- {property}: {value}\n"));
        }

        md.push_str(&format!("\n{}", self.content));

        md
    }

    pub(crate) fn from_markdown(md: &str) -> Result<Self> {
        let mut lines = md.lines();

        let title_line = lines.next().ok_or("Markdown is empty")?;

        let title = title_line
            .strip_prefix("#")
            .ok_or("First line must be a header `# some task title`. Check tatr spec.")?
            .trim()
            .parse::<Title>()?;

        let mut status: Option<Status> = None;
        let mut tags: Vec<Tag> = vec![];
        let mut priority: Option<i32> = None;
        let mut other_properties: HashMap<String, String> = HashMap::new();
        let mut content_lines = vec![];

        // let mut parsing_properties = true;
        while let Some(line) = lines.next() {
            if line.trim().is_empty() {
                continue;
            }

            if let Some(property_line) = line.strip_prefix("-") {
                if let Some((property, value)) = property_line.trim().split_once(':') {
                    let property = property.trim();
                    let value = value.trim();
                    match property {
                        "STATUS" => status = Some(Status::from_str(value)?),
                        "PRIORITY" => {
                            priority = Some(value.parse().map_err(|err| {
                                format!("Priority must be an integer! Not `{}`: {}", value, err)
                            })?)
                        }

                        "TAGS" => tags.extend(
                            value
                                .split(',')
                                .map(|tag| tag.trim())
                                .filter(|tag| !tag.is_empty())
                                .map(|tag| tag.parse())
                                .collect::<Result<Vec<Tag>>>()?,
                        ),
                        _ => {
                            let _ = other_properties
                                .insert(property.to_ascii_uppercase(), value.to_string());
                        }
                    }
                }

                continue;
            }

            // We found something that doesnt start with '-' so we finished parsing properties
            // and now we eat everthing into the content string.
            content_lines.push(line);
            break;
        }

        content_lines.extend(lines);

        Ok(Self {
            title,
            status: status.ok_or("STATUS property not present")?,
            tags,
            priority: priority.ok_or("PRIORITY property not present")?,
            other_properties,
            content: content_lines.join("\n"),
        })
    }

    pub(crate) fn save_to_file(&self, path: &Path) -> Result<()> {
        let md = self.to_markdown();

        fs::write(path, md).map_err(|err| {
            format!(
                "Failed to write markdown file at {}\nReason: {}",
                path.display(),
                err
            )
        })?;

        Ok(())
    }

    pub(crate) fn from_file(file: &Path) -> Result<Self> {
        let file_contents = fs::read_to_string(file)
            .map_err(|err| format!("Unable to read file at {} Reason: {}", file.display(), err))?;

        let task = Self::from_markdown(file_contents.as_str())
            .map_err(|err| format!("Markdown Parsing Error: {}", err))?;

        Ok(task)
    }
}

pub(crate) fn summary_line(task: &Task, path: &str, huid: &str) -> String {
    let mut formatted_tags = String::new();
    if !task.tags.is_empty() {
        formatted_tags.push('[');
        if let Some((first, rest)) = task.tags.split_first() {
            formatted_tags.push_str(&first.0);

            for tag in rest {
                formatted_tags.push_str(", ");
                formatted_tags.push_str(&tag.0);
            }
        }
        formatted_tags.push(']');
    };

    format!(
        "{}{}/TASK.md:1: {:>6} [PRIORITY: {:3}] {} {}",
        path, huid, task.status, task.priority, formatted_tags, task.title
    )
}

pub(crate) fn read_all_tasks(tasks_path: &Path) -> Result<Vec<(String, Task)>> {
    let mut tasks: Vec<(String, Task)> = vec![];

    // if !utils::is_tasks_dir(tasks_path)? {
    //     return Err("Not a tasks directory. Stopped reading tasks.".to_string());
    // }

    let dir_childs = tasks_path.read_dir().map_err(|err| {
        format!(
            "Unable to get directory contents at {} Reason: {}",
            tasks_path.display(),
            err
        )
    })?;

    for child in dir_childs {
        let child = child.map_err(|err| {
            format!(
                "Failed while iterating {} directory. Reason {}",
                tasks_path.display(),
                err
            )
        })?;

        let child_type = match child.file_type() {
            Ok(ft) => ft,
            Err(e) => {
                eprintln!(
                    "WARNING: Skipped checking metadata of '{}'. Reason: {}",
                    child.path().display(),
                    e
                );
                continue;
            }
        };

        if !child_type.is_dir() {
            continue;
        }

        let os_string = child.file_name();

        let Some(task_huid) = os_string.to_str() else {
            eprintln!(
                "WARNING: Skipped '{}' as it has non-utf8 characters in name.",
                child.path().display(),
            );
            continue;
        };

        if !utils::is_valid_huid(task_huid) {
            continue;
        };

        // By now we know that child is a directory holding a TASK.md
        let task_file = child.path().join("TASK.md");

        let task = match Task::from_file(&task_file) {
            Ok(task) => task,
            Err(err) => {
                eprintln!("WARNING: Skipped '{}'. {}", child.path().display(), err);
                continue;
            }
        };
        tasks.push((task_huid.to_string(), task));
    }
    Ok(tasks)
}
