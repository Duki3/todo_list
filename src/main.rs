use std::fs::{self, OpenOptions};
use std::io::{Write, stdin, stdout};
fn main() {
    let mut todo_file = OpenOptions::new()
        .create(true)
        .truncate(false)
        .append(true)
        .open("todo_list")
        .ok()
        .unwrap();

    file_reader();
    let _ = stdout().flush();
    let input = user_input();
    let _ = stdout().flush();
    let _ = writeln!(todo_file, "{}", input.trim_end());
    let _ = stdout().flush();

    file_reader();
}

fn user_input() -> String {
    let mut _input = String::new();
    println!("Please enter a Todoo");
    let _ = stdout().flush();
    stdin().read_line(&mut _input).unwrap();
    _input
}

fn file_reader() {
    let todo_list = fs::read_to_string("todo_list").expect("faild by read file");
    for s in todo_list.chars() {
        print!("{s}");
    }
}
