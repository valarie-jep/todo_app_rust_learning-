use std::fs::OpenOptions;
use std::io::{self, Write, BufRead, BufReader};

#[derive(Debug)]
struct Task {
    description: String,
    completed: bool,
}

const FILE_NAME: &str = "./tasks.txt"; // Always in project root

fn save_tasks(tasks: &Vec<Task>) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true) // overwrite each time
        .open(FILE_NAME)
        .expect("Could not open tasks.txt for writing");

    for task in tasks {
        let line = format!("{}|{}\n", task.completed, task.description);
        file.write_all(line.as_bytes())
            .expect("Failed to write to tasks.txt");
    }
}

fn load_tasks() -> Vec<Task> {
    let mut tasks = Vec::new();

    if let Ok(file) = OpenOptions::new().read(true).open(FILE_NAME) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() == 2 {
                    let completed = parts[0] == "true";
                    let description = parts[1].to_string();
                    tasks.push(Task { description, completed });
                }
            }
        }
    }

    tasks
}

fn main() {
    let mut tasks = load_tasks();

    loop {
        println!("\n--- Rust To-Do List ---");
        println!("1. Add a Task");
        println!("2. View Tasks");
        println!("3. Mark Task as Completed");
        println!("4. Delete Task");
        println!("5. Exit");

        println!("\nEnter your choice:");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                println!("Enter task description:");
                let mut description = String::new();
                io::stdin().read_line(&mut description).unwrap();

                tasks.push(Task {
                    description: description.trim().to_string(),
                    completed: false,
                });

                save_tasks(&tasks);
                println!("Task added!");
            }

            "2" => {
                if tasks.is_empty() {
                    println!("No tasks available.");
                } else {
                    for (i, task) in tasks.iter().enumerate() {
                        let status = if task.completed { "[x]" } else { "[ ]" };
                        println!("{}. {} {}", i + 1, status, task.description);
                    }
                }
            }

            "3" => {
                if tasks.is_empty() {
                    println!("No tasks to mark as completed.");
                    continue;
                }

                println!("Enter task number to mark as completed:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();

                if let Ok(index) = input.trim().parse::<usize>() {
                    if index > 0 && index <= tasks.len() {
                        tasks[index - 1].completed = true;
                        save_tasks(&tasks);
                        println!("Task marked as completed!");
                    } else {
                        println!("Invalid task number.");
                    }
                } else {
                    println!("Invalid input.");
                }
            }

            "4" => {
                if tasks.is_empty() {
                    println!("No tasks to delete.");
                    continue;
                }

                println!("Enter task number to delete:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();

                if let Ok(index) = input.trim().parse::<usize>() {
                    if index > 0 && index <= tasks.len() {
                        tasks.remove(index - 1);
                        save_tasks(&tasks);
                        println!("Task deleted!");
                    } else {
                        println!("Invalid task number.");
                    }
                } else {
                    println!("Invalid input.");
                }
            }

            "5" => {
                println!("Goodbye!");
                break;
            }

            _ => println!("Invalid choice, try again."),
        }
    }
}
