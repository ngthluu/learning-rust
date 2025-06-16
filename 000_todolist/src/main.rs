use crate::date::format_datetime;
use std::env;

mod date;
mod db;
mod file;

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        return Err("Your command is invalid. Use `help` for information.".to_string());
    }

    match args[1].as_str() {
        "stats" => {
            let db = db::ToDoList::load()?;
            let done = db.count_by_status(db::ToDoListRecordStatus::Done);
            let in_progress = db.count_by_status(db::ToDoListRecordStatus::InProgress);
            println!(
                "Number of task DONE: {} ({}%)",
                done,
                100 * done / (done + in_progress)
            );
            println!(
                "Number of task IN-PROGRESS: {} ({}%)",
                in_progress,
                100 * in_progress / (done + in_progress)
            );
        }
        "help" => {
            println!("To do list");
            println!("- stats: Display the number and distribution percentage of tasks");
            println!("- help: ");
            println!("- list: List all to-do tasks");
            println!("- add \"<content>\": Add new task");
            println!("- edit <id> \"<content>\": Edit an existing task");
            println!("- check <id>: Mark a task as completed");
            println!("- delete <id>: Soft-delete a task");
        }
        "list" => {
            let db = db::ToDoList::load()?;
            println!(
                "{:<10} {:<20} {:<12} {:<20}",
                "ID", "Timestamp", "Status", "Content"
            );
            db.get_list().iter().all(|value| {
                let (key, record) = (value.0, value.1);
                let timestamp = format_datetime(record.timestamp).unwrap();
                println!(
                    "{:<10} {:<20} {:<12} {:<20}",
                    key, timestamp, record.status, record.content
                );
                true
            });
        }
        "add" => {
            let content = args.get(2).ok_or_else(|| {
                format!(
                    "Command `{}` requires <content>. Use `help` for information.",
                    args[1]
                )
            })?;

            let mut db = db::ToDoList::load()?;
            let key = db.add(content.to_string())?;
            println!("Add item successfully. New key: {}", key);
        }
        "edit" => {
            let (key, content) = args.get(2).zip(args.get(3)).ok_or_else(|| {
                format!(
                    "Command `{}` require <id>, <content>. Use `help` for information.",
                    args[1]
                )
            })?;

            let mut db = db::ToDoList::load()?;
            db.edit(key.to_string(), content.to_string())?;
            println!("Edit item {} successfully.", args[2]);
        }
        "check" => {
            let key = args.get(2).ok_or_else(|| {
                format!(
                    "Command `{}` require <id>. Use `help` for information.",
                    args[1]
                )
            })?;
            let mut db = db::ToDoList::load()?;
            db.check(key.to_string())?;
            println!("Check item {} successfully.", args[2]);
        }
        "delete" => {
            let key = args.get(2).ok_or_else(|| {
                format!(
                    "Command `{}` require <id>. Use `help` for information.",
                    args[1]
                )
            })?;
            let mut db = db::ToDoList::load()?;
            db.delete(key.to_string())?;
            println!("Delete item {} successfully.", args[2]);
        }
        _ => {
            return Err(format!(
                "Unknown command: {}. Use `help` for information.",
                args[1]
            ));
        }
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error occurred: {}", err);
        std::process::exit(1);
    }
}
