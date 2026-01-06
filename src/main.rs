use std::fs;
use std::io::{self};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Task {
    description: String,
    completed: bool,
    priority: String,
    due_date: Option<String>,
}

const FILE_NAME: &str = "./tasks.json";

/* ---------- Persistence ---------- */

fn save_tasks(tasks: &Vec<Task>) {
    let json = serde_json::to_string_pretty(tasks).expect("Failed to serialize tasks");
    fs::write(FILE_NAME, json).expect("Failed to write tasks.json");
}

fn load_tasks() -> Vec<Task> {
    let data = fs::read_to_string(FILE_NAME).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
}

/* ---------- Menu ---------- */

fn print_menu() {
    println!("\n--- Rust To-Do List ---");
    println!("1. Add a Task");
    println!("2. View All Tasks");
    println!("3. Mark Task as Completed");
    println!("4. Delete Task");
    println!("5. Exit");
    println!("6. View Pending Tasks");
    println!("\nEnter your choice:");
}

/* ---------- Task Operations ---------- */

fn add_task(tasks: &mut Vec<Task>) {
    println!("Enter task description:");
    let mut description = String::new();
    io::stdin().read_line(&mut description).unwrap();

    println!("Enter priority (Low / Medium / High):");
    let mut priority = String::new();
    io::stdin().read_line(&mut priority).unwrap();
    let priority = match priority.trim().to_lowercase().as_str() {
        "high" => "High",
        "medium" => "Medium",
        _ => "Low",
    }.to_string();

    println!("Enter due date (YYYY-MM-DD) or leave blank:");
    let mut due_date = String::new();
    io::stdin().read_line(&mut due_date).unwrap();
    let due_date = if due_date.trim().is_empty() {
        None
    } else {
        Some(due_date.trim().to_string())
    };

    tasks.push(Task {
        description: description.trim().to_string(),
        completed: false,
        priority,
        due_date,
    });

    save_tasks(tasks);
    println!("Task added successfully!");
}

fn sort_tasks(tasks: &mut Vec<Task>) {
    tasks.sort_by(|a, b| {
        let priority_rank = |p: &str| match p {
            "High" => 0,
            "Medium" => 1,
            "Low" => 2,
            _ => 3,
        };

        priority_rank(&a.priority)
            .cmp(&priority_rank(&b.priority))
            .then_with(|| match (&a.due_date, &b.due_date) {
                (Some(a), Some(b)) => a.cmp(b),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                _ => std::cmp::Ordering::Equal,
            })
    });
}

fn view_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks available.");
        return;
    }

    let mut sorted_tasks = tasks.clone();
    sort_tasks(&mut sorted_tasks);

    println!("\nYour Tasks:");
    for (i, task) in sorted_tasks.iter().enumerate() {
        let status = if task.completed { "[x]" } else { "[ ]" };
        let due = task.due_date.clone().unwrap_or("-".to_string());
        println!(
            "{}. {} {} (Priority: {}, Due: {})",
            i + 1,
            status,
            task.description,
            task.priority,
            due
        );
    }
}

fn view_pending_tasks(tasks: &Vec<Task>) {
    let pending: Vec<&Task> = tasks.iter().filter(|t| !t.completed).collect();

    if pending.is_empty() {
        println!("No pending tasks 🎉");
        return;
    }

    println!("\nPending Tasks:");
    for (i, task) in pending.iter().enumerate() {
        let due = task.due_date.clone().unwrap_or("-".to_string());
        println!(
            "{}. {} (Priority: {}, Due: {})",
            i + 1,
            task.description,
            task.priority,
            due
        );
    }
}

fn mark_task_completed(tasks: &mut Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks to complete.");
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
    }
}

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
    }
}

/* ---------- Main ---------- */

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
            "6" => view_pending_tasks(&tasks),
            _ => println!("Invalid choice, try again."),
        }
    }
}
