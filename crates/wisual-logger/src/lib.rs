use std::io::Write;
use std::time::SystemTime;
use std::{io, thread};

use chrono::{DateTime, Utc};
use env_logger::fmt::{Color, Formatter};
use log::Record;

pub struct LogFormatter;

impl LogFormatter {
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

pub fn init_from_env() {
    let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format(|buf, record| LogFormatter::format(buf, record))
        .try_init();
}
