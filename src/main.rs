use std::{fs, io};

fn main() {
    let mut input = String::new();

    let motd = fs::read_to_string("./src/motd.txt").expect("Something went wrong reading the file");
    println!("{}", motd);

    while true {
        println!("user@pentagon~# ");
        io::stdin()
            .read_line(&mut input)
            .expect("Something went wrong reading the input");

        match input.as_str() {
            "test" => println!("{}", motd.trim()),
            _ => println!("Uknown input"),
        }

    }
}
