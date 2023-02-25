use std::env;

use rugo::commands::run;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("No argument provided");
        return;
    }

    let command = &args[1];
    match command.as_str() {
        "run" => run(),
        _ => println!("unknown command")
    }
}
