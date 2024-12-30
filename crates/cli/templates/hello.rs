use crate::cli::CommandHandler;
use clap::{Arg, ArgMatches, Command};

pub struct HelloCommand;

impl CommandHandler for HelloCommand {
    fn name(&self) -> String {
        "hello".to_string()
    }

    fn define(&self) -> Command {
        Command::new("hello").about("Prints a greeting").arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .value_parser(clap::value_parser!(String))
                .help("Your name"),
        )
    }

    fn run(&self, matches: &ArgMatches) {
        let binding = "World".to_string();
        let name = matches.get_one::<String>("name").unwrap_or(&binding);
        println!("Hello, {}!", name);
    }
}
