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

//! Preset configuration for `log4rs` inside of a VST plugin.
//!
//! `get_configuration_root_path()` will return the user $HOME/.ruas directory.
//!
//! `init("logger-name")` will set-up logging within this directory. Logs will rotate if they are
//! over 10MB. The directories will be created automatically.

use std::path::PathBuf;

pub mod logging;

pub fn get_configuration_root_path() -> PathBuf {
    let home_path = dirs::home_dir().unwrap_or_else(|| PathBuf::from(""));
    home_path.join(".ruas")
}

pub fn init(name: &str) {
    if let Err(err) = logging::configure_logging(&get_configuration_root_path(), name) {
        eprintln!("{}: Failed to initialize logging {:?}", name, err);
    }
}
