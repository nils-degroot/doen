use crate::app::*;
use crate::commands::*;

mod app;
mod commands;
mod todo;

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black = 0,
    Red,
    Green,
    Brown,
    Blue,
    Purple,
    Cyan,
    LightGray,
}

impl Color {
    pub fn fg_code(&self) -> u8 {
        (*self as u8) + 30
    }

    pub fn bg_code(&self) -> u8 {
        (*self as u8) + 40
    }
}

fn main() -> Result<(), String> {
    let cli = App::get();

    match cli.command.unwrap_or_default() {
        Command::Show { count } => show(ShowContext { count }),
        Command::Add { title, priority } => add(AddContext { title, priority }),
        Command::Remove => remove(),
    }
}
