use std::env;

use rhugo::commands::run;
use rhugo::commands::new;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("No argument provided");
        return;
    }

    let command = &args[1];
    match command.as_str() {
        "run" => run(),
        "build" => todo!(),
        "help" => todo!(),
        "new" => new(&args),
        _ => println!("unknown command")
    }
}
