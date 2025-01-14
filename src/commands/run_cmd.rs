use crate::app;
use async_trait::async_trait;
use clap::{Arg, ArgMatches, Command};
use cli::CommandHandler;

pub struct RunCommand;

#[async_trait]
impl CommandHandler for RunCommand {
    fn name(&self) -> String {
        "run".to_string()
    }

    fn define(&self) -> Command {
        Command::new("run").about("run app").arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_parser(clap::value_parser!(String))
                .help("config file path"),
        )
    }

    async fn run(&self, matches: &ArgMatches) {
        let config_file = matches.get_one::<String>("config").unwrap();
        let share_state = app::SharedState::new(config_file.clone().into()).await;

        share_state.run().await.unwrap();
    }
}
