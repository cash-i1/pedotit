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

            print!("{}", "- ".red().bold());
            println!("{}", name.yellow().bold());

            print!("{}", "- ".red().bold());
            println!("  {}", "placeholder i dont have database".bright_black().italic());


        }
        _ => {
            panic!("all arguments exhausted.")
        }
    }
}
