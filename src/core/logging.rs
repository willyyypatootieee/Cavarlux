use directories::ProjectDirs;
use std::fs::{self, File};
use std::sync::Arc;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn init() {
    let project_dirs =
        ProjectDirs::from("", "", "cavarlux").expect("Failed to determine project directories");

    let logs_dir = project_dirs.data_dir().join("logs");
    if !logs_dir.exists() {
        fs::create_dir_all(&logs_dir).expect("Failed to create logs directory");
    }

    let log_file = File::create(logs_dir.join("cavarlux.log")).expect("Failed to create log file");
    let file_writer = Arc::new(log_file);

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let stdout_log = fmt::layer().pretty();
    let file_log = fmt::layer().json().with_writer(file_writer);

    let _ = tracing_subscriber::registry()
        .with(filter)
        .with(stdout_log)
        .with(file_log)
        .try_init();
}
