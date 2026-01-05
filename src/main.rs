use std::io;

struct Task {
    description: String,
    completed: bool,
}

fn print_menu() {
    println!("\n--- Rust To-Do List ---");
    println!("1. Add a Task");
    println!("2. View Tasks");
    println!("3. Mark Task as Completed");
    println!("4. Delete Task");
    println!("5. Exit");
    println!("\nEnter your choice:");
}

fn add_task(tasks: &mut Vec<Task>) {
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

fn view_tasks(tasks: &Vec<Task>) {
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

fn mark_task_completed(tasks: &mut Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks to mark as completed.");
        return;
    }

    view_tasks(tasks);
    println!("Enter task number to mark as completed:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");

    let task_number: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number.");
            return;
        }
    };

    if task_number == 0 || task_number > tasks.len() {
        println!("Task number out of range.");
    } else {
        tasks[task_number - 1].completed = true;
        println!("Task marked as completed!");
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
    io::stdin().read_line(&mut input).expect("Failed");

    let task_number: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number.");
            return;
        }
    };

    if task_number == 0 || task_number > tasks.len() {
        println!("Task number out of range.");
    } else {
        tasks.remove(task_number - 1);
        println!("Task deleted successfully!");
    }
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        print_menu();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim() {
            "1" => add_task(&mut tasks),
            "2" => view_tasks(&tasks),
            "3" => mark_task_completed(&mut tasks),
            "4" => delete_task(&mut tasks),
            "5" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice, please try again"),
        }
    }
}
