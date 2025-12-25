use std::io;

fn main() {
    println!("Welcome to your Rust To-Do List!");
    println!("1. Add a Task");
    println!("2. View Tasks");
    println!("3. Mark Task as Completed");
    println!("4. Delete Task");
    println!("5. Exit");

    println!("\nEnter your choice:");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    println!("You chose: {}", choice.trim());
}

