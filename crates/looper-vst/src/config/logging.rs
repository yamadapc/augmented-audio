use std::path::{Path, PathBuf};

use log::{LevelFilter, SetLoggerError};
use log4rs::append::console::ConsoleAppender;
use log4rs::append::rolling_file::policy::compound::roll::delete::DeleteRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config::runtime::ConfigErrors;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Config;

#[derive(thiserror::Error, Debug)]
pub enum LoggingSetupError {
    #[error("Failed to create logging directory")]
    CreateLogDirectoryError(std::io::Error),
    #[error("Failed to set-up log-file appender")]
    FileAppenderError(std::io::Error),
    #[error("Failed to set-up logging configuration")]
    LogConfigError(#[from] ConfigErrors),
    #[error("Failed to set logger configuration")]
    SetLoggerError(#[from] SetLoggerError),
}

pub type Result<T> = std::result::Result<T, LoggingSetupError>;

fn ensure_logging_directory(root_config_path: &Path) -> Result<PathBuf> {
    std::fs::create_dir_all(root_config_path)
        .map_err(LoggingSetupError::CreateLogDirectoryError)?;
    Ok(root_config_path.to_path_buf())
}

pub fn configure_logging(root_config_path: &Path) -> Result<()> {
    let log_dir = ensure_logging_directory(root_config_path)?;
    let log_path = log_dir.join("looper-plugin.log");
    let logfile = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d} [{l}] {M}:{L} - {m} - tid:{T}:{t} pid:{P}\n",
        )))
        .build(
            log_path,
            Box::new(CompoundPolicy::new(
                Box::new(SizeTrigger::new(1024 * 1024 * 10)),
                Box::new(DeleteRoller::new()),
            )),
        )
        .map_err(LoggingSetupError::FileAppenderError)?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(Appender::builder().build("stdout", Box::new(ConsoleAppender::builder().build())))
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stdout")
                .build(LevelFilter::Info),
        )?;

    log4rs::init_config(config)?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ensure_logging_directory() {
        let _ = ensure_logging_directory(&PathBuf::from(dirs::home_dir().unwrap().join(".ruas")))
            .unwrap();
    }
}
