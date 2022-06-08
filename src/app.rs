use clap::{Parser, Subcommand};

use crate::todo::Priority;

const DEFAULT_SHOW_COUNT: usize = 5;

#[derive(Debug, Parser)]
pub struct App {
    #[clap(subcommand)]
    pub command: Option<Command>,
}

impl App {
    pub fn get() -> Self {
        Self::parse()
    }
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Displays the active todos ordered by priority (default subcommand)
    Show {
        /// The count of todos to show, defaults to 5
        #[clap(short, long, default_value_t = DEFAULT_SHOW_COUNT)]
        count: usize,
    },
    /// Add a new todo
    Add {
        #[clap(short, long)]
        title: Option<String>,
        #[clap(short, long)]
        priority: Option<Priority>,
    },
    /// Removes a todo
    Remove,
}

impl Default for Command {
    fn default() -> Self {
        Self::Show {
            count: DEFAULT_SHOW_COUNT,
        }
    }
}
