pub mod migrate_cmd;
pub mod run_cmd;

use cli::{build_cli, Cli};

pub async fn run_command() {
    let cli = build_cli!(
        "server",
        "v1.0",
        migrate_cmd::MigrateCommand,
        run_cmd::RunCommand
    );
    cli.run().await;
}
