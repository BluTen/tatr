use crate::Result;
use std::{borrow::Borrow, collections::HashMap, fs, path::Path, str::FromStr};

#[derive(Clone, Debug)]
pub(crate) struct Title(String);

impl Title {
    fn to_str(&self) -> &str {
        &self.0
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

impl Borrow<str> for Title {
    fn borrow(&self) -> &str {
        self.to_str() // Return a reference to the inner string
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Tag(String);

impl Tag {
    fn to_str(&self) -> &str {
        &self.0
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

impl Borrow<str> for Tag {
    fn borrow(&self) -> &str {
        self.to_str() // Return a reference to the inner string
    }
}

#[derive(Clone, Debug)]
pub(crate) enum Status {
    Open,
    Closed,
    Other(String),
}

impl Status {
    fn to_str(&self) -> &str {
        match self {
            Self::Open => "OPEN",
            Self::Closed => "CLOSED",
            Self::Other(s) => &s,
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

pub(crate) struct Task {
    title: Title,

    status: Status,
    tags: Vec<Tag>,
    priority: u8,

    other_properties: HashMap<String, String>,

    content: String,
}

impl Task {
    pub(crate) fn new(title: Title, status: Status, tags: Vec<Tag>, priority: u8) -> Self {
        Task {
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

        md.push_str(&format!("# {}\n\n", self.title.to_str()));

        md.push_str(&format!("- STATUS: {}\n", self.status.to_str()));
        md.push_str(&format!("- PRIORITY: {}\n", self.priority));
        md.push_str(&format!("- TAGS: {}\n", self.tags.join(", ")));

        for (property, value) in self.other_properties.iter() {
            md.push_str(&format!("- {property}: {value}\n"));
        }

        md.push_str(&format!("\n{}", self.content));

        md
    }

    pub(crate) fn save_to_file(&self, path: &Path) -> Result<()> {
        let md = self.to_markdown();

        fs::write(path, md).map_err(|err| {
            format!(
                "Error: Failed to write markdown file at {}\nReason: {}",
                path.display(),
                err
            )
        })?;

        Ok(())
    }
}
