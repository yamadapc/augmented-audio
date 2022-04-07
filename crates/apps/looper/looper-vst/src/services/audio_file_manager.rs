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
//! This might be better extract off of here

use std::collections::HashMap;
use std::path::PathBuf;

use audio_processor_file::file_io::AudioFileError;
use audio_processor_file::InMemoryAudioFile;

type FileId = usize;

/// Handles input messages to read files, dispatches outputs messages when files have been read
pub struct AudioFileManager {
    audio_files: HashMap<FileId, InMemoryAudioFile>,
    current_id: FileId,
}

impl Default for AudioFileManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioFileManager {
    pub fn new() -> Self {
        Self {
            audio_files: Default::default(),
            current_id: 0,
        }
    }

    pub fn read_file(&mut self, file_path: PathBuf) -> Result<FileId, AudioFileError> {
        self.current_id += 1;
        let file_id = self.current_id;
        let file = InMemoryAudioFile::from_path(file_path.to_str().unwrap())?;
        self.audio_files.insert(file_id, file);
        Ok(file_id)
    }
}
