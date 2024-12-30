use logging;
use pop_up_city::commands;

pub const LOG_PATH: &str = "logs";

#[tokio::main]
async fn main() {
    logging::logging_init(LOG_PATH).unwrap();

    commands::run_command().await;
}
