use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use regex::Regex;

const TODO_FILE: &str = "/home/kevin/TODOS.md";

fn add_todo(data: &str) {
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

fn remove_todo(idx: usize) {
  // regex to match todos
  let re = Regex::new(r"^#").unwrap();
  let mut todos = Vec::<&str>::new();

  // get contents ot the file
  let result = match fs::read_to_string(TODO_FILE) {
    Ok(result) => result,
    Err(e) => panic!("{}", e),
  };

  for r in result.as_str().rsplit('\n') {
    if re.is_match(r) {
      todos.push(r);
    }
  }

  if todos.len() == 1 {
    if let Err(e) = std::fs::write(TODO_FILE, "") {
      panic!("{}", e);
    }
  }

  for (i, td) in todos.iter().rev().enumerate() {
    if i == idx {
      println!("todo: {td} gone!");
      continue;
    }
    if let Err(e) = std::fs::write(TODO_FILE, format!("{}\n", td)) {
      panic!("{}", e);
    }
  }
}

fn main() {
  let command = match std::env::args().nth(1) {
    Some(v) => v,
    None => {
      eprintln!("Usage: ktdl (list | add | rm) <content | idx>");
      std::process::exit(1);
    } 
  };

  match command.as_str() {
    "list" => list_todos(),
    "rm" => {
      match std::env::args().nth(2) {
        Some(num) => {
          let idx = num.parse::<usize>().unwrap();
          remove_todo(idx);
        }, 
        None => eprintln!("Usage: ktdl (list | add | rm) <content | idx>"),
      };
    },
    "add" => {
      match std::env::args().nth(2) {
        Some(todo) => add_todo(todo.as_str()),
        None => eprintln!("Usage: ktdl (list | add | rm) <content | idx>"),
      };
    }
    _ => eprintln!("Usage: ktdl (list | add | rm) <content | idx>"),
  }
}
