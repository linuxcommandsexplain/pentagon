mod commands;

use std::{fs, io};
use crate::commands::help::show_help;

fn main() {
    let mut input = String::new();

    let motd = fs::read_to_string("./src/motd.txt").expect("Something went wrong reading the file");
    println!("{}", motd);

    while input != "quit".to_string() {
        println!("user@pentagon~# ");
        io::stdin()
            .read_line(&mut input)
            .expect("Something went wrong reading the input");

        match input.trim() {
            "test" => println!("this is test command"),
            "quit" => break,
            "help" => show_help(),
            _ => println!("Uknown input"),
        }
    }
}
