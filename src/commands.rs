use crate::todo::*;
use std::io::{self, Write};

lazy_static::lazy_static! {
    static ref STORAGE: TodoPersistance = TodoPersistance::new();
}

macro_rules! println_colored {
    ($color:expr, $text:expr) => {
        let text = format!("{}", $text);
        println!("\x1b[{}m{}\x1b[0m", $color, text);
    };
}

pub struct ShowContext {
    pub count: usize,
}

pub fn show(context: ShowContext) -> Result<(), String> {
    let mut todos = STORAGE.read_todos().map_err(|e| e.to_string())?;
    if todos.is_empty() {
        return Ok(());
    }

    todos.sort_by_key(|todo| todo.priority);

    let top = todos.into_iter().take(context.count).collect::<Vec<_>>();
    let mut last_priority = None;

    for todo in top {
        if Some(todo.priority) != last_priority {
            let prio_text = if last_priority == None {
                todo.priority.to_string()
            } else {
                format!("\n{}", todo.priority.to_string())
            };

            println_colored!(todo.priority.color().fg_code(), prio_text);
            last_priority = Some(todo.priority)
        }

        println!("- {}", todo.title);
    }

    Ok(())
}

pub struct AddContext {
    pub title: Option<String>,
    pub priority: Option<Priority>,
}

pub fn add(context: AddContext) -> Result<(), String> {
    let mut todos = STORAGE.read_todos().map_err(|e| e.to_string())?;

    let title = context
        .title
        .ok_or_else(|| {
            let mut title = String::new();
            loop {
                print!("Title of the todo: ");
                io::stdout()
                    .flush()
                    .map_err(|_| "Failed to flush to stdout")?;

                if io::stdin().read_line(&mut title).is_ok() {
                    title = title.trim().to_string();
                    break;
                };
            }
            Ok::<_, String>(title)
        })
        .expect("Failed to flush to stdout");

    let priority = context.priority.unwrap_or_else(|| loop {
        let mut priority = String::new();

        for priority in Priority::all() {
            println!("{}. {}", priority as u8, priority.to_string());
        }

        print!("Priority of the todo (default Trivial): ");
        io::stdout().flush().expect("Failed to flush to stdout");

        if io::stdin().read_line(&mut priority).is_err() {
            continue;
        };

        if priority.trim().is_empty() {
            break Priority::Trivial;
        }

        if let Ok(idx) = priority.trim().parse::<usize>() {
            if let Ok(priority) = Priority::try_from(idx) {
                break priority;
            }
        }
    });

    todos.push(Todo { title, priority });
    STORAGE.override_todos(todos).map_err(|e| e.to_string())
}

pub fn remove() -> Result<(), String> {
    let mut todos = STORAGE.read_todos().map_err(|e| e.to_string())?;
    todos.sort_by_key(|todo| todo.priority);

    let mut input_buff = String::new();

    let idx = loop {
        for (idx, todo) in todos.iter().enumerate() {
            println!(
                "{}. {} ({})",
                idx + 1,
                todo.title,
                todo.priority.to_string()
            );
        }

        print!("Select an index to remove: ");
        io::stdout().flush().expect("Failed to flush to stdout");

        if io::stdin().read_line(&mut input_buff).is_err() {
            continue;
        };

        if let Ok(idx) = input_buff.trim().parse::<usize>() {
            break idx - 1;
        }
    };

    todos.remove(idx);
    STORAGE.override_todos(todos).map_err(|e| e.to_string())
}
