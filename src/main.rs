use std::io;

struct Task {
    description: String,
    completed: bool,
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

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
            "1" => {
                println!("Enter task description:");

                let mut description = String::new();
                io::stdin()
                    .read_line(&mut description)
                    .expect("Failed to read input");

                let task = Task {
                    description: description.trim().to_string(),
                    completed: false,
                };

                tasks.push(task);
                println!("Task added successfully!");
            }
            "2" => {
                if tasks.is_empty() {
                    println!("No tasks available.");
                } else {
                    println!("\nYour Tasks:");
                    for (index, task) in tasks.iter().enumerate() {
                        let status = if task.completed { "✓" } else { " " };
                        println!("{}. [{}] {}", index + 1, status, task.description);  
                    }
                }
            }
            "3" => {
                if tasks.is_empty() {
                    println!("No tasks to mark as completed.");
                } else {
                    println!("\nSelect a task number to mark as completed:");

                    for (index, task) in tasks.iter().enumerate() {
                        let status = if task.completed { "✓" } else { " " };
                        println!("{}. [{}] {}", index + 1, status, task.description);
                    }

                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read input");

                    let task_number: usize = match input.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Invalid number.");
                            continue;
                        }
                    };

                    if task_number == 0 || task_number > tasks.len() {
                        println!("Task number out of range.");
                    } else {
                        let task = &mut tasks[task_number - 1];
                        task.completed = true;
                        println!("Task marked as completed!");
                    }
                }   
            }

            "4" => println!("Delete Task selected"),
            "5" => {
                println!("Goodbye!");
                break 'menu;
            }
            _ => println!("Invalid choice, please try again"),
        }
    }
}


