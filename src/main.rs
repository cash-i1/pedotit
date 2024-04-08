use std::fs;

use clap::*;
use colored::Colorize;
use serde::*;

const TASKS_JSON_PATH: &str = "./src/tasks.json";

#[derive(Serialize, Deserialize)]
struct Task {
    name: String,
    description: String,
}

fn main() {
    let matches = Command::new("pedotit")
        .about("The best todo app in the an")
        .subcommand(
            Command::new("add")
                .about("Add a new todo item")
                .arg(
                    Arg::new("task")
                        .help("Name of the task")
                        .short('t')
                        .long("task")
                )
                .arg(
                    Arg::new("description")
                        .help("the description of the task")
                        .requires("task")
                        .required(false)
                        .short('d')
                        .long("description")
                )
            )
        .subcommand(
            Command::new("del")
                .about("Remove a todo item")
                .arg(
                    Arg::new("task")
                        .short('t')
                        .long("task")

                )
        )
        .subcommand(
            Command::new("list")
                .about("list all of the tasks")
        )
            
        .get_matches();
    
    
    match matches.subcommand() {
        Some(("add", submatches)) => {
            let name = submatches
                .get_one::<String>("task");
            let desc = submatches
                .get_one::<String>("description");

            if name.is_none() {
                println!("{} a task name was not supplied", "!!".red().bold());
                return;
            } else if name.unwrap().is_empty() {
                println!("{} task name is invalid ('{}')", "!!".red().bold(), name.unwrap());
                return;
            }

            let name = name.unwrap();
            let mut tasks = load_tasks();

            if let Some(task) = tasks.iter().find(|&task| &task.name == name) {
                println!("{} task '{}' already exists", "!!".red().bold(), task.name);
            } else {
                let mut new_task: Task = Task { name: name.to_string(), description: String::from("") };

                print!("{}", "+ ".green().bold());
                println!("{}", name.yellow().bold());
                
                if desc.is_some() {
                    new_task.description = desc.unwrap().to_string();
                    print!("{}", "+ ".green().bold());
                    println!("  {}", desc.unwrap().bright_black().italic());
                }

                tasks.push(new_task);
                write_tasks(tasks);
            }


        }

        Some(("del", submatches)) => {
            let name = submatches
                .get_one::<String>("task")
                .expect("you need to parse in atask");


            let mut tasks = load_tasks();

            if let Some(task) = tasks.iter().find(|&task| &task.name == name) {
                print!("{}", "- ".red().bold());
                println!("{}", name.yellow().bold());

                print!("{}", "- ".red().bold());
                println!("  {}", &task.description.bright_black().italic());

                let task_idx = tasks.iter().position(|t| &t.name == name).unwrap();
                tasks.remove(task_idx);
                write_tasks(tasks);

            } else {
                println!("{} task '{}' could not be found", "!!".red().bold(), name)
            }

        }
        _ => {
            panic!("all arguments exhausted.")
        }
    }
}

fn load_tasks() -> Vec<Task> {
    let s = fs::read_to_string(TASKS_JSON_PATH).expect("couldnt");
    let tasks: Vec<Task> = serde_json::from_str(s.as_str()).expect("msg");
    tasks
}

fn write_tasks(tasks: Vec<Task>) {
    let tasks: String = serde_json::to_string_pretty(&tasks).expect("msg");
    fs::write(TASKS_JSON_PATH, tasks);
}