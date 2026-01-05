use std::fs;
use std::io::{self, Write};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    description: String,
    completed: bool,
    priority: String,          // Low, Medium, High
    due_date: Option<String>,  // YYYY-MM-DD or None
}

const FILE_NAME: &str = "./tasks.json";

// Save all tasks to JSON file
fn save_tasks(tasks: &Vec<Task>) {
    let json = serde_json::to_string_pretty(tasks).expect("Failed to serialize tasks");
    fs::write(FILE_NAME, json).expect("Failed to write tasks.json");
}

// Load tasks from JSON file
fn load_tasks() -> Vec<Task> {
    let data = fs::read_to_string(FILE_NAME).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
}

// Print menu
fn print_menu() {
    println!("\n--- Rust To-Do List ---");
    println!("1. Add a Task");
    println!("2. View Tasks");
    println!("3. Mark Task as Completed");
    println!("4. Delete Task");
    println!("5. Exit");
    println!("\nEnter your choice:");
}

// Add a new task with priority and due date
fn add_task(tasks: &mut Vec<Task>) {
    println!("Enter task description:");
    let mut description = String::new();
    io::stdin().read_line(&mut description).unwrap();

    println!("Enter priority (Low / Medium / High):");
    let mut priority = String::new();
    io::stdin().read_line(&mut priority).unwrap();
    let priority = match priority.trim().to_lowercase().as_str() {
        "low" => "Low".to_string(),
        "medium" => "Medium".to_string(),
        "high" => "High".to_string(),
        _ => "Low".to_string(),
    };

    println!("Enter due date (YYYY-MM-DD) or leave blank:");
    let mut due_date = String::new();
    io::stdin().read_line(&mut due_date).unwrap();
    let due_date = {
        let trimmed = due_date.trim();
        if trimmed.is_empty() { None } else { Some(trimmed.to_string()) }
    };

    tasks.push(Task {
        description: description.trim().to_string(),
        completed: false,
        priority,
        due_date,
    });

    save_tasks(tasks);
    println!("Task added!");
}

// View all tasks with status, priority, and due date
fn view_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks available.");
    } else {
        println!("\nYour Tasks:");
        for (i, task) in tasks.iter().enumerate() {
            let status = if task.completed { "[x]" } else { "[ ]" };
            let due = task.due_date.clone().unwrap_or("-".to_string());
            println!(
                "{}. {} {} (Priority: {}, Due: {})",
                i + 1, status, task.description, task.priority, due
            );
        }
    }
}

// Mark a task as completed
fn mark_task_completed(tasks: &mut Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks to mark as completed.");
        return;
    }

    view_tasks(tasks);
    println!("Enter task number to mark as completed:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if let Ok(index) = input.trim().parse::<usize>() {
        if index > 0 && index <= tasks.len() {
            tasks[index - 1].completed = true;
            save_tasks(tasks);
            println!("Task marked as completed!");
        } else {
            println!("Invalid task number.");
        }
    } else {
        println!("Invalid input.");
    }
}

// Delete a task
fn delete_task(tasks: &mut Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks to delete.");
        return;
    }

    view_tasks(tasks);
    println!("Enter task number to delete:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if let Ok(index) = input.trim().parse::<usize>() {
        if index > 0 && index <= tasks.len() {
            tasks.remove(index - 1);
            save_tasks(tasks);
            println!("Task deleted!");
        } else {
            println!("Invalid task number.");
        }
    } else {
        println!("Invalid input.");
    }
}

fn main() {
    let mut tasks = load_tasks();

    loop {
        print_menu();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => add_task(&mut tasks),
            "2" => view_tasks(&tasks),
            "3" => mark_task_completed(&mut tasks),
            "4" => delete_task(&mut tasks),
            "5" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice, try again."),
        }
    }
}
