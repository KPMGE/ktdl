use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;

const TODO_FILE: &str = "/home/kevin/TODOS.md";

fn write_todo(data: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(TODO_FILE)
        .unwrap();

    if let Err(e) = writeln!(file, "#TODO {}\n", data) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn list_todos() {
    match fs::read_to_string(TODO_FILE) {
        Ok(result) => {
            println!("YOUR TODOS: ");
            println!("{}", result);
        }
        Err(e) => panic!("{}", e),
    }
}

fn main() {
    let command = match std::env::args().nth(1) {
        Some(v) => v,
        None => {
            eprintln!("Usage: ktdl list | add <content>");
            std::process::exit(1);
        } 
    };

    match command.as_str() {
        "list" => list_todos(),
        "add" => {
            match std::env::args().nth(2) {
                Some(todo) => write_todo(todo.as_str()),
                None => eprintln!("Usage: ktdl list | add <content>"),
            };
        }
        _ => eprintln!("Usage: ktdl list | add <content>"),
    }
}
