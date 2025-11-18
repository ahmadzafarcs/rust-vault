mod commands;
mod model;
mod storage;

use commands::{add, delete, get, list};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    let command = args[1].as_str();

    match command {
        "add" => {
            if args.len() != 5 {
                println!("Usage: add <service> <username> <password>");
            } else {
                add(&args[2], &args[3], &args[4]);
            }
        }
        "list" => {
            list();
        }
        "get" => {
            if args.len() != 3 {
                println!("Usage: get <service>");
            } else {
                get(&args[2]);
            }
        }
        "delete" => {
            if args.len() != 3 {
                println!("Usage: delete <service>");
            } else {
                delete(&args[2]);
            }
        }
        _ => {
            print_help();
        }
    }
}

fn print_help() {
    println!("Password CLI");
    println!("Usage:");
    println!("  add <service> <username> <password>  - Add a new entry");
    println!("  list                                 - List all entries");
    println!("  get <service>                        - Get password for a service");
    println!("  delete <service>                     - Delete an entry");
}
