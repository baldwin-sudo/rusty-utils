use database::{Credential, Database};
use prettytable::row;
use prettytable::Table;
use rusqlite::Connection;
use std::env;
use std::fs;
use std::io;
use std::io::stdin;
use std::io::Read;
use std::path::Path;
use std::process::exit;
use std::process::Command;
use utils::Action;
use utils::{compare_hashes, hash_master, vigenere_decrypt, vigenere_encrypt};
mod credentials;
mod database;
mod utils;

fn main() {
    print_help();
    let db_path = if let Ok(path) = get_db_path() {
        path
    } else {
        println!(" We don't support this os yet ...");
        exit(1)
    };
    let conn = rusqlite::Connection::open(db_path);
    let db = database::Database::setup(conn.unwrap());
    let exist = db.create_if_not_exist().unwrap();
    if !exist {
        create_master_key(&db);
    }
    println!("-enter master to log in :");
    let mut master_key = String::new();
    stdin().read_line(&mut master_key).unwrap();
    if db.check_master(master_key.clone()).unwrap() {
        main_loop(db, &master_key);
    } else {
        println!("master key incorrect , exiting the program now ...");
        exit(1);
    }
}
fn create_master_key(db: &Database) {
    println!("-The master key is the key that manages all your credentials , it won't be stored !");
    println!("Enter your master key :");
    let mut master_key = String::new();
    std::io::stdin().read_line(&mut master_key).unwrap();
    db.store_master_hash(master_key).unwrap();
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
fn main_loop(db: Database, master_key: &String) -> ! {
    let name = env::var("USER").unwrap_or(String::from("Baldwin"));
    println!("Welcome to Your Todolist,{name}");

    loop {
        let action = display_menu();

        match action {
            Ok(Action::ADD) => {
                let cred = Credential::create_credential().unwrap();
                db.insert_creds(cred, &master_key).unwrap();
            }
            Ok(Action::REMOVE) => {}
            Ok(Action::GETInfo) => {
                clear_terminal();
                println!("-Enter the id of the credential");

                let mut id = String::new();
                stdin().read_line(&mut id).unwrap();
                let cred = db.select_one_cred(id).unwrap();
                let mut table = Table::new();
                table.add_row(row!["id", "usage_desc", "username", "plain password"]);
                table.add_row(row![cred.id, cred.usage_desc, cred.username, cred.password]);
                table.printstd();
            }
            Ok(Action::DISPLAY) => {
                let creds = db.select_all_creds_usage_desc().unwrap();

                println!("Displaying creds ...");
                clear_terminal();
                // pretty table :
                let mut table = Table::new();
                table.add_row(row!["id", "usage_desc", "username", "encrypted password"]);
                if creds.is_empty() {
                    println!("No tasks in the TODO list.");
                } else {
                    for task in &creds {
                        table.add_row(row![task.id, task.usage_desc, task.username, task.password]);
                    }
                }
                table.printstd();
            }
            Ok(Action::QUIT) => {
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
     1 - Add New credentials \n\
     2 - Get credential info \n\
     3 - Remove credential \n\
     4 - Display All credentials \n\
     5 - Quit Menu\n"
    );
    println!("Enter your Action (1/2/3/4/5) :");
    let mut action = String::new();
    io::stdin().read_line(&mut action)?;
    let action = action.trim().parse::<u8>();
    match action {
        Ok(1) => Ok(Action::ADD),
        Ok(2) => Ok(Action::GETInfo),
        Ok(3) => Ok(Action::REMOVE),
        Ok(4) => Ok(Action::DISPLAY),
        Ok(5) => Ok(Action::QUIT),
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
fn get_db_path() -> Result<String, io::Error> {
    let os = env::consts::OS;

    let path_str = match os {
        "windows" => String::from("C://home/credentials.db"),
        "linux" => {
            let home = env::var("HOME").unwrap_or_else(|_| String::from("/tmp"));
            format!("{}/.credentials/credentials.db", home)
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
