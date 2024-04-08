use clap::*;
use colored::Colorize;

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
            )
        .get_matches();
    
    
    let test = matches.subcommand_matches("add").unwrap();
    println!("Adding task '{}' to the DB", test.get_one::<String>("task").unwrap())
}
