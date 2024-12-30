use async_trait::async_trait;
use clap::{ArgMatches, Command};
use std::collections::HashMap;

pub struct Cli {
    pub app: Command,
    pub commands: HashMap<String, Box<dyn CommandHandler>>,
}

impl Cli {
    pub fn new(name: &'static str, version: &'static str) -> Self {
        Self {
            app: Command::new(name).version(version),
            commands: HashMap::new(),
        }
    }

    pub fn register_command<H: CommandHandler + 'static>(&mut self, handler: H) {
        let command = handler.define();
        self.app = self.app.clone().subcommand(command);
        self.commands.insert(handler.name(), Box::new(handler));
    }

    pub async fn run(&self) {
        let matches = self.app.clone().get_matches();

        if let Some((subcommand, sub_matches)) = matches.subcommand() {
            if let Some(handler) = self.commands.get(subcommand) {
                handler.run(sub_matches).await;
            } else {
                eprintln!("Command not found: {}", subcommand);
            }
        } else {
            eprintln!("No subcommand provided.");
        }
    }
}

#[async_trait]
pub trait CommandHandler {
    fn name(&self) -> String;

    fn define(&self) -> Command;

    async fn run(&self, matches: &ArgMatches);
}
