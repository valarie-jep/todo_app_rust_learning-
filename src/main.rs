use std::io;

fn main() {
    'menu: loop {
        println!("\n--- Rust To-Do List ---");
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

        match choice.trim() {
            "1" => println!("Add Task selected"),
            "2" => println!("View Tasks selected"),
            "3" => println!("Mark Task as Completed selected"),
            "4" => println!("Delete Task selected"),
            "5" => {
                println!("Goodbye!");
                break 'menu;
            }
            _ => println!("Invalid choice, please try again"),
        }
    }
}

