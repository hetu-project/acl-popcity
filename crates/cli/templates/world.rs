use crate::cli::CommandHandler;
use clap::{ArgMatches, Command};

pub struct WorldCommand;

impl CommandHandler for WorldCommand {
    fn name(&self) -> String {
        "world".to_string()
    }

    fn define(&self) -> Command {
        Command::new("world").about("Prints 'World'")
    }

    fn run(&self, _matches: &ArgMatches) {
        println!("World!");
    }
}
