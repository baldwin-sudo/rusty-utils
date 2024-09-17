mod database;
use database::Priority;
use prettytable::{row, Table};

use database::Database;
use database::Task;
use rusqlite::{self};
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::{io, process::exit};
enum Action {
    Add,
    Remove,
    Display,
    Quit,
}

fn main() {
    print_help();
    let db_path = if let Ok(path) = get_db_path() {
        path
    } else {
        println!(" We don't support this os yet ...");
        exit(1);
    };
    let conn = rusqlite::Connection::open(db_path);
    let db = database::Database::setup(conn.unwrap());
    db.create_if_not_exist().unwrap();
    /* db.insert_task(task.unwrap());
    let tasks = db.select_all_tasks().unwrap();
    for t in tasks {
        println!("{}", t);
    } */
    main_loop(db);
}
fn get_db_path() -> Result<String, io::Error> {
    let os = env::consts::OS;

    let path_str = match os {
        "windows" => String::from("C://home/tasks.db"),
        "linux" => {
            let home = env::var("HOME").unwrap_or_else(|_| String::from("/tmp"));
            format!("{}/.todolist/tasks.db", home)
        }
        _ => return Err(io::Error::new(io::ErrorKind::Other, "OS not supported yet")),
    };

    let path = Path::new(&path_str);

    // Create the file if it doesn't exist
    if !path.exists() {
        if let Some(parent) = path.parent() {
            // Create the parent directory if it doesn't exist
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        // Create an empty file
        fs::File::create(path)?;
    }

    Ok(path_str)
}
fn print_help() {
    let args = env::args().collect::<String>();
    if args.contains("-h") || args.contains("--help") {
        println!(
            "Todolist CLI Tool\n\
         \n\
         USAGE:\t\t (comming soon ..)\n\
         \tcommand [OPTIONS]\n\
         \n\
         OPTIONS:\n\
         \t-h, --help     Prints this help information\n\
         \n\
         When run without any options, the tool will start an interactive loop.\n\
         In interactive mode, you can add, remove, or display tasks using the menu.\n"
        );
        exit(0);
    }
}
fn main_loop(db: Database) -> ! {
    let name = env::var("USER").unwrap_or(String::from("Baldwin"));
    println!("Welcome to Your Todolist,{name}");

    loop {
        let action = display_menu();

        match action {
            Ok(Action::Add) => {
                if let Ok(valid_task) = Task::create_task() {
                    println!("--Creating Task :--");
                    db.insert_task(valid_task);
                    clear_terminal();
                    println!("Task added successfully.");
                } else {
                    println!("Error adding task...");
                }
            }
            Ok(Action::Remove) => {
                println!("Removing a task...");
                let task_name = loop {
                    println!("Enter name of the task to remove (or type 'quit' to exit):");
                    let mut name = String::new();
                    if io::stdin().read_line(&mut name).is_err() {
                        println!("Error reading input.");
                        continue;
                    }
                    let name = name.trim().to_string();
                    if name.to_lowercase() == "quit" {
                        exit(0);
                    }
                    if !name.is_empty() {
                        break name;
                    } else {
                        println!("Task name cannot be empty. Please enter a valid task name.");
                    }
                };
                // TODO
                //db.delete_task();
                println!("Task '{}' removed successfully (if it existed).", task_name);
            }
            Ok(Action::Display) => {
                clear_terminal();
                println!("Displaying TODO list...");
                let todolist = db.select_all_tasks().unwrap_or_default();
                // pretty table :
                let mut table = Table::new();
                table.add_row(row!["name", "body", "priority"]);
                if todolist.is_empty() {
                    println!("No tasks in the TODO list.");
                } else {
                    for task in &todolist {
                        match task.priority {
                            Priority::High => {
                                table.add_row(row![task.name,task.name,Fr->"HIGH"]);
                            }
                            Priority::Medium => {
                                table.add_row(row![task.name,task.name,Fb->"MEDIUM"]);
                            }
                            Priority::Low => {
                                table.add_row(row![task.name, task.name, "LOW"]);
                            }
                        }
                    }
                    table.printstd();
                }
            }
            Ok(Action::Quit) => {
                println!("Quitting...");
                clear_terminal();
                exit(0);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn display_menu() -> Result<Action, io::Error> {
    println!(
        "###########################\n\
         #####  Actions Menu   #####\n\
     ###########################\n\
     1 - Create New Task\n\
     2 - Remove Task\n\
     3 - Display TODO List\n\
     4 - Quit Menu\n"
    );
    println!("Enter your Action (1/2/3/4) :");
    let mut action = String::new();
    io::stdin().read_line(&mut action)?;
    let action = action.trim().parse::<u8>();
    match action {
        Ok(1) => Ok(Action::Add),
        Ok(2) => Ok(Action::Remove),
        Ok(3) => Ok(Action::Display),
        Ok(4) => Ok(Action::Quit),
        Ok(_) => Err(io::Error::new(io::ErrorKind::Other, "INVALID ACTION!")),
        Err(_) => Err(io::Error::new(
            io::ErrorKind::Other,
            "ERROR READING ACTION!",
        )),
    }
}
fn clear_terminal() {
    let os = env::consts::OS;
    match os {
        "windows" => Command::new("cmd").args(&["/C", "cls"]).status().unwrap(),
        _ => Command::new("clear").status().unwrap(),
    };
}
