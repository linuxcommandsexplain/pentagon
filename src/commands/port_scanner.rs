use std::io;
use std::net::TcpStream;
use std::time::Duration;

pub fn launch_port_scanner() {
    let mut target_ip = String::new();
    let mut target_port = String::new();

    println!("Welcome to the port scanner!");

    println!("enter the target IP address: ");
    io::stdin().read_line(&mut target_ip).unwrap();

    println!("enter the target port to scan: ");
    io::stdin().read_line(&mut target_port).unwrap();

    let target = format!("{}:{}", target_ip.trim(), target_port.trim());

    println!("scanning {} on port {}", target_ip, target_port);
    if (TcpStream::connect(&target)).is_ok() {
        println!("successfully connected to {}", target);
    } else {
        println!("failed to connect to {}", target);
    }
}