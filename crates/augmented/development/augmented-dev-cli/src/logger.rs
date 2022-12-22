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

use std::io;
use std::io::Write;
use std::sync::atomic::AtomicI64;

use env_logger::fmt::{Color, Formatter};
use log::{Record, SetLoggerError};

pub struct LogFormatter;

impl LogFormatter {
    pub fn format(buf: &mut Formatter, record: &Record, last_time: &AtomicI64) -> io::Result<()> {
        let level_style = buf.default_styled_level(record.level());
        let timestamp = chrono::Local::now().timestamp_millis();
        let elapsed = timestamp - last_time.swap(timestamp, std::sync::atomic::Ordering::SeqCst);
        let mut style = buf.style();
        let elapsed_str = {
            let elapsed_str = if elapsed == 0 {
                format!("(   --   )")
            } else {
                format!("(+{:5.0}ms)", elapsed)
            };
            if elapsed > 100 {
                style
                    .set_dimmed(true)
                    .set_color(Color::Red)
                    .value(elapsed_str)
            } else {
                style.set_dimmed(true).value(elapsed_str)
            }
        };
        writeln!(buf, "  {} {} {}", level_style, elapsed_str, record.args())
    }
}

pub fn try_init_from_env() -> Result<(), SetLoggerError> {
    let now = chrono::Local::now().timestamp_millis();
    let last_time = AtomicI64::new(now);

    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info,wgpu_core=off"),
    )
    .format(move |buf, record| LogFormatter::format(buf, record, &last_time))
    .try_init()
}
