use crate::database::migration::Migrator;
use async_trait::async_trait;
use clap::{Arg, ArgMatches, Command};
use cli::CommandHandler;
use db::setup_db;

pub struct MigrateCommand;

#[async_trait]
impl CommandHandler for MigrateCommand {
    fn name(&self) -> String {
        "migrate".to_string()
    }

    fn define(&self) -> Command {
        Command::new("migrate").about("migrate database").arg(
            Arg::new("db_url")
                .short('d')
                .long("db_url")
                .value_parser(clap::value_parser!(String))
                .help("database url"),
        )
    }

    async fn run(&self, matches: &ArgMatches) {
        let db_url = matches.get_one::<String>("db_url").unwrap();
        if let Ok(url) = url::Url::parse(db_url) {
            let db_name = url.path().trim_start_matches('/');
            let base_url = url.as_str().trim_end_matches(db_name);
            setup_db::<Migrator>(base_url, db_name).await.unwrap();
        }
    }
}
