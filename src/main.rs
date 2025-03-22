use std::{fs, env, process};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    done: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: rust-todo <add/list/done>");
        process::exit(1);
    }

    let command = args[1].as_str();
    let mut tasks = load_tasks();

    match command {
        "add" => {
            if args.len() < 3 {
                println!("Provide a task description.");
                process::exit(1);
            }
            let description = args[2..].join(" ");
            tasks.push(Task { description, done: false });
            save_tasks(&tasks);
            println!("✅ Task added!");
        }
        "list" => {
            if tasks.is_empty() {
                println!("📭 No tasks yet!");
            } else {
                for (i, task) in tasks.iter().enumerate() {
                    let status = if task.done { "✔️" } else { "❌" };
                    println!("{}. [{}] {}", i + 1, status, task.description);
                }
            }
        }
        "done" => {
            if args.len() < 3 {
                println!("Provide a task number to mark as done.");
                process::exit(1);
            }
            let task_index: usize = args[2].parse().unwrap_or(0) - 1;
            if task_index >= tasks.len() {
                println!("Invalid task number.");
                process::exit(1);
            }
            tasks[task_index].done = true;
            save_tasks(&tasks);
            println!("🎉 Task marked as done!");
        }
        _ => println!("Invalid command. Use 'add', 'list', or 'done'."),
    }
}

fn load_tasks() -> Vec<Task> {
    let filename = "tasks.json";
    if let Ok(content) = fs::read_to_string(filename) {
        serde_json::from_str(&content).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}

fn save_tasks(tasks: &Vec<Task>) {
    let filename = "tasks.json";
    fs::write(filename, serde_json::to_string_pretty(tasks).unwrap()).expect("Failed to save tasks");
}
