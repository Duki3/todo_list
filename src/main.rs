use std::fs::{self, OpenOptions};
use std::io::{Write, stdin, stdout};

fn main() {
    let _ = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open("todo_list");

    file_reader();

    let input = user_input();

    file_wirter(input);
    file_reader();
}

fn user_input() -> String {
    let mut _input = String::new();
    println!("Please enter a Todoo");
    let _ = stdout().flush();
    stdin().read_line(&mut _input).unwrap().to_string()
}

fn file_reader() {
    let todo_list = fs::read_to_string("todo_list").expect("faild by read file");
    for s in todo_list.chars() {
        print!("{s}");
    }
}

fn file_wirter(input: String) {
    let _ = fs::write("todo_list", input);
}
// make file to string and add the todo at the and withe space and put it back in the file
