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

//! [![crates.io](https://img.shields.io/crates/v/wisual-logger.svg)](https://crates.io/crates/wisual-logger)
//! [![docs.rs](https://docs.rs/wisual-logger/badge.svg)](https://docs.rs/wisual-logger/)
//! - - -
//! Just a pretty printer configuration for `env_logger`.
//!
//! ```ignore
//! fn main() {
//!     wisual_logger::init_from_env();
//!     log::info!("Hello world");
//! }
//! ```
//!
//! Will output:
//! ![](https://raw.githubusercontent.com/yamadapc/augmented-audio/master/crates/augmented/ops/wisual-logger/screenshot.png)
//!
//! ```shell
//! INFO [2021-07-09T02:26:16.239338+00:00] (main@hello_world) Hello world
//! ```

use std::io::Write;
use std::time::SystemTime;
use std::{io, thread};

use chrono::{DateTime, Utc};
use env_logger::fmt::{Color, Formatter};
use log::{Record, SetLoggerError};

/// A log pretty printer that will output in colors with thread name and pid
pub struct LogFormatter;

impl LogFormatter {
    /// Output log message with level, time, thread & pid
    pub fn format(buf: &mut Formatter, record: &Record) -> io::Result<()> {
        let metadata = record.metadata();
        let target = metadata.target();
        let time: DateTime<Utc> = SystemTime::now().into();
        let time = time.format("%+");
        let current_thread = thread::current();
        let thread_name = current_thread
            .name()
            .map(|s| s.to_string())
            .unwrap_or(format!("tid-{:?}", current_thread.id()));

        let level_style = buf.default_styled_level(record.level());
        let time_style = buf
            .style()
            .set_color(Color::Black)
            .set_intense(true)
            .clone();
        let thread_name_style = buf
            .style()
            .set_color(Color::Magenta)
            .set_intense(false)
            .clone();
        let args_style = buf
            .style()
            .set_color(Color::White)
            .set_intense(true)
            .clone();
        let target_style = buf.style().set_color(Color::Cyan).set_intense(true).clone();

        writeln!(
            buf,
            "{} [{}] ({}@{}) {}",
            level_style,
            time_style.value(time),
            thread_name_style.value(thread_name),
            target_style.value(target),
            args_style.value(record.args())
        )
    }
}

/// Try to set-up the logger and return a result
pub fn try_init_from_env() -> Result<(), SetLoggerError> {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info,wgpu_core=off"),
    )
    .format(LogFormatter::format)
    .try_init()
}

/// Try to set-up the logger and ignore errors
pub fn init_from_env() {
    let _ = try_init_from_env();
}
