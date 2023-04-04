use std::env;

use afskylia::commands::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("No argument provided");
        return;
    }

    let command = &args[1];
    match command.as_str() {
        "run" => run(&args),
        "build" => build(&args),
        "help" => println!("{}", HELP),
        "new" => new(&args),
        "version" => println!("0.0.1"),
        _ => println!("unknown command")
    }
}

static HELP: &str = r#"
afskylia is the main command, used to build your Afskylia site.

Afskylia is a Static Site Generator.

Usage:
  afskylia [flags]
  afskylia [command]

Available Commands:
  new         Create new content for your site
  run         A high performance webserver
  version     Print the version number of Hugo
  build       Build your site

Flags:
None yet.
  "#;
