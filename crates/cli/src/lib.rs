mod cli;

pub use cli::Cli;
pub use cli::CommandHandler;

#[macro_export]
macro_rules! build_cli {
    ($cli_name:expr, $cli_version:expr, $($command_name:expr),*) => {{
        let mut cli = Cli::new($cli_name, $cli_version);

        $(
            cli.register_command($command_name);
        )*

        cli
    }};
}
