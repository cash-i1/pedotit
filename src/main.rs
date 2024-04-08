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
            
        .get_matches();
    
    
    match matches.subcommand() {
        Some(("add", submatches)) => {
            let name = submatches
                .get_one::<String>("task")
                .expect("you need to parse in atask");
            let desc = submatches
                .get_one::<String>("description")
                .expect("was not parsed");

            // println!("{}", "pedotit".cyan().italic().bold());
            // println!("{}", "added:".green());

            print!("{}", "+ ".green().bold());
            println!("{}", name.yellow().bold());

            print!("{}", "+ ".green().bold());
            println!("  {}", desc.bright_black().italic());

        }

        Some(("del", submatches)) => {
            let name = submatches
                .get_one::<String>("task")
                .expect("you need to parse in atask");

            let tasks = load_tasks();

            if let Some(task) = tasks.iter().find(|&task| &task.name == name) {
                println!("{:?}", task.name);
            } else {
                println!("{} task could not be found", "?".yellow().bold())
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
    let tasks: String = serde_json::to_string(&tasks).expect("msg");
    fs::write(TASKS_JSON_PATH, tasks);
}