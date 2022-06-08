use crate::Color;
use serde::{Deserialize, Serialize};

use std::{
    fs::{self, File},
    path::PathBuf,
    str::FromStr,
};

lazy_static::lazy_static! {
    static ref TODO_FILE: PathBuf = {
        let mut target = dirs::home_dir().expect("Failed to find home directory");

        target.push(".config");
        if !target.exists() {
            fs::create_dir(target.clone()).expect("Failed to create $HOME/.config");
        }

        target.push("doen");
        if !target.exists() {
            fs::create_dir(target.clone()).expect("Failed to create $HOME/.config/doen");
        }

        target.push("todo.yml");

        if target.exists() && target.is_dir() {
            panic!("$HOME/.config/doen/todo.yml cannot be a directory");
        } else if !target.exists() {
            File::create(target.clone()).expect("Failed to create todo file");
        }

        target
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub title: String,
    #[serde(default = "Priority::default")]
    pub priority: Priority,
}

#[derive(Debug, Clone, Copy)]
pub enum PriorityError {
    InvalidVariant,
}

impl std::fmt::Display for PriorityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PriorityError::InvalidVariant => "Invalid variant of Priority",
            }
        )
    }
}

impl std::error::Error for PriorityError {}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
pub enum Priority {
    Important = 1,
    Trivial,
    Minor,
}

impl Priority {
    const IMPORTANT: &'static str = "Important";
    const TRIVIAL: &'static str = "Trivial";
    const MINOR: &'static str = "Minor";

    pub fn color(&self) -> Color {
        match self {
            Priority::Important => Color::Red,
            Priority::Trivial => Color::Blue,
            Priority::Minor => Color::Green,
        }
    }

    pub fn all() -> Vec<Priority> {
        vec![Priority::Important, Priority::Trivial, Priority::Minor]
    }
}

impl TryFrom<usize> for Priority {
    type Error = PriorityError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let result = match value {
            1 => Self::Important,
            2 => Self::Trivial,
            3 => Self::Minor,
            _ => Err(PriorityError::InvalidVariant)?,
        };

        Ok(result)
    }
}

impl ToString for Priority {
    fn to_string(&self) -> String {
        match self {
            Priority::Important => Self::IMPORTANT,
            Priority::Trivial => Self::TRIVIAL,
            Priority::Minor => Self::MINOR,
        }
        .to_string()
    }
}

impl FromStr for Priority {
    type Err = PriorityError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = match s {
            Self::IMPORTANT => Priority::Important,
            Self::TRIVIAL => Priority::Trivial,
            Self::MINOR => Priority::Minor,
            _ => Err(PriorityError::InvalidVariant)?,
        };

        Ok(result)
    }
}

impl Default for Priority {
    fn default() -> Self {
        Self::Trivial
    }
}

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (*self as u8).partial_cmp(&(*other as u8))
    }
}

impl Ord for Priority {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u8).cmp(&(*other as u8))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TodoPersistanceError {
    Serialisation,
    FileWrite,
    FileRead,
}

impl ToString for TodoPersistanceError {
    fn to_string(&self) -> String {
        match self {
            TodoPersistanceError::Serialisation => "Failed to serialize todos",
            TodoPersistanceError::FileWrite => "Failed to write todos",
            TodoPersistanceError::FileRead => "Failed to read todos",
        }
        .to_string()
    }
}

pub struct TodoPersistance(());

impl TodoPersistance {
    pub fn new() -> Self {
        Self(())
    }

    pub fn override_todos(&self, todos: Vec<Todo>) -> Result<(), TodoPersistanceError> {
        let updated_file =
            serde_yaml::to_string(&todos).map_err(|_| TodoPersistanceError::Serialisation)?;

        std::fs::write(TODO_FILE.as_path(), updated_file)
            .map_err(|_| TodoPersistanceError::FileWrite)
    }

    pub fn read_todos(&self) -> Result<Vec<Todo>, TodoPersistanceError> {
        let todos = std::fs::read_to_string(TODO_FILE.as_path())
            .map_err(|_| TodoPersistanceError::FileRead)?;

        if todos.trim().is_empty() {
            Ok(vec![])
        } else {
            serde_yaml::from_str::<Vec<Todo>>(&todos)
                .map_err(|_| TodoPersistanceError::Serialisation)
        }
    }
}
