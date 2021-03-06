// Augmented Audio: Audio libraries and applications
// Copyright (c) 2022 Pedro Tacla Yamada
//
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
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
    CreateLogDirectory(std::io::Error),
    #[error("Failed to set-up log-file appender")]
    FileAppender(std::io::Error),
    #[error("Failed to set-up logging configuration")]
    LogConfig(#[from] ConfigErrors),
    #[error("Failed to set logger configuration")]
    SetLogger(#[from] SetLoggerError),
}

pub type Result<T> = std::result::Result<T, LoggingSetupError>;

fn ensure_logging_directory(root_config_path: &Path) -> Result<PathBuf> {
    std::fs::create_dir_all(root_config_path).map_err(LoggingSetupError::CreateLogDirectory)?;
    Ok(root_config_path.to_path_buf())
}

pub fn configure_logging(root_config_path: &Path, name: &str) -> Result<()> {
    let log_dir = ensure_logging_directory(root_config_path)?;
    let log_path = log_dir.join(name);
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
        .map_err(LoggingSetupError::FileAppender)?;

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
        let _ = ensure_logging_directory(&dirs::home_dir().unwrap().join(".ruas")).unwrap();
    }
}
