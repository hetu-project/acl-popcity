use chrono::Local;
use once_cell::sync::OnceCell;
use tracing_appender::{
    non_blocking::WorkerGuard,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

static INSTANCE: OnceCell<WorkerGuard> = OnceCell::new();

pub const LOG_TIME_FORMAT: &str = "%Y-%m-%d_%H-%M-%S";
pub const LOG_KEY_ENV: &str = "RUST_LOG";
pub const LOG_DEFAULT_LEVEL: &str = "info";
pub const LOG_BASE_NAME: &str = "app";

pub fn logging_init(log_dir: &str) -> Result<(), String> {
    if cfg!(debug_assertions) {
        // Get the logging level from the environment or use the default.
        let env_log_level =
            std::env::var(LOG_KEY_ENV).unwrap_or_else(|_| LOG_DEFAULT_LEVEL.to_string());
        let env_filter_layer = EnvFilter::new(env_log_level);

        // Create a rolling file appender that does not rotate automatically.
        let file_appender = RollingFileAppender::new(
            Rotation::NEVER,
            log_dir,
            format!(
                "{}_{}.log",
                Local::now().format(LOG_TIME_FORMAT),
                LOG_BASE_NAME
            ),
        );
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

        // Define a logging layer for writing to log files with timestamps and line numbers.
        let file_layer = fmt::Layer::default()
            .with_writer(non_blocking)
            .with_line_number(true)
            .with_ansi(false); // Disable ANSI colors for log files.
                               // Define a logging layer for console output with timestamps and line numbers.
        let stdout_layer = fmt::Layer::default()
            .with_writer(std::io::stdout)
            .with_line_number(true);

        // Create a tracing subscriber with environment-based filtering and layered output.
        tracing_subscriber::registry()
            .with(env_filter_layer)
            .with(stdout_layer)
            .with(file_layer)
            .init();

        INSTANCE
            .set(_guard)
            .map_err(|_e| "once_cell set gurad error".to_string())?;
    } else {
        // Get the logging level from the environment or use the default.
        let env_log_level =
            std::env::var(LOG_KEY_ENV).unwrap_or_else(|_| LOG_DEFAULT_LEVEL.to_string());
        let env_filter_layer = EnvFilter::new(env_log_level);

        // Create a rolling file appender that does not rotate automatically.
        let file_appender = RollingFileAppender::new(
            Rotation::NEVER,
            log_dir,
            format!(
                "{}_{}.log",
                Local::now().format(LOG_TIME_FORMAT),
                LOG_BASE_NAME
            ),
        );
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

        // Define a logging layer for writing to log files with timestamps and line numbers.
        let file_layer = fmt::Layer::default()
            .with_writer(non_blocking)
            .with_line_number(true)
            .with_ansi(false); // Disable ANSI colors for log files.
        tracing_subscriber::registry()
            .with(env_filter_layer)
            .with(file_layer)
            .init();

        INSTANCE
            .set(_guard)
            .map_err(|_e| "once_cell set gurad error".to_string())?;
    }

    Ok(())
}
