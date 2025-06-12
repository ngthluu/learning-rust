use std::env;

mod db;
mod file;

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        return Err("Your command is invalid. Use `help` for information.".to_string());
    }

    match args[1].as_str() {
        "stats" => {}
        "help" => {}
        "list" => {
            let db = db::ToDoList::load()?;
            println!("ID, Timestamp, Content, Status");
            println!("-----------------------");
            db.get_list()
                .iter()
                .all(|value| {
                    let key = value.0;
                    let record = value.1;
                    println!("{}, {:?}, {}, {}", key, record.timestamp, record.content, record.status);
                    true
                });
        }
        "add" => {
            if args.len() <= 2 {
                return Err(format!("Command `{}` require <content>. Use `help` for information.", args[1]));
            }
            let mut db = db::ToDoList::load()?;
            let key = db.add(args[2].clone())?;
            println!("Add item successfully. New key: {}", key);
        }
        "edit" => {
            if args.len() <= 3 {
                return Err(format!("Command `{}` require <id>, <content>. Use `help` for information.", args[1]));
            }
            let mut db = db::ToDoList::load()?;
            db.edit(args[2].clone(), args[3].clone())?;
            println!("Edit item {} successfully.", args[2]);
        }
        "check" => {
            if args.len() <= 2 {
                return Err(format!("Command `{}` require <id>. Use `help` for information.", args[1]));
            }
            let mut db = db::ToDoList::load()?;
            db.check(args[2].clone())?;
            println!("Check item {} successfully.", args[2]);
        }
        "delete" => {
            if args.len() <= 2 {
                return Err(format!("Command `{}` require <id>. Use `help` for information.", args[1]));
            }
            let mut db = db::ToDoList::load()?;
            db.delete(args[2].clone())?;
            println!("Delete item {} successfully.", args[2]);
        }
        _ => {
            return Err(format!("Unknown command: {}. Use `help` for information.", args[1]));
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
