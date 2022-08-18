use std::fs;

fn write_todo(data: &str) {
    match std::fs::write("/home/kevin/TODOS.md", "TODO: ".to_owned() + data) {
        Ok(_v) => println!("todo added!"),
        Err(e) => panic!("{}", e),
    }
}

fn list_todos() {
    match fs::read_to_string("/home/kevin/TODOS.md") {
        Ok(result) => {
            println!("YOUR TODOS: ");
            println!("{}", result);
        },
        Err(e) => panic!("{}", e),
    }
}

fn main() {
    let command = match std::env::args().nth(1) {
        Some(v) => v,
        None => panic!("Usage: ktdl list | add <content>"),
    };

    // let args: Vec<String> = std::env::args().collect();

    // println!("ARGS");

    match command.as_str() {
        "list" => list_todos(),
        "add" =>  {
            match std::env::args().nth(2) {
                Some(todo) => write_todo(todo.as_str()),
                None => panic!("Usage: ktdl list | add <content>"),
            };
            
        },
        _ => panic!("Invalid usage!"),
    }
}
